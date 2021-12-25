use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;
use serde_json::json;

use super::Query;

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchAllQuery {
    body: HashMap<String, String>
}

impl MatchAllQuery {
    pub fn new() -> MatchAllQuery {
        MatchAllQuery {
            body: HashMap::new()
        }
    }
}

impl Query for MatchAllQuery {
    fn get_type(&self) -> &str {
        "match_all"
    }

    fn set_boost(&mut self, boost: i32) -> () {
        self.body.insert("boost".to_string(), boost.to_string());
    }
    
    fn to_json(&self) -> String {
        let json = json!({
            self.get_type(): self.body
        });

        serde_json::to_string(&json).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_match_all_from_type() {
        let query = MatchAllQuery::new();

        assert_eq!(query.get_type(), "match_all")
    }

    #[test]
    fn should_set_boost_in_body() {
        let mut query = MatchAllQuery::new();

        query.set_boost(12);

        assert_eq!(query.body.get("boost").unwrap(), "12");
    }

    #[test]
    fn should_retrieve_valid_json() {
        let query = MatchAllQuery::new();

        assert_eq!(query.to_json(), "{\"match_all\":{}}");
    }

    #[test]
    fn should_retrieve_valid_json_with_boost() {
        let mut query = MatchAllQuery::new();

        query.set_boost(12);

        assert_eq!(query.to_json(), "{\"match_all\":{\"boost\":\"12\"}}")
    }
}