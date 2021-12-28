use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

pub const KITTY_1: u32 = 0;
pub const KITTY_2: u32 = 1;
pub const KITTY_3: u32 = 2;

#[test]
fn create_kitty_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        System::assert_last_event(
            mock::Event::KittiesModule(crate::Event::KittyCreate(
            ALICE, KITTY_1,
        )));

        assert_ok!(KittiesModule::create(Origin::signed(BOB)));
        System::assert_last_event(
            mock::Event::KittiesModule(crate::Event::KittyCreate(
                BOB, KITTY_2,
            )));
        assert_eq!(KittiesCount::<Test>::try_get(), Ok(2));

        assert_eq!(Owner::<Test>::try_get(KITTY_1).unwrap().unwrap(), ALICE);
        assert_eq!(Owner::<Test>::try_get(KITTY_2), Ok(Some(BOB)));
    })
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        // 创建猫咪
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));

        assert_eq!(KittiesCount::<Test>::get(), Some(1));
        assert_eq!(Owner::<Test>::try_get(KITTY_1), Ok(Some(ALICE)));

        // 转移猫咪
        assert_ok!(KittiesModule::transfer(Origin::signed(ALICE), BOB, KITTY_1));

        //检查事件
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyTransfer(
            ALICE, BOB, 0,
        )));

        // 查看猫咪当前归属
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
        assert_eq!(Owner::<Test>::try_get(KITTY_1), Ok(Some(BOB)));
    })
}

#[test]
fn breed_kitty_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_eq!(KittiesCount::<Test>::get(), Some(2));
        assert_eq!(Owner::<Test>::get(KITTY_1), Some(ALICE));
        assert_eq!(Owner::<Test>::get(KITTY_2), Some(ALICE));

        assert_ok!(KittiesModule::breed(Origin::signed(ALICE), KITTY_1, KITTY_2));

        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyBreed(
            ALICE, KITTY_3,
        )));

        assert_eq!(KittiesCount::<Test>::get(), Some(3));
        assert_eq!(Owner::<Test>::get(KITTY_3), Some(ALICE));
    })
}

#[test]
fn sale_kitty_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_ok!(KittiesModule::sale(Origin::signed(ALICE), KITTY_1, Some(5_000)));
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyForSale(
            ALICE,
            KITTY_1,
            Some(5_000),
        )));
    })
}

#[test]
fn buy_kitty_works() {
    new_test_ext().execute_with(|| {
        // Alice创建Kitty，并挂卖单
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_ok!(KittiesModule::sale(Origin::signed(ALICE), KITTY_1, Some(8_000)));
        assert_eq!(Owner::<Test>::get(KITTY_1), Some(ALICE));
        assert_eq!(KittyPrices::<Test>::get(KITTY_1), Some(8_000));

        // Bob购买Kitty
        assert_ok!(KittiesModule::buy(Origin::signed(BOB), KITTY_1));
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittySaleOut(
            BOB,
            KITTY_1,
            Some(8_000),
        )));

        // 检查是否已经收到转账
        assert_eq!(Balances::free_balance(ALICE), 10_000 + 8_000);

        // 检查是否已经转出
        assert_eq!(
            Balances::free_balance(BOB),
            20_000 - 8_000
        );

        // 检查拥有者
        assert_eq!(Owner::<Test>::get(KITTY_1), Some(BOB));
        // 检查挂单
        assert_eq!(KittyPrices::<Test>::get(KITTY_1), None);
    })
}