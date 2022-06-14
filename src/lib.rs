use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, UnorderedMap, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise,
};

use crate::internal::*;
mod internal;
mod sell;

const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub contract_account_id: AccountId,
    pub children_account_ids: UnorderedMap<AccountId, UnorderedSet<U128>>,
    pub next_child_account_id: U128,
    pub balance_per_account: LookupMap<AccountId, u128>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Contract,
    ByChildrenAccountIds,
    ByChildrenAccountIdsInner { account_id_hash: CryptoHash },
    ByBalancePerAccount,
    ByBalancePerAccountInner { account_id_hash: CryptoHash },

}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new_meta(contract_account_id: AccountId) -> Self {
        let this: Contract = Self {
            contract_account_id,
            children_account_ids: UnorderedMap::new(StorageKey::ByChildrenAccountIds),
            next_child_account_id: U128(0),
            balance_per_account: LookupMap::new(StorageKey::ByBalancePerAccount),
        };
        this
    }

//     pub fn cross_contract_call(
//         &mut self,
//         account_id: AccountId,
//         amount: Balance,
//         gas: Gas,
//     ) -> Promise<AccountId, PanicOnDefault> {
//         let promise = Promise::new(account_id, amount, gas);
//         promise_result_as_success(promise)
// }
    // pub fn cross_tres(&self) -> Promise<AccountId, PanicOnDefault> {
    //     let promise = Promise::new("tres".to_string())
    //     .create_account().deploy_contract(code)
    // }

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

    #[private]
    fn _init_sub_account(&mut self, merchant_id: AccountId, /*sub_id: u128,*/ code_hash: Vec<u8>) /*-> Promise*/ {
        let sub_id = self.next_child_account_id;
        let sub_account_id = AccountId::new_unchecked(
            format!("{}.{}", sub_id.0.to_string().clone(), env::current_account_id())
        );
        self.next_child_account_id.0 += 1;
        //let sub_account_id_hash = CryptoHash::from_slice(&sub_account_id.as_bytes());
        Promise::new(sub_account_id.clone())
        .create_account()
        .transfer(CONTRACT_INIT_BALANCE)
        .deploy_contract(code_hash);

        let mut children_account_ids: UnorderedSet<U128> =
        self.children_account_ids.get(&merchant_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByChildrenAccountIdsInner { 
                    account_id_hash: hash_account_id(&merchant_id),
                }
                .try_to_vec().unwrap(),
            )
        });
        children_account_ids.insert(&sub_id);
        self.children_account_ids.insert(&merchant_id, &children_account_ids);
        //true
    }

}

#[ext_contract(ext_example)]
pub trait ExtExample {
    fn log_signer(&self);
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
}

