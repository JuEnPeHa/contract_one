use external::ext_self;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, UnorderedMap, LookupMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, promise_result_as_success, AccountId,
    Balance, BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise, require,
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
pub struct SellingProcess {
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
    //La cuenta en la qué se desplegó el contrato.
    pub contract_account_id: AccountId,
    //Mantenemos aquí si es usuario pagó para mantener el contrato o no.
    pub keep_account_open: UnorderedMap<AccountId, bool>,
    //Mantenemos el último contrato desplegado del usuario ya sea permanente o no.
    pub children_account_id: UnorderedMap<AccountId, U128>,
    //Mantenemos todas las cuentas qué se han usado por los usuarios.
    pub history_account_by_merchant_id: UnorderedMap<AccountId, UnorderedSet<U128>>,
    //El siguiente id de contrato aun no utilizado.
    pub next_child_account_id: U128,
    //Mantenemos todos los balances usdc de los usuarios.
    pub balance_per_account: LookupMap<AccountId, u128>,
    //Mantenemos los tickets por su ticket id.
    pub history: UnorderedMap<TicketID, Ticket>,
    //Mantenemos los tickets id por los usuarios.
    pub selling_history: LookupMap<AccountId, UnorderedSet<TicketID>>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Contract,
    History,
    ByKeepAccountOpen,
    ByKeepAccountOpenInner { account_id_hash: CryptoHash },
    ByChildrenAccountIds,
    ByChildrenAccountIdsInner { account_id_hash: CryptoHash },
    ByHistoryAccountByMerchantId,
    ByHistoryAccountByMerchantIdInner { merchant_id_hash: CryptoHash },
    ByBalancePerAccount,
    ByBalancePerAccountInner { account_id_hash: CryptoHash },
    BySellingHistory,
    BySellingHistoryInner { account_id_hash: CryptoHash },
}

#[near_bindgen]
impl Contract {

    //Inicialización del contrato.
    #[init]
    pub fn new_meta() -> Self {
        let contract_account_id = env::current_account_id();
        let this: Contract = Self {
            contract_account_id,
            keep_account_open: UnorderedMap::new(StorageKey::ByKeepAccountOpen),
            children_account_id: UnorderedMap::new(StorageKey::ByChildrenAccountIds),
            next_child_account_id: U128(0),
            balance_per_account: LookupMap::new(StorageKey::ByBalancePerAccount),
            history: UnorderedMap::new(StorageKey::History),
            selling_history: LookupMap::new(StorageKey::BySellingHistory),
            history_account_by_merchant_id: UnorderedMap::new(StorageKey::ByHistoryAccountByMerchantId),
        };
        this
    }

    #[payable]
    pub fn question_keep_subaccount_open(&mut self, keep: bool) -> bool {
        let mut max_number: u128 = 0;
        let contract_cost: u128 = CONTRACT_INIT_BALANCE;
        let merchant_id: AccountId = env::signer_account_id();
        let by_merchant_id: UnorderedSet<U128> = self.history_account_by_merchant_id
        .get(&merchant_id).unwrap();
        by_merchant_id.as_vector();
        by_merchant_id.iter().for_each(|number| {
            let temporal_number: u128 = number.clone().0;
            if temporal_number > max_number {
                max_number = temporal_number;
            }
        });
        self.children_account_id.insert(&merchant_id, &U128(max_number));
        let account_id: AccountId = AccountId::new_unchecked(
            format!("{}.{}", max_number, self.contract_account_id)
        );
        if keep {
            require!(
                env::attached_deposit() >= contract_cost,
                "Not enough attached deposit"
            );
            self.keep_account_open.insert(&merchant_id, &keep);
        } else {
            if self.keep_account_open.get(&merchant_id).unwrap_or(false) == true {
                
            } else {
            self.keep_account_open.remove(&merchant_id);
            ext_external::delete_contract(
                account_id, 
                0, 
                GAS_FOR_BASIC_CROSS_CONTRACT_CALL,
            );
            }

        }

        return keep;
        //let contract_account_id: AccountId = 
        
        
    }

    pub fn init_sub_account(&mut self, /*merchant_id: AccountId,*/ amount: Balance /*sub_id: u128,*/ /*code_hash: Vec<u8>*/) /*-> Promise*/ {
        let merchant_id = env::signer_account_id();
        if self.children_account_id.get(&merchant_id).is_none() {
            let sub_id = self.next_child_account_id;
            let sub_account_id: AccountId = AccountId::new_unchecked(
                format!("{}.{}", sub_id.0.to_string().clone(), env::current_account_id())
            );
            self.next_child_account_id.0 += 1;
            Promise::new(sub_account_id.clone())
            .create_account()
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(
                include_bytes!("../../contract_two/target/wasm32-unknown-unknown/release/contract_two.wasm").to_vec(),
            ).then(
                ext_external::new(
                    merchant_id.clone(), 
                    amount,
                    sub_account_id.clone(), 
                    0, 
                    Gas(5_000_000_000_000),
                )
            );
    
            let mut history_account_by_merchant_id: UnorderedSet<U128> =
            self.history_account_by_merchant_id.get(&merchant_id).unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::ByChildrenAccountIdsInner { 
                        account_id_hash: hash_account_id(&merchant_id),
                    }
                    .try_to_vec().unwrap(),
                )
            });
            history_account_by_merchant_id.insert(&sub_id);
            self.history_account_by_merchant_id.insert(&merchant_id, &history_account_by_merchant_id);
            self.children_account_id.insert(&merchant_id, &sub_id);
        } else if self.keep_account_open.get(&merchant_id).unwrap_or(false) == true {
            let sub_id: U128 = self.children_account_id.get(&merchant_id).unwrap();
            let sub_account_id: AccountId = AccountId::new_unchecked(
                format!("{}.{}", sub_id.0.to_string().clone(), env::current_account_id())
            );
                ext_external::update_new(
                    amount,
                    sub_account_id.clone(), 
                    0, 
                    Gas(5_000_000_000_000),
            );
        } else {
            let sub_id: U128 = self.next_child_account_id;
            let sub_account_id: AccountId = AccountId::new_unchecked(
                format!("{}.{}", sub_id.0.to_string().clone(), env::current_account_id())
            );
            self.next_child_account_id.0 += 1;
            Promise::new(sub_account_id.clone())
            .create_account()
            .transfer(CONTRACT_INIT_BALANCE)
            .deploy_contract(
                include_bytes!("../../contract_two/target/wasm32-unknown-unknown/release/contract_two.wasm").to_vec(),
            ).then(
                ext_external::new(
                    merchant_id.clone(), 
                    amount,
                    sub_account_id.clone(), 
                    0, 
                    Gas(5_000_000_000_000),
                )
            );

            let mut history_account_by_merchant_id: UnorderedSet<U128> =
            self.history_account_by_merchant_id.get(&merchant_id).unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::ByChildrenAccountIdsInner { 
                        account_id_hash: hash_account_id(&merchant_id),
                    }
                    .try_to_vec().unwrap(),
                )
            });
            history_account_by_merchant_id.insert(&sub_id);
            self.history_account_by_merchant_id.insert(&merchant_id, &history_account_by_merchant_id);
            self.children_account_id.insert(&merchant_id, &sub_id);
        }

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

        env::log_str("autodestruction");
        env::log_str(format!("signer: {}", env::signer_account_id()).as_str());
        env::log_str(format!("predecessor: {}", env::predecessor_account_id()).as_str());
        // env::log_str(format!("owner: {}", self.owner_id).as_str());
        // env::log_str(format!("user: {}", self.user_id).as_str());
        env::log_str(format!("merchant: {}", merchant_id).as_str());
        env::log_str(format!("amount: {}", amount).as_str());
        env::log_str(format!("promise_result_as_success: {:?}", promise_result_as_success()).as_str());
        env::log_str(format!("attached_gas: {:?}", env::prepaid_gas()).as_str());
        env::log_str(format!("used_gas: {:?}", env::used_gas()).as_str());
        env::log_str(format!("result: {:?}", env::promise_result(0)).as_str());
        require!(promise_result_as_success() != None, "No se pudo transferir el dinero, no hay suficiente");

        let mut balance_per_account: u128 = self.balance_per_account.get(&merchant_id)
        .unwrap_or(0u128);
        balance_per_account += amount;
        //balance_per_account -= CONTRACT_INIT_BALANCE;
        self.balance_per_account.insert(&merchant_id, &balance_per_account);
    }

}

#[ext_contract(ext_external)]
pub trait ExtExternal {
    fn new(user_id: AccountId, required_amount: Balance);
    fn update_new(new_required_amount: Balance);
    fn ft_transfer(&self, receiver_id: String, amount: String, memo: String);
    fn delete_contract(&mut self);
}
