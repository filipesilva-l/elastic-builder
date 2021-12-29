use serde::Serialize;

#[derive(PartialEq, Debug)]
pub enum Operator {
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