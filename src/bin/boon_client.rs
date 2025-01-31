use boon::{Compiler, Schemas, ValidationError};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
// use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

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
    // Generete schéma JSON of structure
    let schema = schema_for!(MyStruct);
    let schema_json = serde_json::to_value(&schema)?;
    let schema_str = serde_json::to_string(&schema_json)?;

    // Display schéma JSON génereted
    println!("Generated Schema: {}", schema_str);

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(schema_str.as_bytes())?;

    // Instance JSON to validate
    let instance = json!({
        "myNumber": 42,
        "myBool": true,
        "myNullableEnum": "Unit"
    });

    // Compile the schéma with boon, et provide second argument (Schemas)
    let mut schemas = Schemas::new();
    let mut compiler = Compiler::new();

    let schema_index = compiler.compile(temp_file.path().to_str().unwrap(), &mut schemas)?;

    // Valide instance JSON using compiled schemas
    let validation_result: Result<(), ValidationError> = schemas.validate(&instance, schema_index);

    // Vérify if the validation is successful
    match validation_result {
        Ok(()) => println!("Validation réussie"),
        Err(e) => {
            println!("Validation échouée");
            eprintln!("Erreur: {}", e);
        }
    }

    Ok(())
}
