use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub phone_number: u64,
    pub amount: u32,
}
