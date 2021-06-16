#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{DefaultEnvironment, Environment};
use ink_lang as ink;
pub enum CustomEnvironment {}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct BabeRandomness {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration: u64,
    pub randomness: [u8; 32],
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ErrorCode {}

impl ink_env::chain_extension::FromStatusCode for ErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[ink::chain_extension]
pub trait BabeRandomnessExt {
    type ErrorCode = ErrorCode;

    /// Reads from runtime storage.
    #[ink(extension = 0x00010000, handle_status = false, returns_result = false)]
    fn current_epoch() -> BabeRandomness;

    #[ink(extension = 0x00010001, handle_status = false, returns_result = false)]
    fn next_epoch() -> BabeRandomness;

    #[ink(extension = 0x00010002, handle_status = false, returns_result = false)]
    fn randomness_of(epoch: u64) -> <DefaultEnvironment as Environment>::Hash;

    #[ink(extension = 0x00010003, handle_status = false, returns_result = false)]
    fn random(subject: &[u8]) -> <DefaultEnvironment as Environment>::Hash;
}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type RentFraction = <DefaultEnvironment as Environment>::RentFraction;

    type ChainExtension = BabeRandomnessExt;
}
