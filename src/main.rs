use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Serialize the person struct into a JSON string.
    let json_string = serde_json::to_string_pretty(&person).unwrap();
    println!("Serialized JSON: {}", json_string);

    // Deserialize the JSON string back into a person struct.
    let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized Person: {:?}", deserialized_person);
}