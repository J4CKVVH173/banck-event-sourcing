use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddPayload {
    pub amount: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RemovePayload {
    pub amount: usize,
}
