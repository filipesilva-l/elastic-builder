use serde::Serialize;
use serde_json::{json, Value};

pub trait ValueExt {
    fn add_if_it_was_set<T: Serialize>(&mut self, field_name: &str, value_option: &Option<T>);
}

impl ValueExt for Value {
    fn add_if_it_was_set<T: Serialize>(&mut self, field_name: &str, value_option: &Option<T>) {
        add_if_it_was_set(self, field_name, value_option, |value| json!(value))
    }
}

fn add_if_it_was_set<T>(
    json_value: &mut Value,
    field_name: &str,
    value_option: &Option<T>,
    get_value: fn(&T) -> Value,
) {
    if let Some(value) = value_option {
        json_value[field_name] = get_value(value);
    }
}
