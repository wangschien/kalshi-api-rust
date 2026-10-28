use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

fn roundtrip<T: DeserializeOwned + Serialize>(value: Value) -> Result<Value, String> {
    serde_json::from_value::<T>(value)
        .and_then(serde_json::to_value)
        .map_err(|error| error.to_string())
}

fn decode(model: &str, value: Value) -> Result<Value, String> {
    include!("fixtures/parity/model_dispatch.rs")
}

#[test]
fn every_python_model_roundtrips_without_field_loss() {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/parity/models.json")).unwrap();
    assert!(fixtures.len() >= 414);
    let names: std::collections::BTreeSet<_> = fixtures
        .iter()
        .map(|case| case["model"].as_str().unwrap())
        .collect();
    assert_eq!(names.len(), 217);
    let mut failures = Vec::new();
    for fixture in fixtures {
        let model = fixture["model"].as_str().unwrap();
        let value = fixture["input"].clone();
        let actual = decode(model, value);
        match actual {
            Ok(actual) if actual == fixture["value"] => {}
            actual => failures.push(format!(
                "{model} {}: {actual:?}; expected {}",
                fixture["case"], fixture["value"]
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "{} model failures.\n{}",
        failures.len(),
        failures
            .iter()
            .take(6)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn every_python_required_model_field_rejects_omission() {
    let required: std::collections::BTreeMap<String, Vec<String>> =
        serde_json::from_str(include_str!("fixtures/parity/required_models.json")).unwrap();
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/parity/models.json")).unwrap();
    assert_eq!(required.len(), 217);
    let mut checked = 0;
    for (model, fields) in required {
        let complete = cases
            .iter()
            .find(|case| case["model"] == model && case["case"] == "full")
            .unwrap();
        for field in fields {
            let mut value = complete["input"].clone();
            value.as_object_mut().unwrap().remove(&field).unwrap();
            assert!(
                decode(&model, value).is_err(),
                "{model}.{field} accepted omission"
            );
            checked += 1;
        }
    }
    assert!(checked > 500);
}
