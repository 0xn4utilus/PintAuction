//! # Mint
//! Contains functionality for minting new tokens in the token contract.
 #![allow(unused)]  

use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Solution, SolutionData},
    Word,
};

use crate::{balance, Query};

/// details
pub struct BuildSolution {
    /// The current balance of the account.
    pub current_balance: Query,
    /// The hashed key of the account.
    pub hashed_key: [Word; 4],
    /// The amount of tokens to mint.
    pub amount: Word,
}

/// Builds a mint solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        current_balance,
        hashed_key,
        amount,
    } = build;
    let balance = calculate_new_balance(balance(current_balance)?, amount)?;
    let pub_vars = super::token::Mint::PubVars {
        key: hashed_key,
        amount
    };
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_key, balance));
    let solution = SolutionData {
        predicate_to_solve: super::token::Mint::ADDRESS,
        decision_variables: Default::default(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}

/// Calculates the new balance after minting tokens.
fn calculate_new_balance(balance: Word, amount: Word) -> anyhow::Result<Word> {
    balance
        .checked_add(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}
