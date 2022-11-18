#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod one_million_dollar_shiden {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct OneMillionDollarShiden {
        /// Stores a single `bool` value on the storage.
        ticket_price: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Provide a detailed comment on the error
        InvalidAmount,
    }

    // result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl OneMillionDollarShiden {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(ticket_price: u64) -> Self {
            Self { ticket_price: ticket_price }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message, payable)]
        pub fn buy_ticket(&mut self) -> Result<()>{
            if (self.env().transferred_value() % self.ticket_price as u128) != 0{
                return Err(Error::InvalidAmount)
            }
            Ok(())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_ticket_price(&self) -> u64 {
            self.ticket_price
        }
        
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut one_million_dollar_shiden = OneMillionDollarShiden::new(1234567890);
            assert_eq!(one_million_dollar_shiden.get_ticket_price() == 1234567890, true);
        }
    }
}
