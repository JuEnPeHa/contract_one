use external::ext_self;
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
mod view;
mod external;

pub type TicketID = String;
const GAS_FOR_BASIC_CROSS_CONTRACT_CALL: Gas = Gas(5_000_000_000_000);

const CONTRACT_INIT_BALANCE: u128 = 1_245_949_999_000_000_000_000_000; //1000
static TICKET_PREFIX: &str = "ticket_";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ticket {
    pub merchant_id: AccountId,
    pub buyer_id: AccountId,
    pub amount: u128,
    pub height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AcceptProcess {
    pub merchant_id: AccountId,
    pub buyer_id: AccountId,
    pub mediator_id: AccountId,
    pub accepted_merchant_id: bool,
    pub accepted_buyer_id: bool,
    pub accepted_mediator_id: bool,
    pub amount: u128,
    pub height: u64,
    pub votes_yes: u8,
    pub passed: bool,
    pub rejected: bool,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub contract_account_id: AccountId,
    pub children_account_ids: UnorderedMap<AccountId, UnorderedSet<U128>>,
    pub next_child_account_id: U128,
    pub balance_per_account: LookupMap<AccountId, u128>,
    pub history: UnorderedMap<TicketID, Ticket>,
    pub selling_history: LookupMap<AccountId, UnorderedSet<TicketID>>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Contract,
    History,
    ByChildrenAccountIds,
    ByChildrenAccountIdsInner { account_id_hash: CryptoHash },
    ByBalancePerAccount,
    ByBalancePerAccountInner { account_id_hash: CryptoHash },
    BySellingHistory,
    BySellingHistoryInner { account_id_hash: CryptoHash },
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
            history: UnorderedMap::new(StorageKey::History),
            selling_history: LookupMap::new(StorageKey::BySellingHistory),
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

    //#[private]
    pub fn init_sub_account(&mut self, merchant_id: AccountId, /*sub_id: u128,*/ /*code_hash: Vec<u8>*/) /*-> Promise*/ {
        let code_hash: Vec<u8> = vec![1, 2, 3];
        let sub_id = self.next_child_account_id;
        let sub_account_id: AccountId = AccountId::new_unchecked(
            format!("{}.{}", sub_id.0.to_string().clone(), env::current_account_id())
        );
        self.next_child_account_id.0 += 1;
        //let sub_account_id_hash = CryptoHash::from_slice(&sub_account_id.as_bytes());
        Promise::new(sub_account_id.clone())
        .create_account()
        .transfer(CONTRACT_INIT_BALANCE)
        //.add_full_access_key(env::signer_account_pk())
        .deploy_contract(
            include_bytes!("../../contract_two/target/wasm32-unknown-unknown/release/contract_two.wasm").to_vec(),
        ).then(
            ext_external::new(
                merchant_id.clone(), 
                sub_account_id.clone(), 
                0, 
                Gas(5_000_000_000_000),
            )
        );

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

    pub fn add_balance_to_merchant(&mut self, merchant_id: AccountId, sub_id: AccountId, amount: u128) {
        env::log_str("add_balance_to_merchant");
        // let mut children_account_ids: UnorderedSet<U128> =
        // self.children_account_ids.get(&merchant_id).unwrap_or_else(|| {
        //     UnorderedSet::new(
        //         StorageKey::ByChildrenAccountIdsInner { 
        //             account_id_hash: hash_account_id(&merchant_id),
        //         }
        //         .try_to_vec().unwrap(),
        //     )
        // });
        // children_account_ids.remove(&sub_id);
        // self.children_account_ids.insert(&merchant_id, &children_account_ids);

        let mut balance_per_account: u128 = self.balance_per_account.get(&merchant_id)
        .unwrap_or(0u128);
        balance_per_account += amount;
        //balance_per_account -= CONTRACT_INIT_BALANCE;
        self.balance_per_account.insert(&merchant_id, &balance_per_account);
            self.add_amount_to_balance(
                merchant_id, 
                amount,
        );

    }

}

#[ext_contract(ext_external)]
pub trait ExtExternal {
    fn new(user_id: AccountId);
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
    fn delete_contract(&mut self);
}
