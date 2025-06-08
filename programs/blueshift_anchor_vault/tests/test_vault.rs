use anchor_lang::InstructionData;
#[cfg(test)]
use mollusk_svm::{program, result::Check, Mollusk};
use solana_sdk::{
    account::{Account, WritableAccount},
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    rent::Rent,
    signature::{Keypair, Signer},
    sysvar::Sysvar,
};

#[test]
fn test_intialize() {
    let (system_prorgam, system_account) = program::keyed_account_for_system_program();
}
