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
fn create_person() -> Result<String> {
    let p = Person {
        name: "John Doe".to_string(),
        age: 43,
        vip: true,
    };
    let j = serde_json::to_string(&p)?;
    println!("{}",j);
    Ok(j)
}


fn main(){

create_person().unwrap();

}

// cargo build --example struct_to_json_inline
// cargo run --example struct_to_json_inline && echo "RC =>$?";