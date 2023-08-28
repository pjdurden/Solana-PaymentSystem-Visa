# Solana-PaymentSystem-Visa
An implementation of a payment system on Solana blockchain

First install Rust and cargo using this command(linux and macos)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

For windows follow this - https://doc.rust-lang.org/cargo/getting-started/installation.html


to deploy solana program in devnet

set env as devnet
 solana config set --url https://api.testnet.solana.com

set a new keygen
 solana-keygen new --force

airdrop some tokens (to be used as gasfees)
 solana airdrop 1  FJCSzkZrBX82QQAq9rcfHgJj8ZmvTkCk2hve2cDXWjLW --url testnet

to deploy the program in testnet 
 solana program deploy ./target/deploy/Rust_Payment_System.so

current pubkey - FJCSzkZrBX82QQAq9rcfHgJj8ZmvTkCk2hve2cDXWjLW

Program id - J4Cazd4f8zShnaCZhCc54gBe46RNh9ukq9eYVzWcjeUm

