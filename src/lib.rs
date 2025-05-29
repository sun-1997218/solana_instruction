
use borsh::{BorshSerialize,BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint,
    entrypoint::ProgramResult, 
    msg, 
    program::invoke, 
    program_error::ProgramError,
    pubkey::Pubkey, 
    system_instruction, sysvar::{rent::Rent,Sysvar}
};

entrypoint!(process_isntruction);

pub fn process_isntruction(
    program_id : &Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8],
)->ProgramResult {
    let instruction = CounterInstruction::unpack(instruction_data)?;

    match instruction{
        CounterInstruction::InitializeCounter { initial_value } =>{
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncremementCounter =>process_increment_counter(program_id, accounts)?,
    };
    Ok(())
}


#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub enum CounterInstruction {
    InitializeCounter {initial_value:u64},
    IncremementCounter,
}
impl CounterInstruction{
    pub fn unpack(input:&[u8]) -> Result<Self,ProgramError>{
        let (&varient,rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;//获取指令类型，分离了第一个字节
        match varient{
            0=>{
                let initial_value = u64::from_be_bytes(
                    rest.try_into()
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Ok(Self::InitializeCounter { initial_value })
            }
            1=>Ok(Self::IncremementCounter),
            _=>Err(ProgramError::InvalidInstructionData),
        }
    }
}

fn process_initialize_counter(
    program_id : &Pubkey,
    accounts:&[AccountInfo],
    initial_value:u64,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    let counter_account = next_account_info(account_iter)?;
    let payer_account = next_account_info(account_iter)?;
    let system_program =next_account_info(account_iter)?;

    let account_space =8;//use u64 type to store account which take 8 bytes

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);  //获取最低租金

    invoke (
        &system_instruction::create_account(
            payer_account.key, 
            counter_account.key, 
            required_lamports, 
            account_space as u64, 
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
            ],
        )?;

    let counter_data = CounterAccount {
        count:initial_value,
    };

    let mut account_data = &mut counter_account.data.borrow_mut()[..];

    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value:{}",initial_value);
    Ok(())

}

fn process_increment_counter (program_id : &Pubkey,accounts:&[AccountInfo]) -> ProgramResult{


    let account_iter = &mut accounts.iter(); //获取账户的迭代器 可变
    let counter_account = next_account_info(account_iter)?; //


    if counter_account.owner!=program_id{
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut data = counter_account.data.borrow_mut();

    let mut counter_data :CounterAccount = CounterAccount::try_from_slice(&data)?;

    counter_data.count = counter_data
            .count
            .checked_add(1)
            .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.serialize (&mut &mut data[..])?;
    msg!("counter incremented to :{}",counter_data.count);
    Ok(())
}

#[derive(Debug,BorshDeserialize,BorshSerialize)]
pub struct CounterAccount{
    count :u64
}
   
