use solana_program::program_error::ProgramError;
use thiserror::Error;

const CODIGO_BASE_ERROR: isize = 0xcfc0d1900000000;

#[derive(Error, Debug, Copy, Clone)]
pub enum SecurityError {
    #[error("Not The Expected Account Address")]
    NotExpectedAddress = CODIGO_BASE_ERROR,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Signer Not Recognized")]
    SignerNotRecognized,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

    #[error("Unrecognized Signer Address")]
    UnrecognizedSignerAddress,
}

impl From<SecurityError> for ProgramError {
    fn from(e: SecurityError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
