mod queries;

use queries::Query;
mod utils;

use serde_json::json;

struct RequestBody<T: Query> {
    query: T,
}

impl<T> RequestBody<T>
where
    T: Query,
{
    fn from_query(query: T) -> RequestBody<T> {
        RequestBody { query }
    }

    fn to_json(self) -> Result<String, serde_json::Error> {
        let value = json!({ "query": self.query.to_json() });

        serde_json::to_string(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::queries::match_all_query::MatchAllQuery;
    use super::*;

    #[test]
    fn should_retrieve_valid_json() {
        let request_body = RequestBody::from_query(MatchAllQuery::new());

        assert_eq!(
            request_body.to_json().unwrap(),
            "{\"query\":{\"match_all\":{}}}"
        );
    }
}
