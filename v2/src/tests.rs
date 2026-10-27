use super::*;
use serde_json::json;
#[test]
fn actual_typescript_locked_library_oracle() {
    let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/data/parity.json")).unwrap();
    let vectors: Vec<Value> = serde_json::from_slice(&bytes).unwrap();
    for v in &vectors {
        let mut n = 0;
        let mut random = || {
            let r = v["tape"]
                .get(n)
                .and_then(Value::as_f64)
                .ok_or("Tape exhausted".into());
            n += 1;
            r
        };
        let input = v["input"].as_str().unwrap();
        let case = v["case"].as_str().unwrap();
        let prefs = &v["preferences"];
        let actual = if v["direct"] == true {
            convert_with(input, case, prefs, &mut random)
        } else {
            modify_with(input, case, prefs, &mut random)
        }
        .unwrap();
        assert_eq!(actual, v["result"].as_str().unwrap(), "{}", v);
        assert_eq!(n, v["used"].as_u64().unwrap() as usize);
    }
}
#[test]
fn resource_limits_and_os_randomness() {
    assert!(modify(&"a".repeat(MAX_INPUT_BYTES + 1), "Camel Case", &json!({})).is_err());
    assert!(convert("a", "forged", &json!({})).is_err());
    assert!(convert_with("a", "Random Case", &json!({}), &mut || Ok(f64::NAN)).is_err());
    let outputs: std::collections::HashSet<_> = (0..8)
        .map(|_| convert(&"a".repeat(128), "Random Case", &json!({})).unwrap())
        .collect();
    assert!(outputs.len() > 1);
    for c in CASES {
        assert!(modify("abc", c, &json!({})).is_ok());
    }
}
