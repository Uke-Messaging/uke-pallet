#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The max allowed length for a username.
		#[pallet::constant]
		type MaxUsernameLength: Get<u32>;

		/// The max allowed length for a conversation ID.
		#[pallet::constant]
		type MaxConvoIdLength: Get<u32>;

			/// The max allowed length for a conversation ID.
			#[pallet::constant]
			type MaxMessageLength: Get<u32>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Message<T: Config> {
		pub(super) sender: T::AccountId,
		pub(super) recipient: T::AccountId,
		pub(super) time: i64,
		pub(super) message: Vec<u8>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ActiveConversation<T: Config> {
		pub(super) initator: T::AccountId,
		pub(super) recipient: T::AccountId,
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Conversation<T: Config> {
		pub(super) sender: T::AccountId,
		pub(super) recipient: T::AccountId,
		pub(super) msgs: Vec<Message<T>>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct User<T: Config> {
		pub(super) account_id: T::AccountId,
		pub(super) username: BoundedVec<u8, T::MaxUsernameLength>,
	}

	#[pallet::storage]
	#[pallet::getter(fn conversations)]
	pub type Conversations<T: Config> = StorageMap<
		_,
		Twox64Concat,
		BoundedVec<u8, T::MaxConvoIdLength>,
		Vec<Message<T>>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn isactive)]
	pub type IsActiveConversation<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxConvoIdLength>, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn active)]
	pub type ActiveConversations<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<ActiveConversation<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn username)]
	pub type Usernames<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, User<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A message was sent
		MessageSent { sender: T::AccountId },
		/// A new active conversation was started
		ConvoStarted { sender: T::AccountId, recipient: T::AccountId },
		/// A new user was registered
		RegisteredUsername { user: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		ExceedsLength,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Dispatch and store a signed message by a user, as well as starts a conversation.  By nature, if a conversation
		/// doesn't exist, it will create one automatically.  Conversations are labelled as "Active" and then added to a list
		/// that can be retrieved later.
		///
		/// At most, we have 3 DB Writes
		#[pallet::weight((
			T::DbWeight::get().writes(3),
			Pays::No
		))]
		pub fn store_message(
			origin: OriginFor<T>,
			message: Vec<u8>,
			time: i64,
			convo_id: Vec<u8>,
			recipient: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let bounded_id: BoundedVec<u8, T::MaxConvoIdLength> =
				convo_id.clone().try_into().map_err(|()| Error::<T>::ExceedsLength)?;

			let mut conversation = <Conversations<T>>::get(&bounded_id);

			let new_message: Message<T> =
				Message { sender: sender.clone(), recipient: recipient.clone(), time, message };

			conversation.push(new_message);
			if !<IsActiveConversation<T>>::get(&bounded_id) {
				<IsActiveConversation<T>>::insert(&bounded_id, true);
				let new_active_convo: ActiveConversation<T> =
					ActiveConversation { initator: sender.clone(), recipient: recipient.clone() };
				let mut sender_conversations_addrs = <ActiveConversations<T>>::get(&sender);
				let mut recipient_conversation_addrs = <ActiveConversations<T>>::get(&recipient);

				sender_conversations_addrs.push(new_active_convo.clone());
				recipient_conversation_addrs.push(new_active_convo.clone());

				Self::deposit_event(Event::<T>::ConvoStarted {
					sender: sender.clone(),
					recipient: recipient.clone(),
				});

				<ActiveConversations<T>>::insert(&sender, sender_conversations_addrs);
				<ActiveConversations<T>>::insert(&recipient, recipient_conversation_addrs);
			}

			Self::deposit_event(Event::<T>::MessageSent { sender: sender.clone() });
			<Conversations<T>>::insert(&bounded_id, conversation);
			Ok(())
		}

		#[pallet::weight((
			T::DbWeight::get().writes(1),
			Pays::No
		))]
		pub fn register(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			let bound_name: BoundedVec<u8, T::MaxUsernameLength> =
				name.clone().try_into().map_err(|()| Error::<T>::ExceedsLength)?;
			let sender = ensure_signed(origin)?;
			let new_user: User<T> = User { account_id: sender.clone(), username: bound_name };
			Self::deposit_event(Event::<T>::RegisteredUsername { user: sender.clone() });
			<Usernames<T>>::insert(&sender.clone(), new_user);
			Ok(())
		}
	}
}
