use caretta_id::CarettaId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub struct Frontmatter{
    pub id: CarettaId,
    #[serde(serialize_with = "serialize_date_time")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize_date_time")]
    pub updated_at:  NaiveDateTime,
    pub tags: Vec<String>
}

fn serialize_date_time<S>(value: &NaiveDateTime,serializer: S) -> Result<S::Ok, S::Error> 
where S: Serializer {
    let s = value.format("%Y-%m-%dT%H:%M").to_string();
    serializer.serialize_str(&s)
}