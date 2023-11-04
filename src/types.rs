use codec::{Encode, Decode};
use scale_info::TypeInfo;
use frame_support::sp_runtime::BoundedVec;
use frame_system::Config as FrameConfig;
use crate::{Config, Pallet};
 
 /// A singluar message that defines the sender, recipient, UNIX timestamp, and the message content itself.
 #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
 #[scale_info(skip_type_params(T))]
 pub struct Message<T: Config + FrameConfig> {
     /// The sender of the message.
     pub(super) sender: T::AccountId,
     /// The recipient of the message.
     pub(super) recipient: T::AccountId,
     /// UNIX timestamp of when the message was sent.
     pub(super) time: u64,
     /// The message content as a byte array. No limit is placed on this for now, however this may change in the future.
     pub(super) message: Vec<u8>,
 }

 /// An active conversation, with the initiator of the conversation and recipient specified.  
 #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
 #[scale_info(skip_type_params(T))]
 pub struct ActiveConversation<T: Config + FrameConfig> {
     /// The initiator (address) of the conversation.
     pub(super) initiator_address: T::AccountId,
     /// The initiator (address) of the conversation.
     pub(super) initiator_name: Vec<u8>,
     /// The recipient, as specified by the initiator.
     pub(super) recipient_name: Vec<u8>,
     /// The initiator (address) of the conversation.
     pub(super) recipient_address: T::AccountId,
 }

 /// A conversation between two accounts that contains the initiator, recipient, and an array of messages.
 #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
 #[scale_info(skip_type_params(T))]
 pub struct Conversation<T: Config + FrameConfig> {
     /// The initiator of the conversation.
     pub(super) sender: T::AccountId,
     /// The recipient of the conversation.
     pub(super) recipient: T::AccountId,
     /// Array of messages between the initiator and recipient.
     pub(super) msgs: Vec<Message<T>>,
 }

 /// Basic structure of how a Uke User looks like
 /// Assocating an Account Id with a UTF-8 username (although we don't verify it here).
 #[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
 #[scale_info(skip_type_params(T))]
 pub struct User<T: Config + FrameConfig> {
     /// Caller's account id.
     pub(super) account_id: T::AccountId,
     /// Username associated with specified account id.
     pub(super) username: BoundedVec<u8, T::MaxUsernameLength>,
 }