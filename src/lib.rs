use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {

}

#[near_bindgen]
impl Contract {
//     pub fn cross_contract_call(
//         &mut self,
//         account_id: AccountId,
//         amount: Balance,
//         gas: Gas,
//     ) -> Promise<AccountId, PanicOnDefault> {
//         let promise = Promise::new(account_id, amount, gas);
//         promise_result_as_success(promise)
// }
    pub fn cross_tres(&self) -> Promise<AccountId, PanicOnDefault> {
        let promise = Promise::new("tres".to_string())
        .
    }

    pub fn cross(&self) -> Promise {
        ext_example::log_signer(
            AccountId::new_unchecked("contract_two.jeph.testnet".to_string()), 
            0, 
            Gas(5_000_000_000_000),
        )
    }
    pub fn cross_dos(&self) -> Promise {
        ext_example::ft_transfer(
            "jephtest.testnet".to_string(), 
            "10000000".to_string(), 
            "".to_string(), 
            AccountId::new_unchecked("usdc.fakes.testnet".to_string()), 
            1, 
            Gas(5_000_000_000_000),
        )
    }

}
#[ext_contract(ext_example)]
trait ExtExample {
    fn log_signer(&self);
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
}

