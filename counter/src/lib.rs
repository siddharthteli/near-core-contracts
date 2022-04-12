use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, init, log, near_bindgen};
use serde::Serialize;
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Default, Clone, Serialize)]
// To store counter value.
pub struct Counter {
    counter: u64,
}

#[near_bindgen]
impl Counter {
    // Basically constructor of this contract.
    #[init]
    pub fn init_state(counter: u64) -> Self {
        assert!(
            !env::state_exists(),
            "Contract can only be initlised once bro"
        );
        Self { counter }
    }
    // Increments counter value by 1.
    // Takes mut self reference to update contract state.
    pub fn increment(&mut self) -> Self {
        self.counter += 1;
        self.clone()
    }

    // Decrements counter value by 1.
    pub fn decrement(&mut self) -> Self {
        self.counter -= 1;
        self.clone()
    }
    // Read state value
    pub fn read_count(&self) -> Self {
        self.clone()
    }
}
