use crate::*;

trait ViewFunctions {
    fn view_balance(&self, merchant_id: AccountId) -> Balance;
    fn view_selling_history_by_merchant_id(&self, merchant_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<Ticket>;
}

#[near_bindgen]
impl ViewFunctions for Contract {
    fn view_balance(&self, merchant_id: AccountId) -> Balance {
        self.balance_per_account.get(&merchant_id).unwrap_or(0u128)
    }
    fn view_selling_history_by_merchant_id(&self, merchant_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<Ticket> {
        let by_merchant_id: Option<UnorderedSet<TicketID>> = self.selling_history.get(&merchant_id);
        //.unwrap_or(vec![])
        let history = if let Some(by_merchant_id) = by_merchant_id {
            by_merchant_id
        } else {
            return vec![];
        };
        let keys = history.as_vector();
        let start: u128 = u128::from(from_index.unwrap_or(U128(0u128)));
        keys.iter().skip(start as usize).take(limit.unwrap_or(0) as usize)
        .map(|ticket: TicketID| self.history.get(&ticket).unwrap())
        .collect()
    }
}