mod match_all_query;
mod match_none_query;

pub trait Query {
    fn get_type(&self) -> &str;
    fn to_json(&self) -> String;
}

pub trait BoostableQuery: Query {
    fn boost(&mut self, factor: f32) -> ();
}
