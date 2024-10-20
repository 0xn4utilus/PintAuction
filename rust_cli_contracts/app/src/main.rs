//! Command-line interface for interacting with the token contract.
 #![allow(unused)]  
use anyhow::bail;
use clap::{Args, Parser, Subcommand};
use essential_app_utils::compile::compile_pint_project;
use essential_rest_client::{
    builder_client::EssentialBuilderClient, node_client::EssentialNodeClient,
};
use essential_signer::Signature;
use essential_types::{convert::word_4_from_u8_32, ContentAddress, PredicateAddress, Word};
use essential_wallet::Wallet;
use std::path::PathBuf;
use token::Query;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Essential wallet directory.
    /// If not set then a sensible default will be used (like ~/.essential-wallet).
    #[arg(short, long)]
    wallet: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Args)]
struct Mint {
    /// The account to mint from.
    account: String,
    /// The amount of token to mint.
    amount: Word,
    
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct Transfer {
    /// The account to transfer from.
    from_account: String,
    /// The account to transfer to.
    /// Hashed key as hex.
    to_account: String,
    /// The amount of token to mint.
    amount: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct PlaceBid {
    /// The account to transfer from.
    bidder_account: String,

    item_id: Word,
    
    amount: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct CreateAuction {
    /// The account to transfer from.
    owner_account: String,

    item_id: Word,
    
    initial_cost: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct StepAuction {
    /// The account to transfer from.
    owner_account: String,

    item_id: Word,
    
    decrease_amt: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The address of the builder to connect to.
    builder_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct Balance {
    /// The account name to get the balance of.
    account: String,
    /// The address of the node to connect to.
    node_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct ExternalBalance {
    /// The account hashed public key to get the balance of.
    /// Encoded as hex.
    account: String,
    /// The address of the node to connect to.
    node_api: String,
    /// The directory of the pint token contract.
    pint_directory: PathBuf,
}

#[derive(Args)]
struct ViewItemDetails {
    /// The account hashed public key to get the balance of.
    /// Encoded as hex.
    item_id: Word,
    /// The address of the node to connect to.
    node_api: String,
    /// The directory of the pint token contract.
    builder_api: String,
    pint_directory: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    Mint(Mint),
    Transfer(Transfer),
    Balance(Balance),
    ExternalBalance(ExternalBalance),
    CreateAuction(CreateAuction),
    StepAuction(StepAuction),
    PlaceBid(PlaceBid),
    ViewItemDetails(ViewItemDetails),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let Cli { wallet, command } = cli;
    let wallet = match &command {
        Command::ExternalBalance(_) => None,
        Command::ViewItemDetails(_) => None,
        _ => {
            let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
            let wallet = match wallet {
                Some(path) => essential_wallet::Wallet::new(&pass, path)?,
                None => essential_wallet::Wallet::with_default_path(&pass)?,
            };
            Some(wallet)
        }
    };
    match command {
        Command::Mint(args) => {
            println!(
                "minting {} for account: {}",
                args.amount, args.account
            );
            let wallet = wallet.unwrap();
            let addr = mint(wallet, args).await?;
            println!("sent mint solution: {}", addr);
        }
        Command::Transfer(args) => {
            println!(
                "transferring {} from account: {} to account: {}",
                args.amount, args.from_account, args.to_account
            );
            let wallet = wallet.unwrap();
            let addr = transfer(wallet, args).await?;
            println!("sent transfer solution: {}", addr);
        }
        Command::Balance(args) => {
            let Balance {
                account,
                node_api,
                pint_directory,
            } = args;
            println!("getting balance for account: {}", account);
            let mut wallet = wallet.unwrap();
            let hashed_key = hash_key(&mut wallet, &account);
            let balance = get_balance(hashed_key, node_api, pint_directory).await?;
            println!("balance is {}", balance);
        }
        Command::ExternalBalance(args) => {
            let ExternalBalance {
                account,
                node_api,
                pint_directory,
            } = args;
            println!("getting balance for account: {}", account);
            let hashed_key = word_4_from_u8_32(
                hex::decode(account)?
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("To key too large"))?,
            );
            let balance = get_balance(hashed_key, node_api, pint_directory).await?;
            println!("balance is {}", balance);
        }
        Command::CreateAuction(args) => {
            println!(
                "creation new auction item {} from account with initial cost {}",
                args.item_id, args.initial_cost
            );
            let wallet = wallet.unwrap();
            let addr = create_auction(wallet, args).await?;
            println!("sent Create Auction Solution: {}", addr);
        }
        Command::StepAuction(args) => {
            println!(
                "Decreasing item_id: {} by amount: {}",
                args.item_id, args.decrease_amt
            );
            let wallet = wallet.unwrap();
            let addr = step_auction(wallet, args).await?;
            println!("sent Step Auction Solution: {}", addr);
        }
        Command::PlaceBid(args) => {
            println!(
                "Placing Bid for item_id: {} for amount: {} from account: {}",
                args.item_id, args.amount, args.bidder_account
            );
            let wallet = wallet.unwrap();
            let addr = place_bid(wallet, args).await?;
            println!("sent Place bid Solution: {}", addr);
        }
        Command::ViewItemDetails(args) => {
            let ViewItemDetails {
                item_id,
                node_api,
                builder_api,
                pint_directory,
            } = args;
            println!("getting balance for item_id: {:?}", item_id);

            
            let node = EssentialNodeClient::new(node_api)?;
            let builder = EssentialBuilderClient::new(builder_api)?;
            

            let address = compile_address(pint_directory).await?;
            
            let cost_key = token::cost_key(item_id);
            let current_cost = node
                .query_state(address.contract.clone(), cost_key)
                .await?;
            
            let item_owner_key = token::get_item_owner_key(item_id);
            
            let status_key = token::auction_status_key(item_id);
            
            let status_bool = node
                .query_state(address.contract.clone(), status_key)
                .await?;
            
            let hashed_key = node
                .query_state(address.contract.clone(), item_owner_key)
                .await?;
            
                if let (Some(cost), Some(hashed_key), Some(status_bool)) = (current_cost.as_ref(), hashed_key.as_ref(), status_bool.as_ref()) {
                    if (status_bool[0] == 0) {
                        println!("Cost: {:?}\nOwner: {:?}\nActive: 'true'\n", cost[0], to_hex_string(hashed_key.to_vec()));
                    }else{
                        println!("Cost: {:?}\nOwner: {:?}\nActive: 'false'\n", cost[0], to_hex_string(hashed_key.to_vec()));
                    }
                } else {
                    println!("Error: current_cost or hashed_key is None");
                }
        }
    }
    Ok(())
}

/// Hashes the public key for an account.
fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

async fn mint(mut wallet: Wallet, args: Mint) -> anyhow::Result<ContentAddress> {
    let Mint {
        account,
        amount,
        node_api,
        builder_api,
        pint_directory,
    } = args;
    let address = compile_address(pint_directory).await?;
    let hashed_key = hash_key(&mut wallet, &account);

    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;

    let balance_key = token::balance_key(hashed_key);
    let balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let build_solution = token::mint::BuildSolution {
        current_balance: Query(balance),
        hashed_key,
        amount: amount
    };
    let solution = token::mint::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn transfer(mut wallet: Wallet, args: Transfer) -> anyhow::Result<ContentAddress> {
    let Transfer {
        amount,
        node_api,
        builder_api,
        pint_directory,
        from_account,
        to_account,
    } = args;
    let address = compile_address(pint_directory).await?;
    let hashed_from_key = hash_key(&mut wallet, &from_account);
    let hashed_to_key = word_4_from_u8_32(
        hex::decode(to_account)?
            .try_into()
            .map_err(|_| anyhow::anyhow!("To key too large"))?,
    );
    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;

    let balance_key = token::balance_key(hashed_from_key);
    let from_balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let balance_key = token::balance_key(hashed_to_key);
    let to_balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    let build_solution = token::transfer::BuildSolution {
        current_from_balance: Query(from_balance),
        current_to_balance: Query(to_balance),
        hashed_from_key,
        hashed_to_key,
        amount: amount,
    };
    let solution = token::transfer::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn create_auction(mut wallet: Wallet, args: CreateAuction) -> anyhow::Result<ContentAddress> {
    let CreateAuction {
        owner_account,
        item_id,
        initial_cost,
        node_api,
        builder_api,
        pint_directory,
    } = args;
    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;
    let address = compile_address(pint_directory).await?;
    let hashed_from_key = hash_key(&mut wallet, &owner_account);


    let build_solution = token::create_auction::BuildSolution {
        item_id: item_id,
        hashed_key: hashed_from_key,
        amount: initial_cost,
    };
    let solution = token::create_auction::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn step_auction(mut wallet: Wallet, args: StepAuction) -> anyhow::Result<ContentAddress> {
    let StepAuction {
        owner_account,
        item_id,
        decrease_amt,
        node_api,
        builder_api,
        pint_directory,
    } = args;
    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;
    let address = compile_address(pint_directory).await?;
    let hashed_from_key = hash_key(&mut wallet, &owner_account);

    let cost_key = token::cost_key(item_id);
    let current_cost = node
        .query_state(address.contract.clone(), cost_key)
        .await?;


    let build_solution = token::step_auction::BuildSolution {
        item_id: item_id,
        hashed_key: hashed_from_key,
        amount: decrease_amt,
        current_cost: Query(current_cost),
    };
    let solution = token::step_auction::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn place_bid(mut wallet: Wallet, args: PlaceBid) -> anyhow::Result<ContentAddress> {
    let PlaceBid {
        bidder_account,
        item_id,
        amount,
        node_api,
        builder_api,
        pint_directory,
    } = args;

    let node = EssentialNodeClient::new(node_api)?;
    let builder = EssentialBuilderClient::new(builder_api)?;
    let address = compile_address(pint_directory).await?;
    let hashed_from_key = hash_key(&mut wallet, &bidder_account);

    let item_owner_key = token::get_item_owner_key(item_id);

    let hashed_to_key_vec = node
        .query_state(address.contract.clone(), item_owner_key)
        .await?.unwrap();

    let hashed_to_key: [i64; 4] = match hashed_to_key_vec[..] {
        [a, b, c, d] => [a, b, c, d],
        _ => panic!("Expected a Vec with exactly 4 elements"),
    };    
    
    let balance_key = token::balance_key(hashed_from_key);
    let from_balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?.unwrap();


    let balance_key = token::balance_key(hashed_to_key);
    let to_balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    
        
    let cost_key = token::cost_key(item_id);
    let current_cost = node
        .query_state(address.contract.clone(), cost_key)
        .await?;
        

    let build_solution = token::place_bid::BuildSolution {
        item_id: item_id,
        hashed_from_key,
        hashed_to_key,
        amount: amount,
        current_from_balance: Query(Some(from_balance)),
        current_to_balance: Query(to_balance),
        current_cost: Query(current_cost),
    };
    let solution = token::place_bid::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca)
}

async fn get_balance(
    hashed_key: [Word; 4],
    node_api: String,
    pint_directory: PathBuf,
) -> anyhow::Result<Word> {
    let address = compile_address(pint_directory).await?;
    let node = EssentialNodeClient::new(node_api)?;

    let balance_key = token::balance_key(hashed_key);
    let balance = node
        .query_state(address.contract.clone(), balance_key)
        .await?;
    token::balance(Query(balance))
}

/// Compiles the contract and returns its address.
async fn compile_address(pint_directory: PathBuf) -> Result<PredicateAddress, anyhow::Error> {
    let counter = compile_pint_project(pint_directory).await?;
    let contract_address = essential_hash::contract_addr::from_contract(&counter);
    let predicate_address = essential_hash::content_addr(&counter.predicates[0]);
    let predicate_address = PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    Ok(predicate_address)
}

fn to_hex_string(vec: Vec<i64>) -> String {
    // Convert each i64 to bytes and concatenate them
    let mut bytes = vec![];
    for &num in &vec {
        bytes.extend_from_slice(&num.to_be_bytes()); // Use to_be_bytes for big-endian representation
    }

    // Convert the byte vector to a hexadecimal string
    let hex_string: String = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();

    // Prepend "0x" to indicate it's hexadecimal
    format!("0x{}", hex_string)
}