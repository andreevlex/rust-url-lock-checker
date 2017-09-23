
#[derive(Deserialize, Debug, Clone)]
pub struct Register {
    
    pub id: Option<i32>,
    
    pub rsoc_id: String,
    
    pub includeTime: String,
    
    pub rsocDate: String,
    
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
    
    pub updateTime: String,
    
    pub source: String,
    
    pub register: Option<Vec<Register>>,

}
