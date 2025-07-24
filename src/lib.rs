//! This is a very simple implementation of Raffle contract.
//! It uses Chainlink VRF
//! Followed the best practices by referencing OpenZeppelin Stylus Contracts.

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use stylus_sdk::{
    alloy_primitives::{uint, Address, U256},
    alloy_sol_types::sol,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap, StorageU256},
    stylus_core::log,
};

const ENTRY_PRICE: &str = "1_000_000_000_000_000_000_000";
const ONE: U256 = uint!(1_U256);

sol! {

    #[derive(Debug)]
    event Raffle_RaffleDrawn(address indexed user);

    #[derive(Debug)]
    error Raffle_WrongDepositAmount();
    #[derive(Debug)]
    error Raffle_NotOpen();
    #[derive(Debug)]
    error Raffle_TransferFailed();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    WrongDepositAmount(Raffle_WrongDepositAmount),
    RaffleNotOpen(Raffle_NotOpen),
    TransferFailed(Raffle_TransferFailed),
}

#[derive(Debug)]
pub enum RaffleState {
    CLOSE,
    CALCULATING,
    OPEN,
}

#[storage]
pub struct Raffle {
    s_players: StorageMap<U256, StorageAddress>,
    s_raffle_state: StorageBool,
    s_total_players: StorageU256,
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Raffle {
    #[payable]
    pub fn enter_raffle(&mut self) -> Result<Address, Error> {
        let entry_price: U256 = U256::from(ENTRY_PRICE.parse::<U256>().unwrap());
        let amount_received = self.vm().msg_value();
        let player = self.vm().msg_sender();
        let total_no_of_players = self.s_total_players.get();

        if !self.s_raffle_state.get() {
            return Err(Error::RaffleNotOpen(Raffle_NotOpen {}));
        }

        if amount_received >= entry_price {
            log(self.vm(), Raffle_RaffleDrawn { user: player });
            self.s_players.setter(total_no_of_players + ONE).set(player);
            self.s_total_players.set(total_no_of_players + ONE);
            return Ok(player);
        } else {
            return Err(Error::WrongDepositAmount(Raffle_WrongDepositAmount {}));
        }
    }

    pub fn close_raffle(&mut self) -> Result<Address, Error> {
        let winner_index = self._get_winner_index()?;
        let player = self.s_players.get(winner_index);
        let amount = self.vm().balance(self.vm().contract_address());
        match self.vm().transfer_eth(player, amount) {
            Ok(_v) => return Ok(player),
            Err(_e) => return Err(Error::TransferFailed(Raffle_TransferFailed {})),
        }
    }
}

impl Raffle {
    pub fn _get_winner_index(&self) -> Result<U256, Error> {
        let block_no = U256::from(self.vm().block_number());
        let total_no_of_players = self.s_total_players.get();
        let winner_index = block_no % total_no_of_players;
        Ok(winner_index)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_counter() {
//         use stylus_sdk::testing::*;
//         let vm = TestVM::default();
//         let mut contract = Counter::from(&vm);

//         assert_eq!(U256::ZERO, contract.number());

//         contract.increment();
//         assert_eq!(U256::from(1), contract.number());

//         contract.add_number(U256::from(3));
//         assert_eq!(U256::from(4), contract.number());

//         contract.mul_number(U256::from(2));
//         assert_eq!(U256::from(8), contract.number());

//         contract.set_number(U256::from(100));
//         assert_eq!(U256::from(100), contract.number());

//         // Override the msg value for future contract method invocations.
//         vm.set_value(U256::from(2));

//         contract.add_from_msg_value();
//         assert_eq!(U256::from(102), contract.number());
//     }
// }
