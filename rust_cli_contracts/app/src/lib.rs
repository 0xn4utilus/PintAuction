#![deny(missing_docs)]
 #![allow(unused)]  
//! # Token
//! Taken contract front end implementation

use anyhow::bail;
use essential_types::{Key, Value, Word};

/// Module containing the token contract ABI.
#[allow(missing_docs)]
pub mod token {
    pint_abi::gen_from_file! {
        abi: "../pint/token/out/debug/token-abi.json",
        contract:  "../pint/token/out/debug/token.json",
    }
}

pub mod mint;
pub mod transfer;
pub mod place_bid;
pub mod step_auction;
pub mod create_auction;

/// Represents a query result, which may or may not contain a value.
pub struct Query(pub Option<Value>);

/// Generates the key for querying an account's balance.
pub fn balance_key(hashed_key: [Word; 4]) -> Key {
    let balance: Vec<_> = token::storage::keys::keys()
        .balances(|e| e.entry(hashed_key))
        .into();
    balance.into_iter().next().expect("Must be a key")
}


/// Extracts the balance from a Query result.
pub fn balance(balance: Query) -> anyhow::Result<Word> {
    let r = match balance.0 {
        Some(balance) => match &balance[..] {
            [] => 0,
            [balance] => *balance,
            _ => bail!("Expected single word, got: {:?}", balance),
        },
        None => 0,
    };
    Ok(r)
}

/// get item owner key
pub fn get_item_owner_key(item_id: Word) -> Key {
    let owner: Vec<_> = token::storage::keys::keys()
        .item_owner(|e| e.entry(item_id))
        .into();
    owner.into_iter().next().expect("Must be a key")
}


/// Generates the key for querying an account's balance.
pub fn cost_key(item_id: Word) -> Key {
    let cost: Vec<_> = token::storage::keys::keys()
        .cost(|e| e.entry(item_id))
        .into();
    cost.into_iter().next().expect("Must be a key")
}

/// Generates the key for auction status.
pub fn auction_status_key(item_id: Word) -> Key {
    let status: Vec<_> = token::storage::keys::keys()
        .auction_ended(|e| e.entry(item_id))
        .into();
    status.into_iter().next().expect("Must be a key")
}


/// Extracts the balance from a Query result.
pub fn cost(cost: Query) -> anyhow::Result<Word> {
    let r = match cost.0 {
        Some(cost) => match &cost[..] {
            [] => 0,
            [cost] => *cost,
            _ => bail!("Expected single word, got: {:?}", cost),
        },
        None => 0,
    };
    Ok(r)
}
