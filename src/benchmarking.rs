//! Benchmarking setup for pallet-ukee

use super::*;

#[allow(unused)]
use crate::Pallet as Uke;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks! {
    store_message {
        let caller: T::AccountId = whitelisted_caller();

        let message = "yes".as_bytes().to_vec();
        let id = "abc123".as_bytes().to_vec();
        let time = 1;
        let bound_id: BoundedVec<u8, T::MaxConvoIdLength> = id.clone().try_into().unwrap();

    }: _(RawOrigin::Signed(caller.clone()), message, time, id, caller.clone())
    verify {
        assert_eq!(Conversations::<T>::get(bound_id).len(), 1);
    }


    register {
        let caller: T::AccountId = whitelisted_caller();
        let username: Vec<u8> =  "badery".as_bytes().to_vec();

    }: _(RawOrigin::Signed(caller.clone()), username)
    verify {
    assert_eq!(Usernames::<T>::get(&caller).unwrap().account_id, caller);
    }


    impl_benchmark_test_suite!(Uke, crate::mock::new_test_ext(), crate::mock::Test);
}
