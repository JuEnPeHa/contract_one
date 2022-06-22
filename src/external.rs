use std::result;

use crate::*;
use near_sdk::{is_promise_success, promise_result_as_success, require, env::log_str};

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn after_sell_confirm(
        &mut self,
        merchant_id: AccountId,
        amount: u128,
    ) -> Promise;
    //fn add_amount_to_balance(&mut self, merchant_id: AccountId, amount: u128);

}

#[near_bindgen]
impl Contract {
    pub fn after_sell_confirm(
        &mut self,
        merchant_id: AccountId,
        amount: u128,
    ) /*-> Promise<void>*/ {
        require!(env::predecessor_account_id() == AccountId::new_unchecked("usdc.fakes.testnet".to_string()), "Only after transfer can confirm sell");
        let transfer_succeeded = is_promise_success();
            let balance: u128 = self.balance_per_account.get(&merchant_id).unwrap_or(0u128);
            let new_balance: u128 = balance - amount;
            self.balance_per_account.insert(&merchant_id, &new_balance);
            env::log_str(format!("The funds has been transferred: {}", transfer_succeeded).as_str());
        }
    
    pub fn add_amount_to_balance(&mut self,
        merchant_id: AccountId,
        amount: u128) {
            log_str("add_amount_to_balance");
            log_str(format!("{}", merchant_id).as_str());
            log_str(format!("{}", amount).as_str());
            log_str(format!("{}", env::current_account_id()).as_str());
            log_str(format!("{}", env::predecessor_account_id()).as_str());
            log_str(format!("{}", env::signer_account_id()).as_str());
let balance: u128 = self.balance_per_account.get(&merchant_id).unwrap_or(0u128);
let new_balance: u128 = balance + amount;
self.balance_per_account.insert(&merchant_id, &new_balance);
}
}
