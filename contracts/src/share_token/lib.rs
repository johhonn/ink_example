#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]
pub use share_token::ShareTokenRef;

#[openbrush::contract]
pub mod share_token {
    // imports from ink!
    use ink::codegen::{EmitEvent, Env};

    // imports from openbrush
    use openbrush::traits::Storage;
    use openbrush::traits::String;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::{
                extensions::{burnable::*, metadata::*, mintable::*},
                PSP22Error,
            },
        },
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ShareToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    // Section contains default implementation without any modifications

    impl PSP22 for ShareToken {}
    impl PSP22Metadata for ShareToken {}
    impl Ownable for ShareToken {}

    impl PSP22Mintable for ShareToken {
        /// override the `mint` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint_to(account, amount)
        }
    }

    // Implement Burnable Trait for our share
    impl PSP22Burnable for ShareToken {
        /// override the `burn` function to add the `only_owner` modifier
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._burn_from(account, amount)
        }
    }

    impl ShareToken {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut _instance = Self::default();
            _instance.metadata.name = name;
            _instance.metadata.symbol = symbol;
            _instance.metadata.decimals = 18;
            ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
            _instance
        }
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl psp22::Internal for ShareToken {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }
        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                spender: _spender,
                value: _amount,
            })
        }
    }
}
