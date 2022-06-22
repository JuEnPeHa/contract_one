use near_sdk::{require};

use crate::{*, external::ext_self};

pub (crate) fn new_ticket_id(merchant_id: &AccountId, index: String) -> TicketID {
    let mut ticket_id: String = TICKET_PREFIX.to_string();
    ticket_id.push_str(&merchant_id.to_string());
    ticket_id.push_str(&env::current_account_id().to_string());
    ticket_id.push_str(&env::predecessor_account_id().to_string());
    ticket_id.push_str(&index);
    ticket_id
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

trait SellingFunctions {
    fn confirm_sell(&mut self, merchant_id: AccountId, buyer_id: AccountId, amount: u128);
    fn start_buy(&mut self, merchant_id: AccountId, amount: u128);
    fn confirm_by_mediator(&mut self, merchant_id: AccountId, buyer_id: AccountId, amount: u128);
}

#[near_bindgen]
impl SellingFunctions for Contract{
     fn confirm_sell(&mut self,
                merchant_id: AccountId,
                buyer_id: AccountId,
                amount: u128) {
        require!(env::signer_account_id() == merchant_id, "Only merchant can confirm sell");
        require!(env::predecessor_account_id() == AccountId::new_unchecked("p2p.near".to_string()), "Only p2p can confirm sell");
        let balance: u128 = self.balance_per_account.get(&merchant_id).unwrap_or(0u128);
        require!(balance >= amount, "Not enough balance");
        let new_balance: u128 = balance - amount;
        self.balance_per_account.insert(&merchant_id, &new_balance);
        ext_external::ft_transfer(
            buyer_id.to_string(), 
            amount.to_string(), 
            "".to_string(), 
            AccountId::new_unchecked("usdc.fakes.testnet".to_string()), 
            1,
            GAS_FOR_BASIC_CROSS_CONTRACT_CALL, 
        ).then(
        ext_self::after_sell_confirm(
            merchant_id, 
            buyer_id, 
            amount, 
            env::current_account_id(), 
            1, 
            GAS_FOR_BASIC_CROSS_CONTRACT_CALL
        )
        );
        }

        fn start_buy(&mut self, merchant_id: AccountId, amount: u128) {
            let index: u64 = self.selling_process.len();
            let ticket_id: TicketID = new_ticket_id(&merchant_id, index.to_string());
            let new_selling_process: SellingProcess = SellingProcess {
                merchant_id: merchant_id.clone(),
                buyer_id: env::signer_account_id(),
                mediator_id: self.mediator_id.clone(),
                accepted_merchant_id: false,
                accepted_buyer_id: true,
                accepted_mediator_id: false,
                amount,
                height: env::block_height(),
                last_height: env::block_height() + VALID_BLOCKS,
                votes_yes: 1u8,
                passed: false,
                rejected: false,
            };
            self.selling_process.insert(&ticket_id, &new_selling_process);
            self.selling_in_progress.insert(&ticket_id);

            let mut by_merchant_id = self.selling_by_merchant_id
            .get(&merchant_id).unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::BySellingByMerchantIdInner { merchant_id_hash: hash_account_id(&merchant_id) }
                    .try_to_vec().unwrap()
                )
            });
            by_merchant_id.insert(&ticket_id);
            self.selling_by_merchant_id.insert(&merchant_id, &by_merchant_id);

        }
        fn confirm_by_mediator(&mut self, merchant_id: AccountId, buyer_id: AccountId, ticket_id: TicketID) {
            require!(env::signer_account_id() == self.mediator_id, "Only mediator can confirm by mediator");
            require!(env::predecessor_account_id() == AccountId::new_unchecked("p2p.near".to_string()), "Only p2p can confirm by mediator");
            let mut selling_process = self.selling_process.get(&ticket_id).unwrap_or_else(|| {
                env::panic_str("Selling process not found")
            });
            require!(selling_process.mediator_id == self.mediator_id, "Only mediator can confirm by mediator");
            require!(selling_process.accepted_mediator_id == false, "Mediator already confirmed");
            require!(selling_process.accepted_buyer_id == true, "Buyer already confirmed");
            require!(selling_process.accepted_merchant_id == false, "Merchant already confirmed");
            require!(selling_process.height == env::block_height(), "Selling process already confirmed");
            require!(selling_process.last_height == env::block_height() + VALID_BLOCKS, "Selling process already confirmed");
            require!(selling_process.votes_yes == 1u8, "Selling process already confirmed");
            require!(selling_process.passed == false, "Selling process already confirmed");
            require!(selling_process.rejected == false, "Selling process already confirmed");
            

            
        }
}

// pub fn cross_dos(&self) -> Promise {
//     ext_example::ft_transfer(
//         "jephtest.testnet".to_string(), 
//         "10000000".to_string(), 
//         "".to_string(), 
//         AccountId::new_unchecked("usdc.fakes.testnet".to_string()), 
//         1, 
//         Gas(5_000_000_000_000),
//     )
// }