use std::result;

use crate::*;
use near_sdk::{is_promise_success, promise_result_as_success, require};

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
    ) /*-> Promise<void>*/ {
        require!(env::signer_account_id() == merchant_id, "Only merchant can confirm sell");
        require!(env::predecessor_account_id() == AccountId::new_unchecked("usdc.fakes.testnet".to_string()), "Only p2p can confirm sell");
        let mut transfer_succeeded = is_promise_success();
        if transfer_succeeded {
            let result = String::from_utf8(promise_result_as_success().unwrap());
            if result.is_ok() {
                let result = result.unwrap();
                if result == "false".to_string() {
                    transfer_succeeded = false;
                    let balance: u128 = self.balance_per_account.get(&merchant_id).unwrap_or(0u128);
                    let new_balance: u128 = balance + ammount;
                    self.balance_per_account.insert(&merchant_id, &new_balance);
                } else {
                    transfer_succeeded = true;
                }
            }
            env::log_str(format!("The funds has been transferred: {}", transfer_succeeded).as_str());
        }
    }
}
