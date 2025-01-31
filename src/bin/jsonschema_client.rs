use jsonschema::{self, validator_for};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyStruct {
    #[serde(rename = "myNumber")]
    pub my_int: i32,
    pub my_bool: bool,
    #[serde(default)]
    pub my_nullable_enum: Option<MyEnum>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum MyEnum {
    Unit,
    StringNewType(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(MyStruct);
    let schema_json = serde_json::to_value(&schema)?;
    println!("{:?}", serde_json::to_string_pretty(&schema_json));

    let instance = json!({
        "myNumber": 42,
        "myBool": true,
        "myNullableEnum": "Unit"
    });

    let validator = validator_for(&schema_json)?;

    assert!(validator.validate(&instance).is_ok());

    for error in validator.iter_errors(&instance) {
        eprintln!("Error: {}", error);
        eprintln!("Location: {}", error.instance_path);
    }

    assert!(validator.is_valid(&instance));

    Ok(())
}
