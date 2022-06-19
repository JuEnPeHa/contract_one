use near_sdk::{require};

use crate::{*, external::ext_self};

trait SellingFunctions {
    fn confirm_sell(&mut self, merchant_id: AccountId, buyer_id: AccountId, amount: u128);
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
        ext_example::ft_transfer(
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