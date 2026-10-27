//! Offline ECMAScript-compatible engine. Adapter is compiled by the host, not loaded dynamically.
mod unicode;
use serde_json::Value;

pub const MAX_INPUT_BYTES: usize = 65_536;
pub const MAX_OUTPUT_BYTES: usize = 524_288;
pub const CASES: [&str; 21] = [
    "Camel Case",
    "Capital Case",
    "Constant Case",
    "Dot Case",
    "Header Case",
    "Lower Case",
    "Lower First",
    "No Case",
    "Kebab Case",
    "Kebab Upper Case",
    "Pascal Case",
    "Pascal Snake Case",
    "Path Case",
    "Random Case",
    "Sentence Case",
    "Snake Case",
    "Alternating Case",
    "Swap Case",
    "Title Case",
    "Upper Case",
    "Upper First",
];
pub const COMMANDS: [&str; 22] = [
    "change-case",
    "convert-camel-case",
    "convert-capital-case",
    "convert-constant-case",
    "convert-dot-case",
    "convert-header-case",
    "convert-lower-case",
    "convert-lower-first",
    "convert-no-case",
    "convert-kebab-case",
    "convert-kebab-upper-case",
    "convert-pascal-case",
    "convert-pascal-snake-case",
    "convert-path-case",
    "convert-random-case",
    "convert-sentence-case",
    "convert-snake-case",
    "convert-alternating-case",
    "convert-swap-case",
    "convert-title-case",
    "convert-upper-case",
    "convert-upper-first",
];
pub fn aliases(case: &str) -> &'static [&'static str] {
    match case {
        "Header Case" => &["train", "dash"],
        "No Case" => &["none"],
        "Kebab Case" => &["dash", "slug", "param"],
        "Random Case" => &["random"],
        "Swap Case" => &["reverse"],
        "Alternating Case" => &["alternating", "sponge"],
        "Constant Case" => &["macro"],
        _ => &[],
    }
}
pub fn boolean(prefs: &Value, key: &str, default: bool) -> bool {
    prefs[key]
        .as_bool()
        .or_else(|| prefs[key].as_str().and_then(|s| s.parse().ok()))
        .unwrap_or(default)
}
pub fn text<'a>(prefs: &'a Value, key: &str, default: &'a str) -> &'a str {
    prefs[key].as_str().unwrap_or(default)
}
fn has(table: &[(u32, u32)], c: char) -> bool {
    let n = c as u32;
    let i = table.partition_point(|&(_, end)| end < n);
    table.get(i).is_some_and(|&(start, _)| start <= n)
}
fn mapped(c: char, upper: bool) -> String {
    let table = if upper {
        unicode::UPPER
    } else {
        unicode::LOWER
    };
    match table.binary_search_by_key(&(c as u32), |&(n, _)| n) {
        Ok(i) => table[i].1.into(),
        Err(_) => c.to_string(),
    }
}
pub fn upper(s: &str) -> String {
    s.chars().map(|c| mapped(c, true)).collect()
}
pub fn lower(s: &str) -> String {
    let cs: Vec<_> = s.chars().collect();
    let mut out = String::new();
    for (i, &c) in cs.iter().enumerate() {
        if c == 'Σ' {
            let before = cs[..i]
                .iter()
                .rev()
                .find(|&&c| !has(unicode::CASE_IGNORABLE, c))
                .is_some_and(|&c| has(unicode::CASED, c));
            let after = cs[i + 1..]
                .iter()
                .find(|&&c| !has(unicode::CASE_IGNORABLE, c))
                .is_some_and(|&c| has(unicode::CASED, c));
            out.push(if before && !after { 'ς' } else { 'σ' });
        } else {
            out.push_str(&mapped(c, false));
        }
    }
    out
}
fn whitespace(c: char) -> bool {
    matches!(c,'\u{0009}'..='\u{000d}'|'\u{0020}'|'\u{00a0}'|'\u{1680}'|'\u{2000}'..='\u{200a}'|'\u{2028}'|'\u{2029}'|'\u{202f}'|'\u{205f}'|'\u{3000}'|'\u{feff}')
}
fn trim(s: &str) -> &str {
    s.trim_matches(whitespace)
}
fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}
fn from_units(s: &[u16]) -> String {
    String::from_utf16_lossy(s)
}
fn unit_case(n: u16, up: bool) -> Vec<u16> {
    char::from_u32(n.into())
        .map(|c| units(&mapped(c, up)))
        .unwrap_or_else(|| vec![n])
}
fn upper_at(s: &[u16], i: usize) -> Vec<u16> {
    let mut v = s.to_vec();
    if i < v.len() {
        v.splice(i..i + 1, unit_case(v[i], true));
    }
    v
}
fn words(s: &str) -> Vec<String> {
    let mut cs: Vec<char> = trim(s).chars().collect();
    let mut i = 0;
    while i + 1 < cs.len() {
        if (has(unicode::LL, cs[i]) || cs[i].is_ascii_digit()) && has(unicode::LU, cs[i + 1]) {
            cs.insert(i + 1, '\0');
            i += 3;
        } else {
            i += 1;
        }
    }
    i = 0;
    while i + 2 < cs.len() {
        if has(unicode::LU, cs[i]) && has(unicode::LU, cs[i + 1]) && has(unicode::LL, cs[i + 2]) {
            cs.insert(i + 1, '\0');
            i += 4;
        } else {
            i += 1;
        }
    }
    let mut out = String::new();
    for c in cs {
        if has(unicode::WORD, c) {
            out.push(c);
        } else if !out.ends_with('\0') {
            out.push('\0');
        }
    }
    out.trim_matches('\0')
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}
fn library(s: &str, case: &str, prefs: &Value) -> String {
    let u = units(s);
    let prefix = units(text(prefs, "prefixCharacters", ""));
    let suffix = units(text(prefs, "suffixCharacters", ""));
    let mut a = 0;
    let mut b = u.len();
    while a < u.len() && prefix.contains(&u[a]) {
        a += 1;
    }
    while b > a && suffix.contains(&u[b - 1]) {
        b -= 1;
    }
    let ws = words(&from_units(&u[a..b]));
    let delimiter = match case {
        "Dot Case" => ".",
        "Path Case" => "/",
        "Kebab Case" | "Kebab Upper Case" | "Header Case" => "-",
        "Snake Case" | "Constant Case" | "Pascal Snake Case" => "_",
        "Camel Case" | "Pascal Case" => "",
        _ => " ",
    };
    let transformed: Vec<_> = ws
        .iter()
        .enumerate()
        .map(|(i, w)| match case {
            "Constant Case" => upper(w),
            "Capital Case" | "Header Case" | "Pascal Snake Case" | "Pascal Case" | "Camel Case"
                if case != "Camel Case" || i > 0 =>
            {
                let v = units(w);
                let first = if i > 0
                    && matches!(case, "Camel Case" | "Pascal Case")
                    && (v[0] as u8).is_ascii_digit()
                    && v[0] < 128
                {
                    format!("_{}", from_units(&v[..1]))
                } else {
                    from_units(&unit_case(v[0], true))
                };
                // A JS word[0] may be a high surrogate. Reassemble before UTF-8 conversion.
                let mut result = if v[0] >= 0xd800 && v[0] <= 0xdbff {
                    v[..1].to_vec()
                } else {
                    units(&first)
                };
                result.extend(if v.get(1).is_some_and(|n| (0xdc00..=0xdfff).contains(n)) {
                    let mut tail = vec![v[1]];
                    tail.extend(units(&lower(&from_units(&v[2..]))));
                    tail
                } else {
                    units(&lower(&from_units(&v[1..])))
                });
                from_units(&result)
            }
            _ => lower(w),
        })
        .collect();
    let result = from_units(&u[..a]) + &transformed.join(delimiter) + &from_units(&u[b..]);
    if case == "Kebab Upper Case" {
        upper(&result)
    } else {
        result
    }
}
const SMALL: &str="a an and as at because but by en for if in neither nor of on only or over per so some than that the to up upon v versus via vs when with without yet";
fn title(s: &str, sentence: bool, prefs: &Value) -> String {
    let small: Vec<_> = SMALL
        .split(' ')
        .chain(
            text(
                prefs,
                "exceptions",
                "iOS, iPadOS, iPhone, macOS, tvOS, watchOS",
            )
            .split(',')
            .map(trim),
        )
        .collect();
    let terms = if sentence { ".!?" } else { ".!?:\"'”" };
    let sep = "—–-―/";
    let u = units(s);
    let mut out = Vec::new();
    let mut a = 0;
    let mut fresh = true;
    while a < u.len() {
        if matches!(u[a], 10 | 13 | 0x2028 | 0x2029) {
            a += 1;
            continue;
        }
        let ch = char::from_u32(u[a].into()).unwrap_or('\0');
        if whitespace(ch) {
            out.push(u[a]);
            a += 1;
            continue;
        }
        let mut b = a + 1;
        while b < u.len() && !char::from_u32(u[b].into()).is_some_and(whitespace) {
            b += 1;
        }
        let token = &u[a..b];
        let t = from_units(token);
        let cs: Vec<_> = t.char_indices().collect();
        let special = cs.windows(2).any(|w| {
            matches!(w[0].1, '.' | '#') && (has(unicode::L, w[1].1) || has(unicode::N, w[1].1))
        });
        let terminates = |n: u16| char::from_u32(n.into()).is_some_and(|c| terms.contains(c));
        if special {
            // Exact acronym regexp: non-letter prefix/suffix and >=2 L-dot pairs.
            let chars: Vec<_> = t.chars().collect();
            let mut start = 0;
            while start < chars.len() && !has(unicode::L, chars[start]) {
                start += 1;
            }
            let mut end = start;
            let mut count = 0;
            while end + 1 < chars.len() && has(unicode::L, chars[end]) && chars[end + 1] == '.' {
                count += 1;
                end += 2;
            }
            let acronym = count >= 2 && chars[end..].iter().all(|&c| !has(unicode::L, c));
            if acronym {
                let pi: usize = chars[..start].iter().map(|c| c.len_utf16()).sum();
                out.extend(if sentence && !fresh {
                    token.to_vec()
                } else {
                    upper_at(token, pi)
                });
                fresh = chars.get(end).is_some_and(|c| terms.contains(*c));
            } else {
                out.extend(token);
                fresh = token.last().is_some_and(|&n| terminates(n));
            }
        } else {
            let mut matches = Vec::new();
            let mut offset = 0;
            let mut start = None;
            for c in t.chars() {
                let alpha = has(unicode::L, c) || has(unicode::N, c);
                if alpha && start.is_none() {
                    start = Some(offset);
                }
                if !alpha {
                    if let Some(st) = start.take() {
                        matches.push((st, offset));
                    }
                }
                offset += c.len_utf16();
            }
            if let Some(st) = start {
                matches.push((st, offset));
            }
            let mut v = token.to_vec();
            let mut sentence_end = false;
            for (i, &(st, end)) in matches.iter().enumerate() {
                let w = from_units(&token[st..end]);
                let next = token.get(end).copied().unwrap_or(0);
                sentence_end = terminates(next);
                let wc: Vec<_> = w.chars().collect();
                let manual = wc
                    .windows(2)
                    .any(|pair| has(unicode::LL, pair[0]) && has(unicode::LU, pair[1]));
                if fresh {
                    fresh = false;
                } else if sentence || manual {
                    continue;
                } else if matches.len() == 1 {
                    if small.contains(&w.as_str()) && b != u.len() && !sentence_end {
                        continue;
                    }
                } else if i > 0 {
                    let previous = char::from_u32(token[st - 1].into()).unwrap_or('\0');
                    if !sep.contains(previous) {
                        continue;
                    }
                    if small.contains(&w.as_str())
                        && char::from_u32(next.into()).is_some_and(|c| sep.contains(c))
                    {
                        continue;
                    }
                }
                v = upper_at(&v, st);
            }
            out.extend(v);
            fresh = sentence_end || token.last().is_some_and(|&n| terminates(n));
        }
        a = b;
    }
    from_units(&out)
}
/// Random source returns a JS Math.random-like number in [0,1).
pub fn convert_with(
    s: &str,
    case: &str,
    prefs: &Value,
    random: &mut impl FnMut() -> Result<f64, String>,
) -> Result<String, String> {
    if s.len() > MAX_INPUT_BYTES {
        return Err("Input exceeds 65536 UTF-8 bytes".into());
    }
    if !CASES.contains(&case) {
        return Err("Unknown case".into());
    }
    let pre;
    let s = if !boolean(prefs, "preserveCase", true)
        && !matches!(
            case,
            "Swap Case" | "Alternating Case" | "Random Case" | "Lower First" | "Upper First"
        ) {
        pre = lower(s);
        pre.as_str()
    } else {
        s
    };
    let result = match case {
        "Lower First" | "Upper First" => {
            let mut v = units(s);
            let index = s
                .char_indices()
                .find(|&(_, c)| has(unicode::L, c))
                .map(|(i, _)| s[..i].encode_utf16().count());
            if let Some(i) = index {
                v.splice(i..i + 1, unit_case(v[i], case == "Upper First"));
                from_units(&v)
            } else if case == "Upper First" {
                upper(s)
            } else {
                lower(s)
            }
        }
        "Swap Case" => s
            .chars()
            .map(|c| {
                let st = c.to_string();
                let lo = lower(&st);
                if st == lo {
                    upper(&st)
                } else {
                    lo
                }
            })
            .collect(),
        "Random Case" | "Alternating Case" => {
            let mut n = 0;
            let mut v = Vec::new();
            for u in units(s) {
                if char::from_u32(u.into()).is_some_and(|c| has(unicode::L, c)) {
                    n += 1;
                    let up = if case == "Random Case" {
                        let r = random()?;
                        if !(0.0..1.0).contains(&r) {
                            return Err("Invalid random sample".into());
                        }
                        r > 0.5
                    } else {
                        n % 2 == 1
                    };
                    v.extend(unit_case(u, up));
                } else {
                    v.push(u);
                }
            }
            from_units(&v)
        }
        "Title Case" | "Sentence Case" => title(s, case == "Sentence Case", prefs),
        "Capital Case" if boolean(prefs, "preservePunctuation", false) => {
            let cs: Vec<_> = s.chars().collect();
            let mut out = String::new();
            let mut i = 0;
            while i < cs.len() {
                let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
                if i == 0 && word(cs[i]) {
                    out.push_str(&mapped(cs[i], true));
                    i += 1;
                } else if (whitespace(cs[i]) || matches!(cs[i], '-' | '_'))
                    && cs.get(i + 1).is_some_and(|&c| word(c))
                {
                    out.push(cs[i]);
                    out.push_str(&mapped(cs[i + 1], true));
                    i += 2;
                } else {
                    out.push(cs[i]);
                    i += 1;
                }
            }
            out
        }
        "Lower Case" if boolean(prefs, "preservePunctuation", false) => lower(s),
        "Upper Case" if boolean(prefs, "preservePunctuation", false) => upper(s),
        _ => {
            let r = library(s, case, prefs);
            if case == "Upper Case" {
                upper(&r)
            } else if case == "Lower Case" {
                lower(&r)
            } else {
                r
            }
        }
    };
    if result.len() > MAX_OUTPUT_BYTES {
        return Err("Output exceeds 524288 UTF-8 bytes".into());
    }
    Ok(result)
}
pub fn convert(s: &str, case: &str, prefs: &Value) -> Result<String, String> {
    convert_with(s, case, prefs, &mut || {
        let mut b = [0u8; 8];
        getrandom::getrandom(&mut b).map_err(|_| "OS random source unavailable")?;
        Ok((u64::from_le_bytes(b) >> 11) as f64 / (1u64 << 53) as f64)
    })
}
pub fn modify_with(
    s: &str,
    case: &str,
    prefs: &Value,
    random: &mut impl FnMut() -> Result<f64, String>,
) -> Result<String, String> {
    if s.len() > MAX_INPUT_BYTES {
        return Err("Input exceeds 65536 UTF-8 bytes".into());
    }
    let lines: Result<Vec<_>, _> = s
        .split('\n')
        .map(|line| convert_with(line, case, prefs, random))
        .collect();
    let r = lines?.join("\n");
    if r.len() > MAX_OUTPUT_BYTES {
        return Err("Output exceeds 524288 UTF-8 bytes".into());
    }
    Ok(r)
}
pub fn modify(s: &str, case: &str, prefs: &Value) -> Result<String, String> {
    modify_with(s, case, prefs, &mut || {
        let mut b = [0u8; 8];
        getrandom::getrandom(&mut b).map_err(|_| "OS random source unavailable")?;
        Ok((u64::from_le_bytes(b) >> 11) as f64 / (1u64 << 53) as f64)
    })
}
#[cfg(test)]
mod tests;
