#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url {
    pub id : Option<String>,
    #[serde(rename="longUrl")]
    pub long_url: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Counter {
    pub id : String,
    pub count: usize
}
