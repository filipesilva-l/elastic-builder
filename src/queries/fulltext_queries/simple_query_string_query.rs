use serde_json::json;

use crate::{queries::{Query, common::Operator}, utils::json_value_extensions::ValueExt};

pub struct SimpleQueryStringQuery {
    query: String,
    flags: Option<String>,
    fields: Option<Vec<String>>,
    analyzer: Option<String>,
    default_operator: Option<Operator>
}

impl SimpleQueryStringQuery {
    fn new(query: String) -> SimpleQueryStringQuery {
        SimpleQueryStringQuery {
            query,
            flags: None,
            fields: None,
            analyzer: None,
            default_operator: None
        }
    }

    fn set_flags(&mut self, flag: String) -> &mut Self {
        self.flags = Some(flag);
        self
    }

    fn set_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    fn set_analyzer(&mut self, analyzer: String) -> &mut Self {
        self.analyzer = Some(analyzer);
        self
    }

    fn set_default_operator(&mut self, operator: Operator) -> &mut Self {
        self.default_operator = Some(operator);
        self
    }
}

impl Query for SimpleQueryStringQuery {
    fn get_type(&self) -> &str {
        "simple_query_string"
    }

    fn to_json(self) -> serde_json::Value {
        let mut query_options = json!({ "query": self.query });

        query_options.add_if_it_was_set("fields", &self.fields);
        query_options.add_if_it_was_set("flags", &self.flags);
        query_options.add_if_it_was_set("analyzer", &self.analyzer);
        query_options.add_if_it_was_set("default_operator", &self.default_operator);

        json!({ self.get_type(): query_options })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod common {
        use super::*;

        pub fn get_valid_instance() -> SimpleQueryStringQuery {
            SimpleQueryStringQuery::new("test".to_string())
        }
    }

    #[test]
    fn should_set_flag() {
        let mut query = common::get_valid_instance();

        query.set_flags("AND|OR".to_string());

        assert_eq!(
            query.to_json(),
            json!({ "simple_query_string": { "query": "test", "flags": "AND|OR" } })
        );
    }

    #[test]
    fn should_set_fields() {
        let mut query = common::get_valid_instance();

        query.set_fields(vec!["test".to_string(), "test2".to_string()]);

        assert_eq!(
            query.to_json(),
            json!({ "simple_query_string": { "query": "test", "fields": ["test", "test2"] } })
        );
    }

    #[test]
    fn should_set_analyzer() {
        let mut query = common::get_valid_instance();

        query.set_analyzer("test".to_string());

        assert_eq!(
            query.to_json(),
            json!({ "simple_query_string": { "query": "test", "analyzer": "test" } })
        );
    }

    fn should_set_default_operator() {
        let mut query = common::get_valid_instance();

        query.set_default_operator(Operator::And);

        assert_eq!(
            query.to_json(),
            json!({ "simple_query_string": { "query": "test", "default_operator": "and" } })
        );
    }
}
