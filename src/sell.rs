use crate::*;

trait SellingFunctions {
    fn sell(&mut self, merchant_id: AccountId, buyer_id: AccountId, ammount: u128);
}

#[near_bindgen]
impl SellingFunctions for Contract{
     fn sell() {
        
     }
}