use chinese_dictionary::query;
use serde_json::json;

fn main() {
    // let result = json!(&query("to run").unwrap());
    let result = json!(&query("keep putting something off").unwrap());
    println!("{}", result.to_string());
}
