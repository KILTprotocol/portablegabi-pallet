#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult, ensure};
use system::ensure_signed;
use rstd::vec::Vec;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		/// The AccumulatorList contains all accumulator. It is a map which
		/// maps an account id and an index to an accumulator
		AccumulatorList get(accumulator_list): map (T::AccountId, u64) => Option<Vec<u8>>;

		/// The AccumulatorCounter stores for each attester the number of
		/// accumulator updates.
		AccumulatorCount get(accumulator_count): map T::AccountId => u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		fn deposit_event() = default;

		/// Updates the attestation
		pub fn update_accumulator(origin, accumulator: Vec<u8>) -> DispatchResult {
			let attester = ensure_signed(origin)?;

			let counter = if !<AccumulatorCount<T>>::exists(&attester) {
				0
			} else {
				<AccumulatorCount<T>>::get(&attester)
			};
			
			let next = counter.checked_add(1).ok_or("Overflow increasing accumulator index")?;
			ensure!(!<AccumulatorList<T>>::exists((&attester, next)),
					"Inconsistent accumulator counter");

			<AccumulatorList<T>>::insert((&attester, counter), &accumulator);
			<AccumulatorCount<T>>::insert(&attester, next);

			Self::deposit_event(RawEvent::Updated(attester, next, accumulator));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// An accumulator has been updated. Therefore an attestation has be revoked
		Updated(AccountId, u64, Vec<u8>),
	}
);

/// tests for this pallet
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the pallet, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type PortablegabiModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		new_test_ext().execute_with(|| {
			// Just a dummy test for the dummy function `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(PortablegabiModule::update_accumulator(Origin::signed(1), vec![1u8, 2u8, 3u8]));
			assert_ok!(PortablegabiModule::update_accumulator(Origin::signed(1), vec![4u8, 5u8, 6u8]));
			assert_ok!(PortablegabiModule::update_accumulator(Origin::signed(1), vec![7u8, 8u8, 9u8]));

			// There should be three accumulators inside the store
			assert_eq!(PortablegabiModule::accumulator_count(1), 3);

			// asserting that the stored value is equal to what we stored
			assert_eq!(PortablegabiModule::accumulator_list((1, 0)), Some(vec![1u8, 2u8, 3u8]));
			assert_eq!(PortablegabiModule::accumulator_list((1, 1)), Some(vec![4u8, 5u8, 6u8]));
			assert_eq!(PortablegabiModule::accumulator_list((1, 2)), Some(vec![7u8, 8u8, 9u8]));
		});
	}
}
