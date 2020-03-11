#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap,
};
use system::ensure_signed;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {

        AvailableEnergy: map hasher(blake2_256) T::AccountId => u32;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        EnergyProduced(AccountId, u32),
        EnergyConsumed(AccountId, u32),
    }
);

// The pallet's dispatchable functions.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        fn generate_energy(origin, amount: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let amount_by_account = <AvailableEnergy<T>>::get(&sender);
            let total_amount = amount_by_account + amount;
            <AvailableEnergy<T>>::insert(&sender, total_amount);
            Self::deposit_event(RawEvent::EnergyProduced(sender, amount));
            Ok(())
        }

        fn consume_energy(origin, provider: T::AccountId, amount: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let amount_by_account = <AvailableEnergy<T>>::get(&provider);
            ensure!(amount_by_account > amount, "provider has not enough energy");
            let total_amount = amount_by_account - amount;
            <AvailableEnergy<T>>::insert(&sender, total_amount);
            Self::deposit_event(RawEvent::EnergyConsumed(sender, amount));
            Ok(())
        }

    }
}
