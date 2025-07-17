//! This is a very simple implementation of Raffle contract.
//! It uses Chainlink VRF
//! Followed the best practices by referencing OpenZeppelin Stylus Contracts.

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use alloy_primitives::{uint, Address, U256, U8};
use stylus_sdk::{alloy_sol_types::sol, prelude::*, stylus_core::log};

const ENTRY_PRICE: &str = "1_000_000_000_000_000_000_000";
const ONE: U256 = uint!(1_U256);

sol! {

    #[derive(Debug)]
    event Raffle_RaffleDrawn(address indexed user);

    #[derive(Debug)]
    error Raffle_WrongDepositAmount();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    WrongDepositAmount(Raffle_WrongDepositAmount),
}

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]

    pub struct Raffle {
        uint256 count;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Raffle {
    pub fn enter_raffle(&mut self) -> Result<U256, Error> {
        let entry_price: U256 = U256::from(ENTRY_PRICE.parse::<U256>().unwrap());
        let amount_received = self.vm().msg_value();

        if amount_received == entry_price {
            log(
                self.vm(),
                Raffle_RaffleDrawn {
                    user: self.vm().msg_sender(),
                },
            );
            return Ok(ONE);
        } else {
            return Err(Error::WrongDepositAmount(Raffle_WrongDepositAmount {}));
        }
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
