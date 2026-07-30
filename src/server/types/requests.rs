use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    phone_number: u64,
    amount: u32,
}
