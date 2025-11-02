use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CellTemplate {
    pub id: String,
    pub name: String,
    pub version: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug)]
pub struct ValidationErrors(Vec<String>);

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation errors: {:?}", self.0)
    }
}

impl Error for ValidationErrors {}

pub fn validate_template(value: &Value) -> Result<(), Box<dyn Error>> {
    let mut errors = Vec::new();

    let obj = value.as_object().ok_or_else(|| {
        Box::new(ValidationErrors(vec![
            "Template must be a JSON object".into()
        ])) as Box<dyn Error>
    })?;

    let expect_string = |field: &str, errors: &mut Vec<String>| match obj.get(field) {
        Some(Value::String(s)) if !s.trim().is_empty() => Some(s.clone()),
        _ => {
            errors.push(format!("Field '{field}' must be a non-empty string"));
            None
        }
    };

    expect_string("id", &mut errors);
    expect_string("name", &mut errors);
    expect_string("version", &mut errors);

    let expect_array = |field: &str, errors: &mut Vec<String>| match obj.get(field) {
        Some(Value::Array(arr)) if !arr.is_empty() => {
            if arr
                .iter()
                .all(|val| matches!(val, Value::String(s) if !s.is_empty()))
            {
                Some(arr)
            } else {
                errors.push(format!(
                    "Field '{field}' must contain only non-empty strings"
                ));
                None
            }
        }
        _ => {
            errors.push(format!(
                "Field '{field}' must be a non-empty array of strings"
            ));
            None
        }
    };

    expect_array("inputs", &mut errors);
    expect_array("outputs", &mut errors);

    if let Some(value) = obj.get("parameters") {
        if !value.is_object() {
            errors.push("Field 'parameters' must be an object when present".into());
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Box::new(ValidationErrors(errors)))
    }
}
