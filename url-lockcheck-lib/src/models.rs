use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Register {
    
    pub id: Option<i32>,
    
    pub rsoc_id: String,
    
    #[serde(rename = "includeTime")]
    #[serde(with = "mysql_date_format")]
    pub include_time: DateTime<Utc>,

    #[serde(rename = "rsocDate")]
    #[serde(with = "date_without_time_format")]
    pub rsoc_date: DateTime<Utc>,
    
    pub org: String,
    
    pub org_act: String,
    
    pub url: String,
    
    pub domain: String,
    
    pub ip: String,
    
    pub country: String,
    
    pub proof: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailInfo {
    
    #[serde(rename = "updateTime")]
    #[serde(with = "mysql_date_format")]
    pub update_time: DateTime<Utc>,
    
    pub source: String,
    
    pub register: Option<Vec<Register>>,

}

mod mysql_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";


    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<D>(D) -> Result<T, D::Error> where D: Deserializer
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod date_without_time_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";


    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<D>(D) -> Result<T, D::Error> where D: Deserializer
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Utc.ymd(1, 1, 1).and_hms(0, 0, 0));
        }
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}