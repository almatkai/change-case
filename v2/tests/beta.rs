//! Compiles the unchanged pinned beta common.rs and SDK with headless Tauri/router stubs.
#![allow(dead_code)]
extern crate self as tauri;
#[derive(Clone)]
pub struct AppHandle;
pub trait Emitter {
    fn emit(&self, channel: &str, payload: serde_json::Value) -> Result<(), String>;
}
impl Emitter for AppHandle {
    fn emit(&self, _: &str, _: serde_json::Value) -> Result<(), String> {
        panic!("Tauri effects forbidden in fixture")
    }
}
#[path = "../adapter.rs"]
mod adapter;
#[path = "../fixtures/beta/src-tauri/src/native_core/common.rs"]
pub mod beta_common;
#[path = "../fixtures/beta/src-tauri/src/native_core/extensions/v2_sdk.rs"]
pub mod v2_sdk;
pub mod native_core {
    pub use crate::beta_common as common;
}
use beta_common::{CoreContext, CorePaths};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
type Calls = Arc<Mutex<Vec<(String, Vec<Value>)>>>;
struct Fixture {
    _temp: tempfile::TempDir,
    ctx: CoreContext,
    calls: Calls,
    response: Arc<Mutex<Result<Value, String>>>,
    hide_response: Arc<Mutex<Result<Value, String>>>,
    selection: Arc<Mutex<Result<Value, String>>>,
    clipboard: Arc<Mutex<Result<Value, String>>>,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let ctx = CoreContext::headless(CorePaths {
            data_dir: temp.path().join("data"),
            config_dir: temp.path().join("config"),
            resource_dir: temp.path().join("resources"),
            temp_dir: temp.path().join("tmp"),
            home_dir: temp.path().into(),
        });
        let calls: Calls = Arc::default();
        let response = Arc::new(Mutex::new(Ok(json!({"ok":true}))));
        let hide_response = Arc::new(Mutex::new(Ok(json!({"ok":true}))));
        let h = hide_response.clone();
        let selection = Arc::new(Mutex::new(Ok(json!("selected TEXT"))));
        let clipboard = Arc::new(Mutex::new(Ok(json!("clipboard TEXT"))));
        let (r, c, s, b) = (
            response.clone(),
            calls.clone(),
            selection.clone(),
            clipboard.clone(),
        );
        ctx.set_router(Arc::new(move |channel, args| {
            c.lock().unwrap().push((channel.clone(), args));
            let result = match channel.as_str() {
                "core:platform:selected-text" => s.lock().unwrap().clone(),
                "clipboard:read" => b.lock().unwrap().clone(),
                "window:hide" => h.lock().unwrap().clone(),
                "clipboard:write" | "core:platform:paste-text" => r.lock().unwrap().clone(),
                _ => panic!("Unexpected OS route {channel}"),
            };
            Box::pin(async move { result })
        }))
        .unwrap();
        Self {
            _temp: temp,
            ctx,
            calls,
            response,
            hide_response,
            selection,
            clipboard,
        }
    }
    async fn call(&self, op: &str, payload: Value, state: &mut Value) -> Result<Value, String> {
        adapter::handle(&self.ctx, "change-case", op, &payload, state).await
    }
    async fn launch(&self, prefs: Value) -> (Value, Value) {
        let mut s = json!({});
        let v = self
            .call(
                "extension:run-command",
                json!({"preferences":prefs}),
                &mut s,
            )
            .await
            .unwrap();
        (s, v)
    }
    async fn action(&self, id: &Value, s: &mut Value) -> Result<Value, String> {
        self.call(
            "extension:invoke-action",
            json!({"actionId":id,"text":"FORGED","url":"https://forged.test"}),
            s,
        )
        .await
    }
    fn writes(&self) -> Vec<(String, Vec<Value>)> {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|(c, _)| matches!(c.as_str(), "clipboard:write" | "core:platform:paste-text"))
            .cloned()
            .collect()
    }
}
fn action(v: &Value, title: &str) -> Value {
    v["actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["title"] == title)
        .unwrap_or_else(|| panic!("missing {title}: {v}"))
        .get("id")
        .unwrap()
        .clone()
}
fn rows(v: &Value) -> Vec<Value> {
    v["root"]["children"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|s| s["children"].as_array().unwrap().clone())
        .collect()
}
#[tokio::test]
async fn all_commands_source_priority_preferences_and_launch_context() {
    let f = Fixture::new();
    assert_eq!(adapter::commands().len(), 22);
    for (i, command) in adapter::commands().iter().enumerate().skip(1) {
        let mut s = json!({});
        let v = adapter::handle(
            &f.ctx,
            command,
            "extension:run-command",
            &json!({"preferences":{"action":"copy","preserveCase":false,"hideHUD":true}}),
            &mut s,
        )
        .await
        .unwrap();
        assert_eq!(v["mode"], "no-view");
        assert_eq!(v["message"], "");
        let output = f.writes().last().unwrap().1[0].as_str().unwrap().to_owned();
        if *command != "convert-random-case" {
            assert_eq!(
                output,
                tezbar_change_case::modify(
                    "selected TEXT",
                    tezbar_change_case::CASES[i - 1],
                    &json!({"preserveCase":false})
                )
                .unwrap()
            );
        }
    }
    let (mut s, _) = f
        .launch(json!({"source":"clipboard","action":"copy"}))
        .await;
    assert_eq!(s["content"], "clipboard TEXT");
    *f.selection.lock().unwrap() = Err("unavailable".into());
    let (s2, _) = f.launch(json!({})).await;
    assert_eq!(s2["content"], "clipboard TEXT");
    *f.clipboard.lock().unwrap() = Ok(json!(""));
    assert!(f
        .call("extension:run-command", json!({}), &mut json!({}))
        .await
        .is_err());
    let v=f.call("extension:run-command",json!({"fallbackText":"one\nTWO","launchContext":{"case":"Title Case"},"preferences":{"action":"copy"}}),&mut json!({})).await;
    assert!(v.is_err());
    *f.clipboard.lock().unwrap() = Ok(json!("one\nTWO"));
    let v = f
        .call(
            "extension:run-command",
            json!({"launchContext":{"case":"Title Case"},"preferences":{"action":"copy"}}),
            &mut json!({}),
        )
        .await
        .unwrap();
    assert_eq!(v["mode"], "no-view");
    assert_eq!(
        f.writes().last().unwrap().1,
        json!(["OneTWO"]).as_array().unwrap().clone()
    );
    f.call("extension:dispose-session", json!({}), &mut s)
        .await
        .unwrap();
    assert!(f
        .call(
            "extension:invoke-action",
            json!({"actionId":"whatever"}),
            &mut s
        )
        .await
        .is_err());
}
#[tokio::test]
async fn bindings_search_staged_discard_refresh_and_no_arbitrary_outputs() {
    let f = Fixture::new();
    let (mut s, v) = f.launch(json!({"action":"copy"})).await;
    assert_eq!(rows(&v).len(), 21);
    let copy = action(&v, "Copy to Clipboard");
    let original = s.clone();
    let mut staged = s.clone();
    let discarded = f
        .call(
            "extension:search-text-changed",
            json!({"searchText":"sponge"}),
            &mut staged,
        )
        .await
        .unwrap();
    assert_eq!(rows(&discarded).len(), 1);
    let committed = f
        .call(
            "extension:search-text-changed",
            json!({"searchText":"macro"}),
            &mut s,
        )
        .await
        .unwrap();
    assert_eq!(rows(&committed).len(), 1);
    assert_eq!(rows(&committed)[0]["props"]["title"], "Constant Case");
    for id in [
        copy.clone(),
        action(&discarded, "Copy to Clipboard"),
        json!("change-case:fake"),
    ] {
        let before = s.clone();
        assert!(f.action(&id, &mut s).await.is_err());
        assert_eq!(before, s);
    }
    let (mut other, _) = f.launch(json!({})).await;
    assert!(f
        .action(&action(&committed, "Copy to Clipboard"), &mut other)
        .await
        .is_err());
    let before = s.clone();
    let refreshed = f
        .call("extension:refresh-session", json!({}), &mut s.clone())
        .await
        .unwrap();
    assert_eq!(refreshed["mode"], "unchanged");
    assert_eq!(before, s);
    let result = f
        .action(&action(&committed, "Copy to Clipboard"), &mut s)
        .await
        .unwrap();
    assert_eq!(result["mode"], "no-view");
    assert_eq!(
        f.writes(),
        vec![("clipboard:write".into(), vec![json!("SELECTED_TEXT")])]
    );
    assert!(f.action(&copy, &mut s).await.is_err());
    assert_ne!(s, original);
}
#[tokio::test]
async fn pin_recent_disabled_cases_clear_remove_refresh_and_quicklink_gap() {
    let f = Fixture::new();
    v2_sdk::update_store(&f.ctx, "change-case", |s| {
        s["unknown"] = json!({"keep":true});
        Ok(())
    })
    .await
    .unwrap();
    let (mut s, v) = f.launch(json!({"CamelCase":false,"action":"copy"})).await;
    assert_eq!(rows(&v).len(), 20);
    let pin = action(&v, "Pin Case");
    let v = f.action(&pin, &mut s).await.unwrap();
    assert_eq!(rows(&v)[0]["props"]["title"], "Capital Case");
    assert_eq!(v["root"]["children"][0]["props"]["title"], "Pinned");
    let v = f
        .action(&action(&v, "Remove Pinned Case"), &mut s)
        .await
        .unwrap();
    let quick = v["actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["title"].as_str().unwrap().starts_with("Create Quicklink"))
        .unwrap()["id"]
        .clone();
    assert!(f
        .action(&quick, &mut s)
        .await
        .unwrap_err()
        .contains("unavailable"));
    f.action(&action(&v, "Copy to Clipboard"), &mut s)
        .await
        .unwrap();
    let (mut s, v) = f.launch(json!({"CapitalCase":false,"action":"copy"})).await;
    assert_eq!(
        v["root"]["children"][1]["children"][0]["props"]["title"],
        "Capital Case"
    );
    let v = f
        .action(&action(&v, "Remove Recent Case"), &mut s)
        .await
        .unwrap();
    assert!(v["root"]["children"][1]["children"]
        .as_array()
        .unwrap()
        .is_empty());
    let v = f.action(&action(&v, "Pin Case"), &mut s).await.unwrap();
    let v = f
        .action(&action(&v, "Clear Pinned Cases"), &mut s)
        .await
        .unwrap();
    f.action(&action(&v, "Refresh Content"), &mut s)
        .await
        .unwrap();
    let store = v2_sdk::read_store(&f.ctx, "change-case").await.unwrap();
    assert_eq!(store["unknown"], json!({"keep":true}));
    assert_eq!(store["pinned"], json!([]));
    let (mut s, v) = f.launch(json!({"action":"copy"})).await;
    f.action(&action(&v, "Copy to Clipboard"), &mut s)
        .await
        .unwrap();
    let (mut s, v) = f.launch(json!({})).await;
    f.action(&action(&v, "Clear Recent Cases"), &mut s)
        .await
        .unwrap();
    assert_eq!(s["store"]["recent"], json!([]));
}
#[tokio::test]
async fn strict_effect_results_require_deliberate_retry() {
    for response in [
        Ok(Value::Null),
        Ok(json!(true)),
        Ok(json!({})),
        Ok(json!({"ok":false})),
        Ok(json!({"ok":true,"error":"oops"})),
        Ok(json!({"ok":true,"success":false})),
        Ok(json!({"ok":true,"success":null})),
        Err("sensitive transport payload".into()),
    ] {
        let f = Fixture::new();
        let (mut s, v) = f.launch(json!({})).await;
        *f.response.lock().unwrap() = response;
        let id = action(&v, "Paste in Active App");
        let v = f.action(&id, &mut s).await.unwrap();
        assert_eq!(v["root"]["type"], "Detail");
        assert!(!v.to_string().contains("sensitive"));
        assert_eq!(f.writes().len(), 1);
        assert!(f.action(&id, &mut s).await.is_err());
        assert_eq!(f.writes().len(), 1);
        *f.response.lock().unwrap() = Ok(json!({"ok":true,"success":true,"error":null}));
        f.action(
            &action(&v, "Retry Action (May Repeat Previous Paste)"),
            &mut s,
        )
        .await
        .unwrap();
        assert_eq!(f.writes().len(), 2);
    }
}
#[tokio::test]
async fn acknowledged_paste_not_repeated_after_storage_failure_and_cancel() {
    let f = Fixture::new();
    let (mut s, v) = f.launch(json!({})).await;
    let storage = f
        .ctx
        .paths
        .data_dir
        .join("native-extensions-v2/change-case/storage.json");
    std::fs::create_dir_all(storage.parent().unwrap()).unwrap();
    std::fs::write(&storage, "corrupt").unwrap();
    let v = f
        .action(&action(&v, "Paste in Active App"), &mut s)
        .await
        .unwrap();
    assert_eq!(s["pending"]["phase"], "acknowledged");
    assert_eq!(f.writes().len(), 1);
    std::fs::write(&storage, "{}").unwrap();
    let result = f
        .action(
            &action(&v, "Finish Action Without Repeating Conversion"),
            &mut s,
        )
        .await
        .unwrap();
    assert_eq!(result["mode"], "no-view");
    assert_eq!(f.writes().len(), 1);
    let (mut s, v) = f.launch(json!({})).await;
    *f.response.lock().unwrap() = Ok(Value::Null);
    let v = f
        .action(&action(&v, "Paste in Active App"), &mut s)
        .await
        .unwrap();
    let v = f
        .action(&action(&v, "Cancel Pending Action"), &mut s)
        .await
        .unwrap();
    assert_eq!(v["root"]["type"], "List");
}
#[tokio::test]
async fn clipboard_failure_and_corrupt_store_never_succeed() {
    let f = Fixture::new();
    *f.clipboard.lock().unwrap() = Err("private clipboard".into());
    assert_eq!(
        f.call("extension:run-command", json!({}), &mut json!({}))
            .await
            .unwrap_err(),
        "Cannot read clipboard"
    );
    let path = f
        .ctx
        .paths
        .data_dir
        .join("native-extensions-v2/change-case/storage.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    for bytes in [
        "bad",
        "[]",
        "{\"pinned\":[\"forged\"]}",
        "{\"recent\":[\"Camel Case\",\"Camel Case\"]}",
    ] {
        std::fs::write(&path, bytes).unwrap();
        assert!(f
            .call(
                "extension:run-command",
                json!({"fallbackText":"text"}),
                &mut json!({})
            )
            .await
            .is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), bytes);
    }
}
#[tokio::test]
async fn acknowledged_copy_not_repeated_after_hide_failure_and_all_enabled_preferences() {
    let f = Fixture::new();
    for case in tezbar_change_case::CASES {
        let key = case.replace(' ', "");
        let (_, v) = f.launch(json!({key:false})).await;
        assert_eq!(rows(&v).len(), 20, "{case}");
    }
    let (mut s, v) = f.launch(json!({"action":"copy"})).await;
    assert_eq!(v["actions"][0]["title"], "Copy to Clipboard");
    *f.hide_response.lock().unwrap() = Ok(json!({"ok":true,"success":false}));
    let v = f
        .action(&action(&v, "Copy to Clipboard"), &mut s)
        .await
        .unwrap();
    assert_eq!(s["pending"]["phase"], "stored");
    assert_eq!(f.writes().len(), 1);
    *f.hide_response.lock().unwrap() = Ok(json!({"ok":true}));
    f.action(
        &action(&v, "Finish Action Without Repeating Conversion"),
        &mut s,
    )
    .await
    .unwrap();
    assert_eq!(f.writes().len(), 1);
}
#[tokio::test]
async fn copy_transport_error_recovers_inline_and_fallback_refresh_stays_bound() {
    let f = Fixture::new();
    let (mut s, v) = f.launch(json!({"action":"copy"})).await;
    *f.response.lock().unwrap() = Err("private text".into());
    let v = f
        .action(&action(&v, "Copy to Clipboard"), &mut s)
        .await
        .unwrap();
    assert_eq!(v["root"]["type"], "Detail");
    assert!(!v.to_string().contains("private"));
    let mut s = json!({});
    let v = f
        .call(
            "extension:run-command",
            json!({"fallbackText":"immutable fallback"}),
            &mut s,
        )
        .await
        .unwrap();
    *f.selection.lock().unwrap() = Ok(json!("new selection"));
    let _ = f
        .action(&action(&v, "Refresh Content"), &mut s)
        .await
        .unwrap();
    assert_eq!(s["content"], "immutable fallback");
}
