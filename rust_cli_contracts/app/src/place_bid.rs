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

use crate::{balance, cost, Query};

/// details
pub struct BuildSolution {
    /// The current balance of the account.
    pub item_id: Word,
    /// The hashed key of the sender.
    pub hashed_from_key: [Word; 4],
    /// The hashed key of the recipient.
    pub hashed_to_key: [Word; 4],
    /// The amount of tokens to transfer.
    pub amount: Word,
    /// The current balance of the sender.
    pub current_from_balance: Query,
    /// The current balance of the recipient.
    pub current_to_balance: Query,
    /// The current balance of the sender.
    pub current_cost: Query,
}

/// Builds a mint solution based on the provided data.
pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        item_id,
        hashed_from_key,
        hashed_to_key,
        amount,
        current_from_balance,
        current_to_balance,
        current_cost,
    } = build;

    let _cost = calculate_new_balance_sub(cost(current_cost)?, amount)?;
    let from_balance = calculate_new_balance_sub(balance(current_from_balance)?, amount)?;
    let to_balance = calculate_new_balance_add(balance(current_to_balance)?, amount)?;
    
    let pub_vars = super::token::PlaceBid::PubVars {
        item_id: item_id,
        bidder: hashed_from_key,
        amount
    };
    let mutations = super::token::storage::mutations()
        .balances(|map| map.entry(hashed_from_key, from_balance))
        .balances(|map| map.entry(hashed_to_key, to_balance))
        .item_owner(|map| map.entry(item_id, hashed_from_key))
        .auction_ended(|map| map.entry(item_id, true))
        .cost(|map| map.entry(item_id, amount));
    let solution = SolutionData {
        predicate_to_solve: super::token::PlaceBid::ADDRESS,
        decision_variables: Default::default(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };
    Ok(Solution {
        data: vec![solution],
    })
}


/// Calculates the new balance after minting tokens.
fn calculate_new_balance_add(balance: Word, amount: Word) -> anyhow::Result<Word> {
    balance
        .checked_add(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}

/// Calculates the new balance after minting tokens.
fn calculate_new_balance_sub(balance: Word, amount: Word) -> anyhow::Result<Word> {
    balance
        .checked_sub(amount)
        .ok_or(anyhow::anyhow!("Insufficient balance"))
}

