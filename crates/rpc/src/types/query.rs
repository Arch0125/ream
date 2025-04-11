use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RandaoQuery {
    pub epoch: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorBalanceQuery {
    pub id: Option<Vec<String>>,
}