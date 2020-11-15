pub(crate) type Snowflake = u64;

pub struct Guild {
    pub id: Snowflake,
    pub prefix: String,
}

pub struct Sound {
    pub id: u64,
    pub name: String,
    pub aliases: Vec<String>,
    pub source: String,
    pub uploader: Snowflake,
    pub upload_time: u64,
    pub length: f64,
}
