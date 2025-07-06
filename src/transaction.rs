use serde::{Deserialize, Serialize};
use sqlx::FromRow;  

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub sender_id: String,      
    pub recipient_id: String,  
    pub amount: f64,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: String,
    pub balance: f64,
}


#[derive(Debug, Deserialize)]
pub struct Incentive {
    pub amount: f64,
}
