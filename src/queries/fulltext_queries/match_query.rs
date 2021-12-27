use super::mono_field_query::MonoFieldQuery;
use crate::queries::Query;
use crate::utils::json_value_extensions::ValueExt;
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
}

impl Serialize for Operator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Operator::And => serializer.serialize_unit_variant("Operator", 0, "and"),
            Operator::Or => serializer.serialize_unit_variant("Operator", 1, "or"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum ZeroTermsQuery {
    None,
    All,
}

impl Serialize for ZeroTermsQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ZeroTermsQuery::None => serializer.serialize_unit_variant("ZeroTermsQuery", 0, "none"),
            ZeroTermsQuery::All => serializer.serialize_unit_variant("ZeroTermsQuery", 0, "all"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum RewriteType {
    ConstantScore,
    ScoringBoolean,
    ConstantScoreBoolean,
    ConstantScoreFilter,
    TopTermsBoost23,
    TopTerms15,
}

impl Serialize for RewriteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RewriteType::ConstantScore => {
                serializer.serialize_unit_variant("RewriteType", 0, "constant_score")
            }
            RewriteType::ScoringBoolean => {
                serializer.serialize_unit_variant("RewriteType", 1, "scoring_boolean")
            }
            RewriteType::ConstantScoreBoolean => {
                serializer.serialize_unit_variant("RewriteType", 2, "constant_score_boolean")
            }
            RewriteType::ConstantScoreFilter => {
                serializer.serialize_unit_variant("RewriteType", 3, "constant_score_filter")
            }
            RewriteType::TopTermsBoost23 => {
                serializer.serialize_unit_variant("RewriteType", 4, "top_terms_boost_23")
            }
            RewriteType::TopTerms15 => {
                serializer.serialize_unit_variant("RewriteType", 5, "top_terms_15")
            }
        }
    }
}

/// `match` query accepts text/numerics/dates, analyzes them, and constructs a query.
///
/// [Elasticsearch reference](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html)
pub struct MatchQuery<'a> {
    query: MonoFieldQuery<'a>,
    operator: Option<Operator>,
    zero_terms_query: Option<ZeroTermsQuery>,
    fuzzy_rewrite: Option<RewriteType>,
    lenient: Option<bool>,
    fuzziness: Option<String>,
    prefix_length: Option<u32>,
    max_expansions: Option<u32>,
    fuzzy_transpositions: Option<bool>,
    analyzer: Option<String>,
}

impl<'a> MatchQuery<'a> {
    fn new(field: String, query_string: String) -> Result<Self, &'static str> {
        let query_result = MonoFieldQuery::new("match", field, query_string);

        match query_result {
            Err(err) => Err(err),
            Ok(query) => Ok(Self {
                query,
                operator: None,
                zero_terms_query: None,
                fuzzy_rewrite: None,
                lenient: None,
                fuzziness: None,
                prefix_length: None,
                max_expansions: None,
                fuzzy_transpositions: None,
                analyzer: None,
            }),
        }
    }

    // Returns &mut Self so operations can be chained

    fn set_operator(&mut self, operator: Operator) -> &mut Self {
        self.operator = Some(operator);
        self
    }

    fn set_zero_terms_query(&mut self, zero_terms_query: ZeroTermsQuery) -> &mut Self {
        self.zero_terms_query = Some(zero_terms_query);
        self
    }

    fn set_fuzzy_rewrite(&mut self, rewrite: RewriteType) -> &mut Self {
        self.fuzzy_rewrite = Some(rewrite);
        self
    }

    fn set_lenient(&mut self, lenient: bool) -> &mut Self {
        self.lenient = Some(lenient);
        self
    }

    fn set_fuzziness(&mut self, fuzziness: String) -> &mut Self {
        self.fuzziness = Some(fuzziness);
        self
    }

    fn set_prefix_length(&mut self, prefix_length: u32) -> &mut Self {
        self.prefix_length = Some(prefix_length);
        self
    }

    fn set_max_expansions(&mut self, max_expansions: u32) -> &mut Self {
        self.max_expansions = Some(max_expansions);
        self
    }

    fn set_fuzzy_transpositions(&mut self, fuzzy_transpositions: bool) -> &mut Self {
        self.fuzzy_transpositions = Some(fuzzy_transpositions);
        self
    }

    fn set_analyzer(&mut self, analyzer: String) -> &mut Self {
        self.analyzer = Some(analyzer);
        self
    }
}

impl<'a> Query for MatchQuery<'a> {
    fn get_type(&self) -> &str {
        "match"
    }

    fn to_json(self) -> Value {
        let mut value = self.query.to_json();

        let query_options = &mut value[self.get_type()][&self.query.get_field()];

        query_options.add_if_it_was_set("operator", &self.operator);
        query_options.add_if_it_was_set("zero_terms_query", &self.zero_terms_query);
        query_options.add_if_it_was_set("fuzzy_rewrite", &self.fuzzy_rewrite);
        query_options.add_if_it_was_set("lenient", &self.lenient);
        query_options.add_if_it_was_set("fuzziness", &self.fuzziness);
        query_options.add_if_it_was_set("prefix_length", &self.prefix_length);
        query_options.add_if_it_was_set("max_expansions", &self.max_expansions);
        query_options.add_if_it_was_set("fuzzy_transpositions", &self.fuzzy_transpositions);
        query_options.add_if_it_was_set("analyzer", &self.analyzer);

        value
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    mod common {
        use super::*;

        pub fn get_valid_instance<'a>() -> MatchQuery<'a> {
            MatchQuery::new("field".to_string(), "search".to_string()).unwrap()
        }
    }

    #[test]
    fn should_set_operator() {
        let mut query = common::get_valid_instance();

        query.set_operator(Operator::And);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "operator": "and"}}})
        );
    }

    #[test]
    fn should_set_zero_terms_query() {
        let mut query = common::get_valid_instance();

        query.set_zero_terms_query(ZeroTermsQuery::All);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "zero_terms_query": "all"}}})
        );
    }

    #[test]
    fn should_set_fuzzy_rewrite() {
        let mut query = common::get_valid_instance();

        query.set_fuzzy_rewrite(RewriteType::ConstantScoreBoolean);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "fuzzy_rewrite": "constant_score_boolean"}}})
        );
    }

    #[test]
    fn should_set_lenient() {
        let mut query = common::get_valid_instance();

        query.set_lenient(true);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "lenient": true}}})
        );
    }

    #[test]
    fn should_set_fuzziness() {
        let mut query = common::get_valid_instance();

        query.set_fuzziness("AUTO".to_string());

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "fuzziness": "AUTO"}}})
        );
    }

    #[test]
    fn should_set_prefix_length() {
        let mut query = common::get_valid_instance();

        query.set_prefix_length(12);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "prefix_length": 12}}})
        );
    }

    #[test]
    fn should_set_max_expansions() {
        let mut query = common::get_valid_instance();

        query.set_max_expansions(14);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "max_expansions": 14}}})
        );
    }

    #[test]
    fn should_set_fuzzy_transpositions() {
        let mut query = common::get_valid_instance();

        query.set_fuzzy_transpositions(false);

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "fuzzy_transpositions": false}}})
        );
    }

    #[test]
    fn should_set_analyzer() {
        let mut query = common::get_valid_instance();

        query.set_analyzer("test".to_string());

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "analyzer": "test"}}})
        );
    }

    #[test]
    fn should_set_multiple_props() {
        let mut query = common::get_valid_instance();

        query
            .set_operator(Operator::And)
            .set_fuzziness("AUTO".to_string());

        assert_eq!(
            query.to_json(),
            json!({"match":{"field":{"query":"search", "operator": "and", "fuzziness": "AUTO"}}})
        );
    }
}
