#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata, PSP22Capped)]
#[openbrush::contract]
pub mod button {
    use ink::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use openbrush::traits::Storage;

    // === EVENTS ===
    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
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
        #[storage_field]
        cap: capped::Data,
    }

    #[overrider(psp22::Internal)]
    fn _emit_transfer_event(
        &self,
        from: Option<AccountId>,
        to: Option<AccountId>,
        amount: Balance,
    ) {
        self.env().emit_event(Transfer {
            from,
            to,
            value: amount,
        });
    }

    #[overrider(psp22::Internal)]
    fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
        self.env().emit_event(Approval {
            owner,
            spender,
            value: amount,
        });
    }

    impl Button {
        #[ink(constructor)]
        pub fn new(
            cap: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();
            assert!(capped::Internal::_init_cap(&mut instance, cap).is_ok());
            assert!(psp22::Internal::_mint_to(&mut instance, Self::env().caller(), cap).is_ok());
            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);
            instance
        }

        #[ink(message)]
        pub fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            let caller = Self::env().caller();
            if caller != account {
                let allowance: Balance = psp22::Internal::_allowance(self, &account, &caller);
                if allowance < amount {
                    return Err(PSP22Error::InsufficientAllowance);
                }

                psp22::Internal::_approve_from_to(self, account, caller, allowance - amount)?;
            }
            psp22::Internal::_burn_from(self, account, amount)
        }
    }
}
