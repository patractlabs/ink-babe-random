#![cfg_attr(not(feature = "std"), no_std)]

use ink_babe_random::{CustomEnvironment, Randomness};
use ink_lang as ink;

#[ink::contract(env = crate::CustomEnvironment)]
mod example {
    use crate::Randomness;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct Example {}

    impl Example {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn current_epoch(&self) -> Randomness {
            self.env().extension().current_epoch().unwrap()
        }

        #[ink(message)]
        pub fn next_epoch(&self) -> Randomness {
            self.env().extension().next_epoch().unwrap()
        }

        #[ink(message)]
        pub fn randomness_of(&self, epoch: u64) -> Hash {
            self.env().extension().randomness_of(epoch).unwrap()
        }

        #[ink(message)]
        pub fn random(&self, subject: Vec<u8>) -> Hash {
            self.env().extension().random(subject.as_slice()).unwrap()
        }
    }
}
