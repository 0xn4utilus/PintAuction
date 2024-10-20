//! # Transfer
//! Contains functionality for transferring tokens between accounts in the token contract.
 #![allow(unused)]  
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    solution::{Solution, SolutionData},
    Word,
};

use crate::{balance, Query};


/// Contains all necessary information to build a transfer solution.
pub struct BuildSolution {
    /// The hashed key of the sender.
    pub hashed_from_key: [Word; 4],
    /// The hashed key of the recipient.
    pub hashed_to_key: [Word; 4],
    /// The amount of tokens to transfer.
    pub amount: Word,
    /// The current balance of the sender.
    pub current_from_balance: Query,
    /// The current balance of the recipient.
    pub current_to_balance: Query
}

/// Builds a transfer solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_from_key,
        hashed_to_key,
        amount,
        current_from_balance,
        current_to_balance,
    } = build;
    let from_balance = calculate_from_balance(balance(current_from_balance)?, amount)?;
    let to_balance = calculate_to_balance(balance(current_to_balance)?, amount)?;
    let pub_vars = super::token::Transfer::PubVars {
        key: hashed_from_key,
        to: hashed_to_key,
        amount,
    };
    
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_from_key, from_balance))
        .balances(|map| map.entry(hashed_to_key, to_balance));
    let solution = SolutionData {
        predicate_to_solve: super::token::Transfer::ADDRESS,
        decision_variables: Default::default(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}

/// Calculates the new balance for the sender after transferring tokens.
fn calculate_from_balance(from_balance: Word, amount: Word) -> anyhow::Result<Word> {
    from_balance
        .checked_sub(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}

/// Calculates the new balance for the recipient after receiving tokens.
fn calculate_to_balance(to_balance: Word, amount: Word) -> anyhow::Result<Word> {
    to_balance
        .checked_add(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}
