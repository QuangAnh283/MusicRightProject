#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracterror, Address, BytesN, Env, Symbol, Vec,
    token::Client as TokenClient,
};

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    InvalidPercents = 1,
    NotAuthorized = 2,
    TrackNotFound = 3,
    IdentityNotFound = 4,
    TransferFailed = 5,
}

#[contract]
pub struct RoyaltyContract;

fn key_identity(env: &Env, id: &BytesN<32>) -> (Symbol, BytesN<32>) {
    (Symbol::new(env, "id"), id.clone())
}
fn key_track_meta(env: &Env, id: &BytesN<32>) -> (Symbol, BytesN<32>) {
    (Symbol::new(env, "meta"), id.clone())
}
fn key_track_len(env: &Env, id: &BytesN<32>) -> (Symbol, BytesN<32>) {
    (Symbol::new(env, "len"), id.clone())
}
fn key_track_stake(env: &Env, id: &BytesN<32>, idx: u32) -> (Symbol, (BytesN<32>, u32)) {
    (Symbol::new(env, "stk"), (id.clone(), idx))
}

fn split_amount(total: i128, shares: u32, total_shares: u32) -> i128 {
    (total * (shares as i128)) / (total_shares as i128)
}

#[contractimpl]
impl RoyaltyContract {
    pub fn register_identity(env: Env, id_hash: BytesN<32>, addr: Address) {
        addr.require_auth();

        env.storage()
            .persistent()
            .set(&key_identity(&env, &id_hash), &addr);
    }

    pub fn resolve_identity(env: Env, id_hash: BytesN<32>) -> Address {
        env.storage()
            .persistent()
            .get::<(Symbol, BytesN<32>), Address>(&key_identity(&env, &id_hash))
            .unwrap_or_else(|| env.panic_with_error(Error::IdentityNotFound))
    }

    pub fn register_track(
        env: Env,
        track_hash: BytesN<32>,
        metadata_ref: BytesN<32>,
        stakes_addrs: Vec<Address>,
        stakes_shares: Vec<u32>,
    ) {
        let n = stakes_addrs.len();

        if n != stakes_shares.len() || n == 0 {
            env.panic_with_error(Error::InvalidPercents);
        }

        let mut total_shares: u32 = 0;
        for i in 0..n {
            let s = stakes_shares.get(i).unwrap();
            total_shares = total_shares.checked_add(s).unwrap();
        }
        if total_shares != 10000 {
            env.panic_with_error(Error::InvalidPercents);
        }

        let store = env.storage().persistent();
        store.set(&key_track_meta(&env, &track_hash), &metadata_ref);
        store.set(&key_track_len(&env, &track_hash), &n);

        for i in 0..n {
            let addr = stakes_addrs.get(i).unwrap();
            let shares = stakes_shares.get(i).unwrap();
            store.set(&key_track_stake(&env, &track_hash, i as u32), &(addr, shares));
        }
    }

    pub fn get_track_len(env: Env, track_hash: BytesN<32>) -> u32 {
        env.storage()
            .persistent()
            .get::<(Symbol, BytesN<32>), u32>(&key_track_len(&env, &track_hash))
            .unwrap_or_else(|| env.panic_with_error(Error::TrackNotFound))
    }

    pub fn get_track_meta(env: Env, track_hash: BytesN<32>) -> BytesN<32> {
        env.storage()
            .persistent()
            .get::<(Symbol, BytesN<32>), BytesN<32>>(&key_track_meta(&env, &track_hash))
            .unwrap_or_else(|| env.panic_with_error(Error::TrackNotFound))
    }

    pub fn pay_royalty(env: Env, track_hash: BytesN<32>, token_contract: Address, amount: i128) {
        let store = env.storage().persistent();

        let n = store
            .get::<(Symbol, BytesN<32>), u32>(&key_track_len(&env, &track_hash))
            .unwrap_or_else(|| env.panic_with_error(Error::TrackNotFound));

        let token = TokenClient::new(&env, &token_contract);
        let contract_addr = env.current_contract_address();

        let mut total_shares: u32 = 0;
        for i in 0..n {
            let (_addr, s): (Address, u32) =
                store.get(&key_track_stake(&env, &track_hash, i)).unwrap();
            total_shares = total_shares.checked_add(s).unwrap();
        }

        let mut leftover: i128 = amount;
        for i in 0..n {
            let (addr, s): (Address, u32) =
                store.get(&key_track_stake(&env, &track_hash, i)).unwrap();
            let share_amount = split_amount(amount, s, total_shares);
            if share_amount > 0 {
                token.transfer(&contract_addr, &addr, &share_amount);
                leftover -= share_amount;
            }
        }

        if leftover > 0 {
            let (addr, _s): (Address, u32) =
                store.get(&key_track_stake(&env, &track_hash, 0)).unwrap();
            token.transfer(&contract_addr, &addr, &leftover);
        }
    }
}
#[cfg(test)]
mod test;



