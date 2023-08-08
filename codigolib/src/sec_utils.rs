use crate::errors::*;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

// ==== SECURITY RELATED =======

pub fn verify_ownership(program_id: &Pubkey, account_infos: &[&AccountInfo]) -> ProgramResult {
    for account_info in account_infos {
        if *account_info.owner != *program_id {
            return Err(SecurityError::WrongAccountOwner.into());
        }
    }

    Ok(())
}

pub fn verify_len(type_len: usize, account_infos: &[&AccountInfo]) -> ProgramResult {
    for account_info in account_infos {
        if account_info.data_len() != type_len {
            return Err(SecurityError::InvalidAccountLen.into());
        }
    }

    Ok(())
}

pub fn verify_pda(
    account_info: &AccountInfo,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> ProgramResult {
    let (expected_address, _) = Pubkey::find_program_address(seeds, program_id);

    if *account_info.key != expected_address {
        return Err(SecurityError::NotExpectedAddress.into());
    }

    Ok(())
}

pub fn verify_signers(account_infos: &[&AccountInfo]) -> ProgramResult {
    for account_info in account_infos {
        if !account_info.is_signer {
            return Err(SecurityError::SignerNotRecognized.into());
        }
    }

    Ok(())
}

pub fn verify_is_executable(account_infos: &[&AccountInfo]) -> ProgramResult {
    for account_info in account_infos {
        if !account_info.executable {
            return Err(SecurityError::ExecutableAccountExpected.into());
        }
    }

    Ok(())
}

pub fn verify_signer_address(signer_address: &Pubkey, expected_address: &Pubkey) -> ProgramResult {
    if signer_address != expected_address {
        return Err(SecurityError::UnrecognizedSignerAddress.into());
    }

    Ok(())
}

///========= TESTS ==========///

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_verify_ownership_is_ok() {
        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: false,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };
        assert_ne!(Ok(()), verify_ownership(&Pubkey::new_unique(), &[&ai]));
        assert_eq!(Ok(()), verify_ownership(owner, &[&ai]));
    }

    #[test]
    fn it_test_verify_len_is_ok() {
        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: false,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };
        assert_eq!(Ok(()), verify_len(1, &[&ai]));
        assert_ne!(Ok(()), verify_len(2, &[&ai]));
    }

    #[test]
    fn it_test_verify_pda_is_ok() {
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let program_id = &Pubkey::new_unique();
        let wrong_program_id = &Pubkey::new_unique();

        let seeds = &[b"pda" as &[u8]];
        let wrong_seeds = &[b"pba" as &[u8]];

        let (received_address, _) = Pubkey::find_program_address(seeds, program_id);

        let ai: AccountInfo = AccountInfo {
            key: &received_address,
            is_signer: false,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: program_id,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };
        assert_eq!(Ok(()), verify_pda(&ai, seeds, program_id));
        assert_ne!(Ok(()), verify_pda(&ai, wrong_seeds, program_id));
        assert_ne!(Ok(()), verify_pda(&ai, seeds, wrong_program_id));
        assert_ne!(Ok(()), verify_pda(&ai, wrong_seeds, wrong_program_id));
    }

    #[test]
    fn it_test_verify_signers_is_ok() {
        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai_signer: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai_not_signer: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: false,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai_signer_not_writable: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: false,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        assert_eq!(
            Ok(()),
            verify_signers(&[&ai_signer, &ai_signer_not_writable])
        );
        assert_eq!(Ok(()), verify_signers(&[&ai_signer]));
        assert_ne!(Ok(()), verify_signers(&[&ai_signer, &ai_not_signer]));
        assert_ne!(Ok(()), verify_signers(&[&ai_not_signer]));
    }

    #[test]
    fn it_test_verify_is_executable_is_ok() {
        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai_executable: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: true,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai_not_executable: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        let owner: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai_other_executable_signer: AccountInfo = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: false,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: owner,
            executable: true,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        assert_eq!(Ok(()), verify_is_executable(&[&ai_executable]));
        assert_eq!(
            Ok(()),
            verify_is_executable(&[&ai_executable, &ai_other_executable_signer])
        );
        assert_ne!(Ok(()), verify_is_executable(&[&ai_not_executable]));
        assert_ne!(
            Ok(()),
            verify_is_executable(&[&ai_executable, &ai_not_executable])
        );
    }

    #[test]
    fn it_test_verify_signer_address_is_ok() {
        let signer: &Pubkey = &Pubkey::new_unique();
        let zero64: &mut u64 = &mut 0u64;
        let zerou8: &mut [u8; 1] = &mut ([0u8]);

        let ai: AccountInfo = AccountInfo {
            key: signer,
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(zero64)),
            owner: &Pubkey::new_unique(),
            executable: false,
            rent_epoch: 0,
            data: Rc::new(RefCell::new(zerou8)),
        };

        let not_expected_address: &Pubkey = &Pubkey::new_unique();

        assert_eq!(Ok(()), verify_signer_address(&ai.key, signer));
        assert_ne!(Ok(()), verify_signer_address(&ai.key, not_expected_address));
    }
}
