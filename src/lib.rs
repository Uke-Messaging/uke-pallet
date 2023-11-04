// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! \# Uke Pallet
//!
//! The Uke pallet implements a basic username and message storage system that adheres to the Uke Protocol.
//!
//! - \[`Config`]
//! - \[`Call`]
//! - \[`Pallet`]
//!
//! \## Overview
//!
//! The Uke pallet provides the functionality to perform basic messaging and identity assignment to
//! users on a given Substrate chain.
//!
//! - Allows for the propagation and retrieval of messages through a given Substrate instance.
//! - Allows for a username to be mapped / assigned to a given Account ID, to be retrieved later.
//!
//! \## Terminology
//!
//! - **Conversations**: Defined as having a initiator and recipient with a list of messages.  
//! Conversations can be marked as active or inactive.  If the Conversation is active, it is
//! added to the ActiveConversations StorageMap for both participants.
//!
//! - **Messages**: Defined as having a sender and receiver of a particular string of text.
//!
//! - **Convo ID**: A conversation id is how conversations are identified. It is the recipients and senders addresses hashed (right now, via SHA3-256), and is intended to be deterministic but unique.
//!
//! - **Usernames**: Usernames are simple UTF-8 strings assigned to Account IDs for the purpose of human-readable identification
//! of on-chain addresses.
//!
//! \## Interface
//!
//!
//! \### Dispatchable Functions
//!
//! - `store_message` - Store a message addressed to a specific recipient within their respective conversation.
//!
//! - `register` - Assigns a UTF-8 string name to the caller's address.
//!
//! \## Genesis Config
//!
//!
//! \## References
//!
//! - Username functionality inspired by the nick pallet: https://github.com/paritytech/substrate/tree/master/frame/nicks
//!
//! - Based on the wonderfully made Pallet Template: https://github.com/substrate-developer-hub/substrate-node-template/tree/main/pallets/template
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod pallet_ext;
pub mod types;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::weights::WeightInfo;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use types::{ActiveConversation, Message, User};

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The max allowed length for a username.
        #[pallet::constant]
        type MaxUsernameLength: Get<u32>;
        /// The max amount of messages per user per conversation
        #[pallet::constant]
        type MaxMessageAmount: Get<u32>;
        /// The max amount of conversations per user
        #[pallet::constant]
        type MaxActiveConversationAmount: Get<u32>;
        /// The max allowed length for a conversation ID.
        #[pallet::constant]
        type MaxConvoIdLength: Get<u32>;
        /// Information on runtime weights.
        type WeightInfo: WeightInfo;
    }

    /// Conversations in storage, retrievable via the specified ID.
    #[pallet::storage]
    #[pallet::getter(fn conversations)]
    pub type Conversations<T: Config> = StorageMap<
        _,
        Twox64Concat,
        BoundedVec<u8, T::MaxConvoIdLength>,
        BoundedVec<Message<T>, T::MaxMessageAmount>,
        ValueQuery,
    >;

    /// Mapping that specifies whether a conversation ID is active or not.
    #[pallet::storage]
    #[pallet::getter(fn isactive)]
    pub type IsActiveConversation<T: Config> =
        StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxConvoIdLength>, bool, ValueQuery>;

    /// Mapping of active conversations a user is participating in.
    #[pallet::storage]
    #[pallet::getter(fn active)]
    pub type ActiveConversations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<ActiveConversation<T>, T::MaxActiveConversationAmount>,
        ValueQuery,
    >;

    /// Registered account IDs as Users.
    #[pallet::storage]
    #[pallet::getter(fn username)]
    pub type Usernames<T: Config> =
        StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::MaxUsernameLength>, User<T>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A message was sent.
        MessageSent { sender: T::AccountId },
        /// A new active conversation was started.
        ConvoStarted {
            sender: T::AccountId,
            recipient: T::AccountId,
        },
        /// A new user was registered
        RegisteredUsername { user: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        UsernameExceedsLength,
        InvalidConvoId,
        ConversationLimitReached,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Dispatch and store a signed message by a user, as well as starts a conversation.  By nature, if a conversation
        /// doesn't exist, it will create one automatically.  Conversations are labelled as "Active" and then added to a mapping
        /// that can be retrieved later.
        ///
        /// Therefore, starting a new Conversation typically takes more chain resources, as it has extra mappings to write to.
        #[pallet::call_index(0)]
        #[pallet::weight((
			T::WeightInfo::store_message(),
			Pays::No
		))]
        pub fn store_message(
            origin: OriginFor<T>,
            message: Vec<u8>,
            time: u64,
            convo_id: Vec<u8>,
            recipient: T::AccountId,
            recipient_name: Vec<u8>,
            initiator_name: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let bounded_id: BoundedVec<u8, T::MaxConvoIdLength> =
                BoundedVec::<u8, T::MaxConvoIdLength>::try_from(convo_id)
                    .map_err(|_| Error::<T>::InvalidConvoId)?;

            if !<IsActiveConversation<T>>::get(&bounded_id) {
                <IsActiveConversation<T>>::insert(&bounded_id, true);

                let new_active_convo: ActiveConversation<T> = ActiveConversation {
                    initiator_address: sender.clone(),
                    initiator_name,
                    recipient_name,
                    recipient_address: recipient.clone(),
                };

                let mut sender_conversations_addrs = <ActiveConversations<T>>::get(&sender);
                let mut recipient_conversation_addrs = <ActiveConversations<T>>::get(&recipient);

                sender_conversations_addrs.try_push(new_active_convo.clone()).map_err(|_| Error::<T>::ConversationLimitReached)?;
                recipient_conversation_addrs.try_push(new_active_convo).map_err(|_| Error::<T>::ConversationLimitReached)?;

                Self::deposit_event(Event::<T>::ConvoStarted {
                    sender: sender.clone(),
                    recipient: recipient.clone(),
                });

                <ActiveConversations<T>>::insert(&sender, sender_conversations_addrs);
                <ActiveConversations<T>>::insert(&recipient, recipient_conversation_addrs);
            }

            let mut conversation = <Conversations<T>>::get(&bounded_id);

            let new_message: Message<T> = Message {
                sender: sender.clone(),
                recipient: recipient.clone(),
                time,
                message,
            };
            
            // TODO: WRAP MESSAGES AROUND
            conversation.try_push(new_message).map_err(|_| Error::<T>::ConversationLimitReached)?;

            Self::deposit_event(Event::<T>::MessageSent { sender });
            <Conversations<T>>::insert(&bounded_id, conversation);
            Ok(())
        }

        /// Registers a new user.
        ///
        /// Assigns the specified username to the caller's account id.
        /// Inspired by the nicks pallet: https://github.com/paritytech/substrate/tree/master/frame/nicks
        #[pallet::call_index(1)]
        #[pallet::weight((
			T::WeightInfo::register(),
			Pays::No
		))]
        pub fn register(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
            Self::foo();
            let bound_name: BoundedVec<u8, T::MaxUsernameLength> =
                BoundedVec::<u8, T::MaxUsernameLength>::try_from(name)
                    .map_err(|_| Error::<T>::UsernameExceedsLength)?;
            let sender = ensure_signed(origin)?;
            let new_user: User<T> = User {
                account_id: sender.clone(),
                username: bound_name.clone(),
            };
            Self::deposit_event(Event::<T>::RegisteredUsername { user: sender });
            <Usernames<T>>::insert(bound_name, new_user);
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight((
			T::WeightInfo::register(),
			Pays::No
		))]
        pub fn start_conversation(_origin: OriginFor<T>, _recipient: T::AccountId) -> DispatchResult {
            Ok(())
        }
    }
}
