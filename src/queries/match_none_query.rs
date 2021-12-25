use serde_json::json;

use super::Query;

pub struct MatchNoneQuery;

impl Query for MatchNoneQuery {
    fn get_type(&self) -> &str {
        "match_none"
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
    fn should_return_match_none_from_type() {
        let query = MatchNoneQuery;

        assert_eq!(query.get_type(), "match_none")
    }

    #[test]
    fn should_retrieve_valid_json() {
        let query = MatchNoneQuery;

        assert_eq!(query.to_json(), "{\"match_none\":{}}");
    }
}