use serde_json::{json, Value};

use super::Query;

/// The inverse of the `match_all` query, which matches no documents.
///
/// [Elasticsearch reference](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html)
 struct MatchNoneQuery;

impl Query for MatchNoneQuery {
    fn get_type(&self) -> &str {
        "match_none"
    }

    fn to_json(self) -> Value {
        json!({
            self.get_type(): {}
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_match_none_from_type() {
        let query = MatchNoneQuery;

        assert_eq!(query.get_type(), "match_none")
    }

    #[test]
    fn should_retrieve_valid_json() {
        let query = MatchNoneQuery;

        assert_eq!(query.to_json(), json!({ "match_none": { } }));
    }
}
