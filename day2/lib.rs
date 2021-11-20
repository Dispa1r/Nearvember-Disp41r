use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;
use near_sdk::json_types::{Base58PublicKey, U128};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Promise, PromiseResult, PublicKey,
};
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct LinkDrop {
    pub accounts: HashMap<AccountId, u32>,
    counter : u32,
}

const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
/// Gas attached to the callback from account creation.
pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: u64 = 20_000_000_000_000;

/// Indicates there are no deposit for a callback for better readability.

#[near_bindgen]
impl LinkDrop {
    /// Allows given public key to claim sent balance.
    /// Takes ACCESS_KEY_ALLOWANCE as fee from deposit to cover account creation via an access key.
   
    #[payable]
    pub fn HelloWorld(&mut self, name : AccountId) -> String {
        assert!(
            env::is_valid_account_id(name.as_bytes()),
            "Invalid account id"
        );
        self.counter += 1;
        let mut z1 = String::new();
        z1.push_str(&name);
        let mut z2 = String::new();
        z2.push_str(&name);
        let mut s1 = "hello ,".to_string();
        let mut z = String::new();
        z.push_str(&name);
    //let s2 = name;
        s1 += &z;
        let mut s3 = ", you are the ".to_string();
        let string_value = self.counter.to_string();//int to String
        let mut s4 = " people to say hello";
        s3 += & string_value;
        s3 += &s4;
        s1 += &s3;
     

        let bool_key = self.accounts.contains_key(&z1);
        if !bool_key{
            let s5 = ", and it is your first time to say hello!";
            s1 += &s5;
            self.accounts.insert(z1,  1);
            let tmp = ONE_NEAR / (self.counter as u128);
            Promise::new(name).transfer(tmp);
            return s1;
        }else {
            
            let mut s5 = ", and it is your  ".to_string();
            let mut value_key = self.accounts.get(&z2).unwrap().clone();
            let mut s7 = value_key.to_string();
            let mut s6 = " time to say hello!".to_string();
            if value_key <= 10 {
                let tmp = ONE_NEAR / (self.counter as u128);
                Promise::new(name).transfer(tmp);
                value_key+=1;
                self.accounts.insert(z2, value_key);
                s5 += &s7;
                s5 += &s6;
                s1 += &s5;
                return s1;
            }else {
                return "you have say too many times today :(".to_string();
            }
            
            
        }
        
    
    }
    
}

