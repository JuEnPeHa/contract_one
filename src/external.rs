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
    ) -> Promise {
        let mut transfer_succeeded = is_promise_success();
        env::log_str(String::from_utf8(promise_result_as_success().unwrap()).as_str());
        if transfer_succeeded {
            
        }
    }
}