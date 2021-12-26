use serde_json::Value;

pub mod match_all_query;
pub mod match_none_query;
pub mod fulltext_queries;

pub trait Query {
    fn get_type(&self) -> &str;
    fn to_json(self) -> Value;
}

trait BoostableQuery: Query {
    fn boost(&mut self, factor: f32) -> ();
}
