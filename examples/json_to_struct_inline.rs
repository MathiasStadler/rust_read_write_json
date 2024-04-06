// FROM HERE
// https://reintech.io/blog/working-with-json-in-rust


use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}
fn process_person() -> Result<()> {
    let data = r#"{"name": "John Doe", "age": 30, "vip": true}"#;
    let p: Person = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", p.name, p.age);
    Ok(())
}

fn main(){

    process_person().unwrap();

}

// cargo build --example json_to_struct_inline
// cargo run --example json_to_struct_inline && echo "RC =>$?";