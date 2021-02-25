#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{codec::{Decode, Encode},decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::{ensure_signed};
use sp_runtime::{RuntimeDebug};
use sp_std::prelude::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, RuntimeDebug)]
pub struct Transaction<T>{
    amount: u32,
    receiver: T,
	sender: T,
}

// type TEntry = Transaction<T as <frame_support::Trait>::AccountId>;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Transactionx: Vec<Transaction<T::AccountId>>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TransInfo(AccountId,AccountId,u32),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;
		// type TEntry = Transaction<T::AccountId>;
		fn deposit_event() = default;
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn transfer(origin, recv: T::AccountId, something: u32) -> dispatch::DispatchResult {
			
			let who = ensure_signed(origin)?;
			let tx = Transaction::<T::AccountId>{
				amount: something,
				receiver: recv,
				sender: who.clone(),
			};
			let mut x = Transactionx::<T>::get();
			x.push(tx.clone());

			// Update storage.
			Transactionx::<T>::put(x);
			// Emit an event.
			Self::deposit_event(RawEvent::TransInfo(who,tx.receiver,tx.amount));
			// Return a successful DispatchResult
			Ok(())
		}
	}
}
