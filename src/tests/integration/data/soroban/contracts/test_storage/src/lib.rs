#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

#[contract]
pub struct StorageContract;

const INST_KEY: Symbol = symbol_short!("INST");
const TEMP_KEY: Symbol = symbol_short!("TEMP");
const PRST_KEY: Symbol = symbol_short!("PRST");

#[contractimpl]
impl StorageContract {
    pub fn test_u32(env: Env, num1: u32, num2: u32, num3: u32) -> bool {
        env.storage().instance().set(&INST_KEY, &num1);
        env.storage().temporary().set(&TEMP_KEY, &num2);
        env.storage().persistent().set(&PRST_KEY, &num3);

        let snum1: u32 = env.storage().instance().get(&INST_KEY).unwrap();
        let snum2: u32 = env.storage().temporary().get(&TEMP_KEY).unwrap();
        let snum3: u32 = env.storage().persistent().get(&PRST_KEY).unwrap();
        
        num1 == snum1 && num2 == snum2 && num3 == snum3
    }

    pub fn test_u32_overwrite(env: Env, num1: u32, num2: u32) -> bool {
      env.storage().instance().set(&INST_KEY, &num1);
      env.storage().instance().set(&INST_KEY, &num2);
      
      let num: u32 = env.storage().instance().get(&INST_KEY).unwrap();
      
      num == num2
  }
}
