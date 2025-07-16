//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```solidity
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use alloy_primitives::ruint::aliases::U256;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_sol_types::sol, prelude::*, stylus_core::log};

const ENTRY_PRICE: &str = "1_000_000_000_000_000_000_000";

sol! {
    event Raffle_RaffleDrawn(address indexed user);
    error Raffle_WrongDepositAmount();
}

#[derive(SolidityError)]
pub enum MultiCallErrors {
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
    pub fn enter_raffle(&mut self) -> Result<U256, MultiCallErrors> {
        let entry_price: U256 = U256::from(ENTRY_PRICE.parse::<U256>().unwrap());
        let amount_received = self.vm().msg_value();

        if amount_received == entry_price {
            // log(Raffle_RaffleDrawn {
            //     user: self.vm().msg_sender(),
            // });
            return Ok(U256::from(1));
        } else {
            return Err(MultiCallErrors::WrongDepositAmount(
                Raffle_WrongDepositAmount {},
            ));
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
