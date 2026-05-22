/*
counter_2026_05_19 Testing module.

The testing module must cover:
1. loading devnet wallet
2. connecting to devnet
3. create a fresh counter account keypair
4. call initialize_counter, then increment
5. fetch counter back, assert count == 1
*/

use anchor_client::{
    CommitmentConfig,
    Client, Cluster,
};
use solana_keypair::{read_keypair_file};
use solana_pubkey::Pubkey;

#[test]
fn test_initialize() {
    let program_id = counter_2026_05_19::ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::try_from(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let tx = program
        .request()
        .accounts(counter_2026_05_19::accounts::Initialize {})
        .args(counter_2026_05_19::instruction::Initialize {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
