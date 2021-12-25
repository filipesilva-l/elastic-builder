mod match_all_query;
mod match_none_query;

pub trait Query {
    fn get_type(&self) -> &str;
    fn to_json(&self) -> String;
}
