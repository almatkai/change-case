use serde_json::{json, Value};
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let tape: Vec<f64> = v["tape"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        let mut i = 0;
        let mut random = || {
            let r = tape
                .get(i)
                .copied()
                .ok_or("Random tape exhausted".to_string());
            i += 1;
            r
        };
        let s = v["input"].as_str().unwrap();
        let case = v["case"].as_str().unwrap();
        let prefs = &v["preferences"];
        let result = if v["direct"] == true {
            tezbar_change_case::convert_with(s, case, prefs, &mut random)
        } else {
            tezbar_change_case::modify_with(s, case, prefs, &mut random)
        };
        println!("{}", json!({"result":result.unwrap(),"used":i}));
    }
}
