use std::result;

use crate::*;
use near_sdk::{is_promise_success, promise_result_as_success};

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn after_sell_confirm(
        &mut self,
        merchant_id: AccountId,
        buyer_id: AccountId,
        ammount: u128,
    ) -> Promise;
}

#[near_bindgen]
impl Contract {
    pub fn after_sell_confirm(
        &mut self,
        merchant_id: AccountId,
        buyer_id: AccountId,
        ammount: u128,
    ) -> Promise<void> {
        let mut transfer_succeeded = is_promise_success();
        if transfer_succeeded {
            let result = String::from_utf8(promise_result_as_success().unwrap());
            if result.is_ok() {
                let result = result.unwrap();
                if result == "success".to_string() {
                    transfer_succeeded = true;
                } else {
                    transfer_succeeded = false;
                }
            }
            env::log_str(format!("The account is created and link is claimed: {}", transfer_succeeded).as_str());
        }
    }
}