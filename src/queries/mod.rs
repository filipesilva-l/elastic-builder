mod match_all_query;

pub trait Query {
    fn set_boost(&mut self, boost: i32) -> ();
    fn get_type(&self) -> &str;
    fn to_json(&self) -> String;
}