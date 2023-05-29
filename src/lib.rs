use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, env};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Todo {
  records: UnorderedMap<i32, String>
}

impl Default for Todo {
  fn default() -> Self {
    Self { records: UnorderedMap::new(b"r") }
  }
}

#[near_bindgen]
impl Todo {
  pub fn get_records(self) {
    let log_message = format!("Your list: {:?}.", self.records.to_vec());
    env::log(log_message.as_bytes());
  } 

  pub fn add_record(&mut self, number: i32, message: String) {
    self.records.insert(&number, &message);
    let log_message = format!("{:?} record added.", self.records.get(&number).unwrap());
    env::log(log_message.as_bytes());
  }
  
  pub fn get_record(&self, number: i32) -> String {
    let message = match self.records.get(&number) {
                    Some(m) => m,
                    None => "There is no such record!".to_string()
                  };
    return message;
  }
  
  pub fn remove_record(&mut self, number: i32) {
    self.records.remove(&number);
  }

  pub fn remove_all_records(&mut self) {
    self.records.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::MockedBlockchain;
  use near_sdk::{testing_env, VMContext};

  fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
    VMContext {
      current_account_id: "alice.testnet".to_string(),
      signer_account_id: "robert.testnet".to_string(),
      signer_account_pk: vec![0, 1, 2],
      predecessor_account_id: "jane.testnet".to_string(),
      input,
      block_index: 0,
      block_timestamp: 0,
      account_balance: 0,
      account_locked_balance: 0,
      storage_usage: 0,
      attached_deposit: 0,
      prepaid_gas: 10u64.pow(18),
      random_seed: vec![0, 1, 2],
      is_view,
      output_data_receivers: vec![],
      epoch_height: 19,
    }
  }
  
  #[test]
  fn get_record() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Todo::default();
    contract.add_record(1, "Edit the thesis".to_string());
    contract.add_record(2, "Finish the thesis".to_string());
    assert_eq!("Edit the thesis", contract.get_record(1));
    assert_eq!("Finish the thesis", contract.get_record(2));
  }
  
  #[test]
  fn remove_record() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Todo::default();
    contract.add_record(1, "Finish the thesis".to_string());
    assert_eq!(contract.remove_record(1), ());
    assert_eq!(contract.records.len(), 0);
  }

  #[test]
  fn remove_all_records() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Todo::default();
    contract.add_record(1, "Finish the thesis".to_string());
    contract.remove_all_records();
    assert_eq!(contract.records.len(), 0);
  }
}
