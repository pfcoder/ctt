#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{codec::{Decode, Encode}, decl_error, decl_event, decl_module, decl_storage, dispatch};
/// Knowledge power pallet  with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use sp_runtime::RuntimeDebug;
use system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// https://github.com/pfcoder/substrate/wiki/Data-Interface
#[derive(Encode, Decode, PartialEq, Clone, RuntimeDebug)]
pub enum KnowledgeType {
	ProductPublish = 0,
	ProductIdentify,
	ProductTry,
	Comment,
	Unknown,
}

impl Default for KnowledgeType {
	fn default() -> Self { KnowledgeType::ProductPublish }
}

impl From<u8> for KnowledgeType {
	fn from(orig: u8) -> Self {
		match orig {
			0x0 => return KnowledgeType::ProductPublish,
			0x1 => return KnowledgeType::ProductIdentify,
			0x2 => return KnowledgeType::ProductTry,
			0x3 => return KnowledgeType::Comment,
			_ => return KnowledgeType::Unknown
		};
	}
}

type KnowledgeOf<T> = Knowledge<<T as system::Trait>::AccountId, <T as system::Trait>::Hash>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct Knowledge<AccountId, Hash> {
	owner: AccountId,
	knowledge_type: KnowledgeType,
	id: Hash,
	product_id: Hash,
	content_hash: Hash,
	tx_id: Option<Hash>,
	memo: Hash,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as KpStore {
		KnowledgeByIdHash get(fn knowledge_by_idhash):
			map hasher(opaque_blake2_256) <T as system::Trait>::Hash => KnowledgeOf<T>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		// SomethingStored(u32, AccountId),
		KnowledgeCreated(AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Some action needs to check specified account has enough balance to pay for gas fee.
		BalanceNotEnough
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		pub fn create_knowledge(origin,  knowledge_type: u8, knowledge_id: T::Hash, product_id: T::Hash,
			content_hash: T::Hash, tx_id:Option<T::Hash>, memo: T::Hash) -> dispatch::DispatchResult {

			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// TODO: Validation checks:
			// 1. check if knowledge_id is existed already.
			// 2. check if owner account has enough balance for pay gas fee.


			let k = Knowledge {
				owner: who.clone(),
				knowledge_type: knowledge_type.into(),
				 id: knowledge_id,
				product_id,
				content_hash,
				tx_id,
				memo
			};
			<KnowledgeByIdHash<T>>::insert(knowledge_id, k);
			Self::deposit_event(RawEvent::KnowledgeCreated(who));

			Ok(())
		}
	}
}
