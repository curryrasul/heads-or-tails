use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{serde::Serialize, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub player1: AccountId,
    pub player2: Option<AccountId>,
    pub reveal_time: Option<u64>,
    pub player1_commit: Vec<u8>,
    pub player1_reveal: Option<u128>,
    pub player2_commit: Vec<u8>,
    pub player2_reveal: Option<u128>,
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_conversion() {
        let n = 1_234_567_898_765_432u128;

        let v: Vec<u8> = n.to_be_bytes().to_vec();

        let n2 = u128::from_be_bytes(v.as_slice().try_into().unwrap());

        assert_eq!(n, n2);
    }
}
