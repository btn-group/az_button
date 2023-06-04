#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod button {

    // imports from openbrush
    use openbrush::contracts::psp22::extensions::burnable::*;
    use openbrush::contracts::psp22::extensions::metadata::*;
    use openbrush::contracts::psp22::Transfer;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Button {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        cap: Balance,
    }

    // Section contains default implementation without any modifications
    impl PSP22 for Button {}
    impl PSP22Burnable for Button {}
    impl PSP22Metadata for Button {}

    impl Transfer for Button {
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

    impl Button {
        #[ink(constructor)]
        pub fn new(
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Self {
            let mut _instance = Self::default();
            _instance
                ._mint_to(_instance.env().caller(), initial_supply)
                .expect("Should mint");
            _instance.metadata.name = name;
            _instance.metadata.symbol = symbol;
            _instance.metadata.decimals = decimal;
            _instance
        }

        #[ink(message)]
        pub fn cap(&self) -> Balance {
            self.cap
        }

        fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
            if cap <= 0 {
                return Err(PSP22Error::Custom(String::from("Cap must be above 0")));
            }
            self.cap = cap;
            Ok(())
        }
    }
}
