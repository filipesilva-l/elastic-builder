use serde::Serialize;
use serde_json::{json, Value};

pub trait ValueExt {
    fn add_if_it_was_set<T: Serialize>(&mut self, field_name: &str, value_option: &Option<T>);
}

impl ValueExt for Value {
    fn add_if_it_was_set<T: Serialize>(&mut self, field_name: &str, value_option: &Option<T>) {
        if let Some(value) = value_option {
            self[field_name] = json!(value);
        }
    }
}
