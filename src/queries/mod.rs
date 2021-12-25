mod match_all_query;
mod match_none_query;

pub use match_all_query::MatchAllQuery;
pub use match_none_query::MatchNoneQuery;

trait Query {
    fn get_type(&self) -> &str;
    fn to_json(self) -> String;
}

trait BoostableQuery: Query {
    fn boost(&mut self, factor: f32) -> ();
}
