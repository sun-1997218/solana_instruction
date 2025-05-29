use borsh::{BorshDeserialize, BorshSerialize};

// 计数器账户数据结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u64,
}