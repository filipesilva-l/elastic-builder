use serde_json::{json, Value};

use super::{BoostableQuery, Query};

/// The most simple query, which matches all documents, giving them all a `_score` of `1.0`.
///
/// [Elasticsearch reference](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html)
pub struct MatchAllQuery {
    boost: Option<f32>,
}

impl MatchAllQuery {
    fn new() -> MatchAllQuery {
        MatchAllQuery { boost: None }
    }
}

impl Query for MatchAllQuery {
    fn get_type(&self) -> &str {
        "match_all"
    }

    fn to_json(&self) -> String {
        let json: Value = match self.boost {
            Some(factor) => json!({ self.get_type(): { "boost": factor } }),
            None => json!({ self.get_type(): {} }),
        };

        serde_json::to_string(&json).unwrap()
    }
}

impl BoostableQuery for MatchAllQuery {
    fn boost(&mut self, factor: f32) -> () {
        self.boost = Some(factor)
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
    fn should_retrieve_valid_json() {
        let query = MatchAllQuery::new();

        assert_eq!(query.to_json(), "{\"match_all\":{}}");
    }

    #[test]
    fn should_set_boost() {
        let mut query = MatchAllQuery::new();

        query.boost(13.0);

        assert_eq!(query.boost, Some(13.0))
    }

    #[test]
    fn should_retrieve_json_with_boost() {
        let mut query = MatchAllQuery::new();

        query.boost(13.0);

        assert_eq!(query.to_json(), "{\"match_all\":{\"boost\":13.0}}");
    }
}
