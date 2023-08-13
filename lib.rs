#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod button {
    use ink::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use crate::button::psp22::Internal;
    use openbrush::contracts::psp22::extensions::burnable::*;
    use openbrush::contracts::psp22::extensions::metadata::*;
    use openbrush::contracts::psp22::Transfer as PSP22Transfer;
    use openbrush::traits::{Storage, String};

    // === EVENTS ===
    #[ink(event)]
    #[derive(Debug)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }

    #[ink(event)]
    #[derive(Debug)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    // === TYPES ===
    pub type Event = <Button as ContractEventBase>::Type;

    // === STRUCTS ===
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Button {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        cap: Balance,
    }
    impl PSP22 for Button {}
    impl PSP22Burnable for Button {}
    impl PSP22Metadata for Button {}
    impl PSP22Transfer for Button {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if _from.is_none() && (self.total_supply() + _amount) > self.cap() {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")));
            }
            Ok(())
        }
    }
    // emit events
    // https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md
    impl Internal for Button {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        ) {
            Button::emit_event(
                self.env(),
                Event::Transfer(Transfer {
                    from: _from,
                    to: _to,
                    value: _amount,
                }),
            );
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            Button::emit_event(
                self.env(),
                Event::Approval(Approval {
                    owner: _owner,
                    spender: _spender,
                    value: _amount,
                }),
            );
        }
    }
    impl Button {
        #[ink(constructor)]
        pub fn new(
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();
            assert!(instance._init_cap(initial_supply).is_ok());
            assert!(instance
                ._mint_to(instance.env().caller(), initial_supply)
                .is_ok());
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimal;
            instance
        }

        #[ink(message)]
        pub fn cap(&self) -> Balance {
            self.cap
        }

        fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
            if cap == 0 {
                return Err(PSP22Error::Custom(String::from("Cap must be above 0")));
            }
            self.cap = cap;
            Ok(())
        }

        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }
    }
}
