//! Concrete compiled beta adapter. Vendor this directory at src-tauri/native-extensions/change-case.
#[allow(dead_code)]
#[path = "src/lib.rs"]
mod engine;
use super::v2_sdk;
use crate::native_core::common::{CoreContext, CoreResult};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(1);
const SLUG: &str = "change-case";
pub fn commands() -> &'static [&'static str] {
    &engine::COMMANDS
}
fn fresh() -> Result<String, String> {
    let n = NEXT
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .map_err(|_| "Action nonce exhausted")?;
    // A process-wide monotonic counter prevents discarded staged searches from reviving IDs.
    Ok(format!("change-case:{n}"))
}
fn success(v: Value) -> Result<(), String> {
    if !v.is_object()
        || v["ok"] != true
        || v.get("error").is_some_and(|v| !v.is_null())
        || v.get("success").is_some_and(|v| v != true)
    {
        Err("Operation did not acknowledge success; its outcome may be uncertain".into())
    } else {
        Ok(())
    }
}
async fn effect(ctx: &CoreContext, route: &str, args: Vec<Value>) -> Result<(), String> {
    success(
        ctx.request(route, args)
            .await
            .map_err(|_| "Operation transport failed; its outcome may be uncertain")?,
    )
}
fn array(v: &Value, key: &str) -> Result<Vec<String>, String> {
    if v.get(key).is_none() {
        return Ok(vec![]);
    }
    let a = v[key]
        .as_array()
        .ok_or("Change Case history must be an array")?;
    if a.len() > 21 {
        return Err("Change Case history exceeds 21 entries".into());
    }
    let mut out = Vec::new();
    for item in a {
        let s = item
            .as_str()
            .filter(|s| engine::CASES.contains(s))
            .ok_or("Invalid Change Case history entry")?;
        if out.iter().any(|v| v == s) {
            return Err("Duplicate Change Case history entry".into());
        }
        out.push(s.to_owned());
    }
    Ok(out)
}
fn validate_store(store: &Value) -> Result<(), String> {
    array(store, "pinned")?;
    array(store, "recent")?;
    Ok(())
}
async fn read_store(ctx: &CoreContext) -> CoreResult {
    let store = v2_sdk::read_store(ctx, SLUG)
        .await
        .map_err(|_| "Cannot read Change Case storage")?;
    validate_store(&store)?;
    Ok(store)
}
async fn save(ctx: &CoreContext, verb: String, case: String) -> CoreResult {
    v2_sdk::update_store(ctx, SLUG, move |store| {
        validate_store(store)?;
        let mut pinned = array(store, "pinned")?;
        let mut recent = array(store, "recent")?;
        match verb.as_str() {
            "pin" => {
                pinned.retain(|c| c != &case);
                pinned.insert(0, case.clone());
                recent.retain(|c| c != &case);
            }
            "unpin" => pinned.retain(|c| c != &case),
            "clear-pinned" => pinned.clear(),
            "remove-recent" => recent.retain(|c| c != &case),
            "clear-recent" => recent.clear(),
            "recent" => {
                recent.retain(|c| c != &case);
                recent.insert(0, case);
                recent.truncate(4 + pinned.len());
            }
            _ => return Err("Invalid storage operation".into()),
        }
        store["pinned"] = json!(pinned);
        store["recent"] = json!(recent);
        Ok(())
    })
    .await
    .map_err(|_| "Cannot update Change Case storage".into())
}
async fn content(ctx: &CoreContext, prefs: &Value) -> Result<String, String> {
    // Clipboard failures reject in the original Promise.all; selection failures fall back.
    let (clipboard_result, selection_result) = tokio::join!(
        ctx.request("clipboard:read", vec![]),
        ctx.request("core:platform:selected-text", vec![])
    );
    let clipboard = clipboard_result.map_err(|_| "Cannot read clipboard")?;
    let clipboard = clipboard
        .as_str()
        .ok_or("Clipboard returned malformed text")?;
    let selection = match selection_result {
        Ok(v) => v
            .as_str()
            .ok_or("Selection returned malformed text")?
            .to_owned(),
        Err(_) => String::new(),
    };
    let (primary, secondary) = if engine::text(prefs, "source", "selected") == "clipboard" {
        (clipboard, selection.as_str())
    } else {
        (selection.as_str(), clipboard)
    };
    let s = if primary.is_empty() {
        secondary
    } else {
        primary
    };
    if s.is_empty() {
        return Err("No text available — select text or copy to clipboard first".into());
    }
    if s.len() > engine::MAX_INPUT_BYTES {
        return Err("Input exceeds 65536 UTF-8 bytes".into());
    }
    Ok(s.into())
}
fn bind(
    state: &mut Value,
    actions: &mut Vec<Value>,
    verb: &str,
    title: &str,
    case: &str,
    output: &str,
) -> Result<String, String> {
    let id = fresh()?;
    state["bindings"][&id] = json!({"verb":verb,"case":case,"output":output});
    actions.push(json!({"id":id,"title":title,"kind":"action"}));
    Ok(id)
}
fn build(state: &mut Value) -> CoreResult {
    let mut staged = state.clone();
    let view = build_staged(&mut staged)?;
    *state = staged;
    Ok(view)
}
fn build_staged(state: &mut Value) -> CoreResult {
    state["bindings"] = json!({});
    let mut actions = Vec::new();
    if state["pending"].is_object() {
        let uncertain = state["pending"]["phase"] == "uncertain";
        bind(
            state,
            &mut actions,
            if uncertain { "retry" } else { "resume" },
            if uncertain {
                "Retry Action (May Repeat Previous Paste)"
            } else {
                "Finish Action Without Repeating Conversion"
            },
            "",
            "",
        )?;
        bind(
            state,
            &mut actions,
            "cancel",
            "Cancel Pending Action",
            "",
            "",
        )?;
        return Ok(v2_sdk::view(
            "Change Case",
            json!({"type":"Detail","props":{"markdown":state["notice"].as_str().unwrap_or("Action interrupted. Confirm retry only after checking the target app. An acknowledged paste will not repeat.")}}),
            actions,
        ));
    }
    let prefs = state["preferences"].clone();
    let input = state["content"].as_str().unwrap_or("").to_owned();
    let pinned = array(&state["store"], "pinned")?;
    let recent = array(&state["store"], "recent")?;
    let query = engine::lower(state["searchText"].as_str().unwrap_or(""));
    let mut sections = Vec::new();
    for (section, cases) in [
        ("Pinned", pinned.clone()),
        (
            "Recent",
            recent
                .iter()
                .filter(|c| !pinned.contains(c))
                .cloned()
                .collect(),
        ),
        (
            "All Cases",
            engine::CASES
                .iter()
                .filter(|&&c| {
                    !pinned.iter().any(|s| s == c)
                        && !recent.iter().any(|s| s == c)
                        && engine::boolean(&prefs, &c.replace(' ', ""), true)
                })
                .map(|s| (*s).to_owned())
                .collect(),
        ),
    ] {
        let mut rows = Vec::new();
        for case in cases {
            // Aliases match the source; output text is also searchable as Raycast accessories.
            let output = if let Some(s) = state["outputs"][&case].as_str() {
                s.to_owned()
            } else {
                let s = engine::modify(&input, &case, &prefs)?;
                state["outputs"][&case] = json!(s);
                s
            };
            if !query.is_empty()
                && !engine::lower(&case).contains(&query)
                && !engine::lower(&output).contains(&query)
                && !engine::aliases(&case).iter().any(|s| s.contains(&query))
            {
                continue;
            }
            let mut ids = Vec::new();
            let primary = engine::text(&prefs, "action", "paste");
            for verb in if primary == "copy" {
                ["copy", "paste"]
            } else {
                ["paste", "copy"]
            } {
                ids.push(bind(
                    state,
                    &mut actions,
                    verb,
                    if verb == "copy" {
                        "Copy to Clipboard"
                    } else {
                        "Paste in Active App"
                    },
                    &case,
                    &output,
                )?);
            }
            if section == "Pinned" {
                ids.push(bind(
                    state,
                    &mut actions,
                    "unpin",
                    "Remove Pinned Case",
                    &case,
                    "",
                )?);
                ids.push(bind(
                    state,
                    &mut actions,
                    "clear-pinned",
                    "Clear Pinned Cases",
                    &case,
                    "",
                )?);
            } else {
                ids.push(bind(state, &mut actions, "pin", "Pin Case", &case, "")?);
            }
            if section == "Recent" {
                ids.push(bind(
                    state,
                    &mut actions,
                    "remove-recent",
                    "Remove Recent Case",
                    &case,
                    "",
                )?);
                ids.push(bind(
                    state,
                    &mut actions,
                    "clear-recent",
                    "Clear Recent Cases",
                    &case,
                    "",
                )?);
            }
            ids.push(bind(
                state,
                &mut actions,
                "quicklink",
                &format!("Create Quicklink to Convert to {case}"),
                &case,
                "",
            )?);
            ids.push(bind(
                state,
                &mut actions,
                "refresh",
                "Refresh Content",
                &case,
                "",
            )?);
            let markdown = output
                .split('\n')
                .map(|s| format!("{}\n", if s.is_empty() { "\u{200b}" } else { s }))
                .collect::<Vec<_>>()
                .join("\n");
            rows.push(json!({"type":"List.Item","props":{"id":case,"title":case,"accessories":[{"text":output}],"detail":{"type":"List.Item.Detail","props":{"markdown":markdown}},"actionIds":ids}}));
        }
        sections.push(json!({"type":"List.Section","props":{"title":section},"children":rows}));
    }
    Ok(v2_sdk::view(
        "Change Case",
        json!({"type":"List","props":{"isShowingDetail":true,"filtering":false,"__hasServerSearch":true,"searchText":state["searchText"],"selectedItemId":recent.first(),"searchBarPlaceholder":"Search cases and aliases"},"children":sections}),
        actions,
    ))
}
async fn finish(ctx: &CoreContext, state: &mut Value) -> CoreResult {
    let pending = state["pending"].clone();
    let phase = pending["phase"].as_str().unwrap_or("");
    if phase == "uncertain" {
        return build(state);
    }
    if phase == "ready" {
        state["pending"]["phase"] = json!("uncertain"); // Written before await: cancellation cannot claim acknowledgement.
        let output = pending["output"]
            .as_str()
            .ok_or("Missing bound conversion")?;
        let result = if pending["verb"] == "paste" {
            effect(ctx, "core:platform:paste-text", vec![json!(output)]).await
        } else {
            v2_sdk::copy(ctx, output)
                .await
                .map_err(|_| "Copy transport failed; its outcome may be uncertain".to_owned())
                .and_then(success)
        };
        if let Err(message) = result {
            state["notice"] = json!(message);
            return build(state);
        }
        state["pending"]["phase"] = json!("acknowledged");
    }
    if state["pending"]["phase"] == "acknowledged" {
        if pending["record"] == true {
            match save(
                ctx,
                "recent".into(),
                pending["case"].as_str().unwrap_or("").into(),
            )
            .await
            {
                Ok(store) => state["store"] = store,
                Err(message) => {
                    state["notice"] = json!(format!(
                        "Conversion acknowledged. {message}. Finish will not repeat it."
                    ));
                    return build(state);
                }
            }
        }
        state["pending"]["phase"] = json!("stored");
    }
    let prefs = state["preferences"].clone();
    // paste-text already hides/restores focus/pastes. Copy closes the interactive view.
    if pending["verb"] == "copy"
        && (pending["record"] == true
            || engine::boolean(&prefs, "popToRoot", false)
            || pending["immediate"] == true)
    {
        if let Err(message) = effect(ctx, "window:hide", vec![]).await {
            state["notice"] = json!(format!(
                "Conversion acknowledged. {message}. Finish will not repeat it."
            ));
            return build(state);
        }
    }
    state["bindings"] = json!({});
    state["completed"] = json!(true);
    state["pending"] = Value::Null;
    Ok(v2_sdk::no_view(
        &if engine::boolean(&prefs, "hideHUD", false) {
            String::new()
        } else {
            format!("Converted to {}", pending["case"].as_str().unwrap_or(""))
        },
    ))
}
pub async fn handle(
    ctx: &CoreContext,
    command: &str,
    operation: &str,
    payload: &Value,
    state: &mut Value,
) -> CoreResult {
    if !commands().contains(&command) {
        return Err("Unknown Change Case command".into());
    }
    if operation == "extension:dispose-session" {
        *state = json!({"disposed":true});
        return Ok(v2_sdk::unchanged());
    }
    if matches!(
        operation,
        "extension:refresh-session" | "extension:load-more"
    ) {
        return Ok(v2_sdk::unchanged());
    }
    if operation == "extension:run-command" {
        if state["pending"].is_object() {
            return finish(ctx, state).await;
        }
        if state["completed"] == true || state["disposed"] == true {
            return Err("Session has ended; launch a new command".into());
        }
        let prefs = payload["preferences"].clone();
        let fallback = payload["fallbackText"].as_str().filter(|s| !s.is_empty());
        let immediate = payload["launchContext"]
            .get("case")
            .and_then(Value::as_str)
            .filter(|_| command == "change-case");
        let input = if command == "change-case" && immediate.is_none() {
            if let Some(s) = fallback {
                if s.len() > engine::MAX_INPUT_BYTES {
                    return Err("Input exceeds 65536 UTF-8 bytes".into());
                }
                s.into()
            } else {
                content(ctx, &prefs).await?
            }
        } else {
            content(ctx, &prefs).await?
        };
        *state = json!({"preferences":prefs,"content":input,"searchText":"","bindings":{},"outputs":{},"fallbackText":fallback});
        if command == "change-case" && immediate.is_none() {
            state["store"] = read_store(ctx).await?;
            return build(state);
        }
        let case = if let Some(case) = immediate.filter(|_| command == "change-case") {
            if !engine::CASES.contains(&case) {
                return Err("Unknown quicklink case".into());
            }
            case
        } else {
            engine::CASES[commands()
                .iter()
                .position(|&c| c == command)
                .ok_or("Unknown command")?
                - 1]
        };
        let output = if immediate.is_some() {
            engine::convert(&input, case, &state["preferences"])?
        } else {
            engine::modify(&input, case, &state["preferences"])?
        };
        state["pending"] = json!({"case":case,"output":output,"verb":engine::text(&state["preferences"],"action","paste"),"record":false,"immediate":immediate.is_some(),"phase":"ready"});
        return finish(ctx, state).await;
    }
    if state["disposed"] == true || state["completed"] == true {
        return Err("Session has ended".into());
    }
    if operation == "extension:search-text-changed" {
        if state["pending"].is_object() {
            return build(state);
        }
        let query = payload["searchText"]
            .as_str()
            .ok_or("Missing search text")?;
        if query.len() > 4096 {
            return Err("Search exceeds 4096 UTF-8 bytes".into());
        }
        state["searchText"] = json!(query);
        return build(state);
    }
    if operation != "extension:invoke-action" {
        return Err("Unsupported Change Case operation".into());
    }
    let id = payload["actionId"].as_str().ok_or("Missing action ID")?;
    let binding = state["bindings"]
        .get(id)
        .filter(|v| v.is_object())
        .cloned()
        .ok_or("Action is stale or not bound to this session")?;
    let verb = binding["verb"].as_str().ok_or("Invalid action")?;
    let case = binding["case"].as_str().unwrap_or("");
    if state["pending"].is_object() && !matches!(verb, "retry" | "resume" | "cancel") {
        return finish(ctx, state).await;
    }
    match verb{
        "copy"|"paste"=>{state["pending"]=json!({"verb":verb,"case":case,"output":binding["output"],"record":true,"phase":"ready"});finish(ctx,state).await},
        "retry"=>{if state["pending"]["phase"]!="uncertain"{return Err("No uncertain action to retry".into());}state["pending"]["phase"]=json!("ready");finish(ctx,state).await},
        "resume"=>finish(ctx,state).await,
        "cancel"=>{state["pending"]=Value::Null;build(state)},
        "pin"|"unpin"|"clear-pinned"|"remove-recent"|"clear-recent"=>{state["store"]=save(ctx,verb.into(),case.into()).await?;state["outputs"]=json!({});build(state)},
        "quicklink"=>Err("Quicklink creation is unavailable in the pinned Native V2 beta API; no quicklink was created".into()),
        "refresh"=>{state["content"]=if let Some(s)=state["fallbackText"].as_str(){json!(s)}else{json!(content(ctx,&state["preferences"]).await?)};state["outputs"]=json!({});build(state)},
        _=>Err("Unsupported bound action".into())
    }
}
