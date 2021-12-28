#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::Randomness,
		traits::Currency,
		traits::ExistenceRequirement,
	};

	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{
		AtLeast32BitUnsigned,
		Bounded,  // ===> T::KittyIndex::max_value()
		One,
		Zero,
	};

	#[derive(Encode, Decode, TypeInfo)]
	pub struct Kitty(pub [u8; 16]);

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// type T::KittyIndex = u32;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
		type KittyIndex: Parameter + Member + AtLeast32BitUnsigned  + Default + Copy;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, T::KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),
		KittyBreed(T::AccountId, T::KittyIndex),
		KittyForSale(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
		KittySaleOut(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		AlreadyOwned,
		NotForSale,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	// pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;
	pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat,
		T::KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_prices)]
	pub type KittyPrices<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>)	-> DispatchResult {
			let who = ensure_signed(origin)?;

			// let kitty_id = match Self::kitties_count() {
			// 	Some(id) => {
			// 		ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
			// 		id
			// 	},
			// 	None => {
			// 		1
			// 	}
			// };
			let kitty_id = Self::next_kitty_id()?;

			let dna = Self::random_value(&who);

			// Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
			// Owner::<T>::insert(kitty_id, Some(who.clone()));
            // KittiesCount::<T>::put(kitty_id + 1);
			Self::save_db(&who, kitty_id, &dna);

			Self::deposit_event(Event::KittyCreate(who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, new_owner: T::AccountId,
						kitty_id: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);
			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));
			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex,
					 kitty_id_2: T::KittyIndex) -> DispatchResult {

			let who = ensure_signed(origin)?;
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id_1), Error::<T>::NotOwner);
			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id_2), Error::<T>::NotOwner);

			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			// let kitty_id = match Self::kitties_count() {
			// 	Some(id) => {
			// 		ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
			// 		id
			// 	},
			// 	None => {
			// 		1
			// 	}
			// };
			let kitty_id = Self::next_kitty_id()?;

			let dna1 = kitty1.0;
			let dna2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];

			for i in 0..dna1.len() {
				new_dna[i] = (selector[i] & dna1[i]) | (!selector[i] & dna2[i]);
			}

			// Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));
			// Owner::<T>::insert(kitty_id, Some(who.clone()));
			// KittiesCount::<T>::put(kitty_id + 1);
			Self::save_db(&who, kitty_id, &new_dna);

			Self::deposit_event(Event::KittyBreed(who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn sale(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			sale_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );

			KittyPrices::<T>::insert(kitty_id, sale_price);

			Self::deposit_event(
				Event::KittyForSale(
					who,
					kitty_id,
					sale_price));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_owner =
				Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			ensure!(
                Some(who.clone()) != Some(kitty_owner.clone()),
                Error::<T>::AlreadyOwned
            );

			let kitty_price =
				KittyPrices::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;

			//转账（购买）
			T::Currency::transfer(
				&who,
				&kitty_owner,
				kitty_price,
				ExistenceRequirement::KeepAlive,
			)?;

			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittyPrices::<T>::remove(kitty_id);

			Self::deposit_event(Event::KittySaleOut(
				who, kitty_id, Some(kitty_price)));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
			// let kitty_id = match Self::kitties_count() {
			// 	Some(id) => {
			// 		ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
			// 		id
			// 	},
			// 	None => {
			// 		0
			// 	}
			// };
			// Ok(kitty_id)

			let kitty_id = Self::kitties_count();
			if kitty_id == T::KittyIndex::max_value() {
				return Err(Error::<T>::KittiesCountOverflow.into());
			}

			Ok(kitty_id)
		}

		fn save_db(who: &T::AccountId, kitty_id: T::KittyIndex, dna: &[u8; 16]) {
			Kitties::<T>::insert(kitty_id, Some(Kitty(*dna)));
			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittiesCount::<T>::put(kitty_id + One::one());
		}
	}
}
