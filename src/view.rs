use crate::*;

trait ViewFunctions {
    fn erase_all_subaccounts(&mut self);
    fn view_balance(&self, merchant_id: AccountId) -> Balance;
    fn view_selling_history_by_merchant_id(&self, merchant_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<Ticket>;
    fn get_merchant_subaccount_ids(&self, merchant_id: AccountId) -> Vec<AccountId>;
}

#[near_bindgen]
impl ViewFunctions for Contract {
    fn erase_all_subaccounts(&mut self) {
        let id_numbers = 0u128..self.next_child_account_id.0;
        for id_number in id_numbers {
            let id = AccountId::new_unchecked(
                format!("{}.{}", id_number.to_string().clone(), env::current_account_id())
            );
            Promise::new(id).delete_account(AccountId::new_unchecked("contract_one.jeph.testnet".to_string()));
        }
    }
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
    fn get_merchant_subaccount_ids(&self, merchant_id: AccountId) -> Vec<AccountId> {
        let by_merchant_id: UnorderedSet<U128> = self.children_account_ids.get(&merchant_id).unwrap();
        by_merchant_id.as_vector();

        let mut accounts: Vec<AccountId> = vec![];
        by_merchant_id.iter().for_each(|sub_id| {
            let sub_account_id: AccountId = AccountId::new_unchecked(
                format!("{}.{}", sub_id.0.to_string().clone(), env::current_account_id())
            );
            accounts.push(sub_account_id);
        });
        accounts
    }
}