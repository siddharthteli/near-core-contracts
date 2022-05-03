use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId};
use serde::{Serialize, Deserialize};
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize )]
pub struct Party {
    owner: AccountId,
    symbol: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteDetails {
    party: Vector<Party>,
    votes: UnorderedMap<AccountId, Party>,
    total_votes: u128,
    party_vote_count: UnorderedMap<AccountId, u128>,
}


impl Default for VoteDetails {
     fn default() -> Self {
        VoteDetails {
            party : Vector::new(b"p"),
            votes: UnorderedMap::new(b"v"),
            total_votes: 0,
            party_vote_count: UnorderedMap::new(b"c"),
        }
    }
}

#[near_bindgen]
impl VoteDetails {
    #[init]
    pub fn init_state() -> Self {
        Self {
            party: Vector::new(b"p".to_vec()),
            votes: UnorderedMap::new(b"v".to_vec()),
            total_votes: 0,
            party_vote_count: UnorderedMap::new(b"c".to_vec()),
        }
    }

    #[payable]
    pub fn register_as_party(&mut self, _symbol: String) {
        if near_sdk::env::attached_deposit() <= 100 {
            env::panic_str("Send more money");
        }
        self.party.push(&Party {
            owner: env::signer_account_id(),
            symbol: _symbol,
        });
    }

    pub fn vote(&mut self, party_owner: Party) {
        for (account_id, party) in self.votes.iter() {
            if (account_id == env::signer_account_id()) {
                env::panic_str("You already voted");
            }
        }

        self.votes.insert(&env::signer_account_id(), &party_owner);
    }

 
}
