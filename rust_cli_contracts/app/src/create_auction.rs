//! # Mint
//! Contains functionality for minting new tokens in the token contract.
 #![allow(unused)] 
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    Word,
};

use crate::{balance, Query};

/// details
pub struct BuildSolution {
    /// The current balance of the account.
    pub item_id: Word,
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to mint.
    pub amount: Word,
}

/// Builds a mint solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        item_id,
        hashed_key,
        amount,
    } = build;

    let pub_vars = super::token::CreateAuction::PubVars {
        item_id: item_id,
        initial_cost: amount,
        key: hashed_key,
    };
    let mutations = super::token::storage::mutations()
        .item_owner(|map| map.entry(item_id, hashed_key))
        .auction_ended(|map| map.entry(item_id, false))
        .cost(|map| map.entry(item_id, amount));
    let solution = SolutionData {
        predicate_to_solve: super::token::CreateAuction::ADDRESS,
        decision_variables: Default::default(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}
