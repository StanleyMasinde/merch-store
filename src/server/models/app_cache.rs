#[derive(toasty::Model)]
pub struct AppCache {
    #[key]
    #[auto]
    pub id: u64,

    #[column(type = varchar(255))]
    #[unique]
    pub key: String,

    #[column(type = varchar(255))]
    pub value: String,
    pub expires_at: jiff::Timestamp,
}
