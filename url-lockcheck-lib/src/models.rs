use chrono::prelude::*;
use chrono::ParseResult;

#[derive(Deserialize, Debug, Clone)]
pub struct Register {
    
    pub id: Option<i32>,
    
    pub rsoc_id: String,
    
    #[serde(rename = "includeTime")]
    pub include_time: String,

    #[serde(rename = "rsocDate")]
    pub rsoc_date: String,
    
    pub org: String,
    
    pub org_act: String,
    
    pub url: String,
    
    pub domain: String,
    
    pub ip: String,
    
    pub country: String,
    
    pub proof: String,

}

#[derive(Deserialize, Debug, Clone)]
pub struct DetailInfo {
    
    #[serde(rename = "updateTime")]
    pub update_time: String,
    
    pub source: String,
    
    pub register: Option<Vec<Register>>,

}

impl DetailInfo {
     pub fn get_update_time(&self) -> ParseResult<DateTime<Utc>> {
         Utc.datetime_from_str(&self.update_time, "%Y-%m-%d %H:%M:%S")
     }
}