use serde_json::{json, Value};

#[derive(Debug)]
pub struct MonoFieldQuery<'a> {
    query_type: &'a str,
    field: String,
    query_string: String,
}

impl<'a> MonoFieldQuery<'a> {
    pub fn new(query_type: &'a str, field: String, query_string: String) -> Result<Self, &'static str> {
        if query_type.is_empty() {
            return Err("query_type is required");
        }

        if field.is_empty() {
            return Err("field is required");
        }

        Ok(Self {
            query_type,
            field,
            query_string,
        })
    }

    pub fn to_json(&self) -> Value {
        json!({
            self.query_type: {
                &self.field: {
                    "query": self.query_string
                }
            }
        })
    }

    pub fn get_field(&self) -> &String {
        &self.field
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::MonoFieldQuery;

    mod common {
        use super::MonoFieldQuery;

        pub fn get_valid_instance<'a>() -> MonoFieldQuery<'a> {
            MonoFieldQuery::new("my_query", "field".to_string(), "search".to_string()).unwrap()
        }
    }

    #[test]
    fn should_construct_with_props() {
        let query = common::get_valid_instance();

        assert_eq!(query.query_type, "my_query");
        assert_eq!(query.field, "field");
        assert_eq!(query.query_string, "search");
    }

    #[test]
    fn should_return_error_when_query_type_is_empty() {
        let query = MonoFieldQuery::new("", "field".to_string(), "".to_string());

        query.unwrap_err();
    }

    #[test]
    fn should_return_error_when_field_is_empty() {
        let query = MonoFieldQuery::new("my_type", "".to_string(), "".to_string());

        query.unwrap_err();
    }

    #[test]
    fn should_retrieve_valid_json() {
        let query = common::get_valid_instance();

        assert_eq!(query.to_json(), json!({"my_query":{"field":{"query":"search"}}}));
    }
}
