//! # Mint
//! Contains functionality for minting new tokens in the token contract.
 #![allow(unused)]  
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    Word,
};

use crate::{balance, Query, cost};

/// details
pub struct BuildSolution {
    /// The current balance of the account.
    pub item_id: Word,
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to mint.
    pub amount: Word,
    /// The current balance of the sender.
    pub current_cost: Query,
}

/// Builds a mint solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        item_id,
        hashed_key,
        amount,
        current_cost,
    } = build;

    let pub_vars = super::token::StepAuction::PubVars {
        item_id: item_id,
        decrease_amt: amount,
        key: hashed_key,
    };
    let cost = calculate_from_cost(cost(current_cost)?, amount)?;
    let mutations = super::token::storage::mutations()
        .cost(|map| map.entry(item_id, cost));
    let solution = SolutionData {
        predicate_to_solve: super::token::StepAuction::ADDRESS,
        decision_variables: Default::default(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}


/// Calculates the new cost.
fn calculate_from_cost(from_cost: Word, amount: Word) -> anyhow::Result<Word> {
    from_cost
        .checked_sub(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}
