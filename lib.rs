#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct CoinFlipContract;

#[contractimpl]
impl CoinFlipContract {

    // Player starts the game with a bet
    pub fn start_game(env: Env, player: Address, bet: i128) {
        player.require_auth();

        env.storage().instance().set(&symbol_short!("player1"), &player);
        env.storage().instance().set(&symbol_short!("bet"), &bet);
    }

    // Second player joins the game
    pub fn join_game(env: Env, player: Address) {
        player.require_auth();

        let p1: Address = env.storage().instance().get(&symbol_short!("player1")).unwrap();
        env.storage().instance().set(&symbol_short!("player2"), &player);

        // simple pseudo random flip
        let rand = env.ledger().timestamp() % 2;

        let winner: Address = if rand == 0 { p1 } else { player };

        env.storage().instance().set(&symbol_short!("winner"), &winner);
    }

    // Read winner
    pub fn get_winner(env: Env) -> Address {
        env.storage().instance().get(&symbol_short!("winner")).unwrap()
    }
}