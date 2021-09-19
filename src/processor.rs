use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use std::str::from_utf8;

pub fn process_to_lower(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    // Iterating over tx callers anc checking signature is valid
    let mut is_signed = accounts.len() > 0;

    for account_info in accounts.iter() {
        if let Some(address) = account_info.signer_key() {
            msg!("Signed by {:?}", address);
        } else {
            is_signed = false;
        }
    }

    if !is_signed {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let input = String::from(from_utf8(input).map_err(|err| {
        msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
        ProgramError::InvalidInstructionData
    })?);

    msg!("Lowercase input data: {}", input.to_lowercase());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        account_info::IntoAccountInfo, program_error::ProgramError, pubkey::Pubkey,
    };
    use solana_sdk::account::Account;

    #[test]
    fn test_signed() {
        let program_id = Pubkey::new(&[0; 32]);
        let input = "AbaCAba".as_bytes();

        let pubkey = Pubkey::new_unique();
        let mut account = Account::default();

        let signed_account_infos = vec![(&pubkey, true, &mut account).into_account_info()];

        assert_eq!(
            Ok(()),
            process_to_lower(&program_id, &signed_account_infos, input)
        );
    }

    #[test]
    fn test_unsigned() {
        let program_id = Pubkey::new(&[0; 32]);
        let input = "AbaCAba".as_bytes();

        let pubkey = Pubkey::new_unique();
        let mut account = Account::default();

        let signed_account_infos = vec![(&pubkey, false, &mut account).into_account_info()];

        assert_eq!(
            Err(ProgramError::MissingRequiredSignature),
            process_to_lower(&program_id, &signed_account_infos, input)
        );
    }

    #[test]
    fn test_without_signers() {
        let program_id = Pubkey::new(&[0; 32]);
        let input = "AbaCAba".as_bytes();

        let pubkey = Pubkey::new_unique();
        let mut account = Account::default();

        let signed_account_infos: Vec<AccountInfo> = Vec::new();

        assert_eq!(
            Err(ProgramError::MissingRequiredSignature),
            process_to_lower(&program_id, &signed_account_infos, input)
        );
    }
}