use serde_json::json;

use super::Query;

/// The most simple query, which matches all documents, giving them all a `_score` of `1.0`.
/// 
/// [Elasticsearch reference](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html)
pub struct MatchAllQuery;

impl Query for MatchAllQuery {
    fn get_type(&self) -> &str {
        "match_all"
    }

    fn to_json(&self) -> String {
        let json = json!({
            self.get_type(): {}
        });

        serde_json::to_string(&json).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_match_all_from_type() {
        let query = MatchAllQuery;

        assert_eq!(query.get_type(), "match_all")
    }

    #[test]
    fn should_retrieve_valid_json() {
        let query = MatchAllQuery;

        assert_eq!(query.to_json(), "{\"match_all\":{}}");
    }
}