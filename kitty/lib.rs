#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// use ink_storage::traits::StorageLayout;

#[ink::contract]
mod kitty {
    use ink_storage::{
        traits::{SpreadAllocate, PackedLayout,SpreadLayout},
        Mapping,
    };
    use ink_prelude::vec::Vec;
    use scale::{Encode, Decode};

    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
    #[derive(PartialEq)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
    pub enum Gender{
        Male, 
        Female
    }

    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
    #[derive(PartialEq)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
    pub struct DrawKitty {
        // Stores a single `bool` value on the storage.
        dna: Vec<u8>,
        price: u32,
        gen_gender: Gender,
        create_date: u64
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Kitty {
        owner_kitty: Mapping<AccountId, DrawKitty>,
        
    }

    impl Kitty {
        // Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
        
            })
        }

        #[ink(message)]
        pub fn create_kitty(&mut self, _dna: Vec<u8>, _price: u32, _gen_gender: u8) -> Result<(),()> {
            let caller = Self::env().caller();
            let mut _gen = Gender::Female;
            if _gen_gender == 1{
                _gen = Gender::Male;
            }
            let _kitty = DrawKitty{
                dna: _dna,
                price: _price,
                gen_gender: _gen,
                create_date: Self::env().block_timestamp()
            };
            self.owner_kitty.insert(&caller, &_kitty);

            Ok(())
        }

        // /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_kitty(&self, accountId: AccountId) -> Result<(),()> {
            self.owner_kitty.get(accountId);
            Ok(())
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

        // We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let kitty = Kitty::default();
        //     assert_eq!(kitty.get(), false);
        // }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn create_kitty() {
           let mut contract = Kitty::default();
           let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
           let mut contract = Kitty::new();
            contract.create_kitty(vec![1], 1, 1);
            
            let get_kitty = contract.get_kitty(accounts.alice);
            let message = format!("thanks for instantiation {:?}", get_kitty);
            //ink_env::debug_println!(&message);
            assert_eq!(get_kitty , Ok(()));
        }

        #[ink::test]
        fn get_kitty(){

        }

        //fn create_contract()

    }

}
