use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn store_message_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(Uke::store_message(
            RuntimeOrigin::signed(1),
            "yes".as_bytes().to_vec(),
            1,
            "abc123".as_bytes().to_vec(),
            1
        ));
    });
}

#[test]
fn registering_new_name_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(Uke::register(
            RuntimeOrigin::signed(1),
            "badery".as_bytes().to_vec()
        ));
    });
}

#[test]
fn username_too_long_should_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Uke::register(RuntimeOrigin::signed(1), "12345678901".as_bytes().to_vec()),
            Error::<Test>::UsernameExceedsLength
        );
    });
}

#[test]
fn invalid_id_should_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Uke::store_message(
                RuntimeOrigin::signed(1),
                "yes".as_bytes().to_vec(),
                1,
                "12345678901".as_bytes().to_vec(),
                1
            ),
            Error::<Test>::InvalidConvoId
        );
    });
}
