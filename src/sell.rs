use near_sdk::require;

use crate::*;

trait SellingFunctions {
    fn confirm_sell(&mut self, merchant_id: AccountId, buyer_id: AccountId, ammount: u128);
}

#[near_bindgen]
impl SellingFunctions for Contract{
     fn confirm_sell(&mut self,
                merchant_id: AccountId,
                buyer_id: AccountId,
                ammount: u128) {
        require!(env::signer_account_id() == merchant_id, "Only merchant can confirm sell");
        require!(env::predecessor_account_id() == AccountId::new_unchecked("p2p.near".to_string()), "Only p2p can confirm sell");
        let balance: u128 = self.balance_per_account.get(&merchant_id).unwrap_or(0u128);
        require!(balance >= ammount, "Not enough balance");
        let new_balance: u128 = balance - ammount;
        self.balance_per_account.insert(&merchant_id, &new_balance);
        ext_example::ft_transfer(
            buyer_id.to_string(), 
            ammount.to_string(), 
            "".to_string(), 
            AccountId::new_unchecked("usdc.fakes.testnet".to_string()), 
            1,
            Gas(5_000_000_000_000), 
        ).then(
self.selling_history.insert(&merchant_id, &self.history.get(&self.history.len()).unwrap())
        
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