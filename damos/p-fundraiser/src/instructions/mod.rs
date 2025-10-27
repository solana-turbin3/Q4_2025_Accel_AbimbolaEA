use pinocchio::program_error::ProgramError;

pub mod check_contributions;
pub mod contribute;
pub mod initialize;
pub mod refund;

pub use check_contributions::*;
pub use contribute::*;
pub use initialize::*;
pub use refund::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FundraiserInstructions {
    Initialize = 0,
    Contribute = 1,
    CheckContributions = 2,
    Refund = 3
}

impl TryFrom<u8> for FundraiserInstructions {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Contribute),
            2 => Ok(Self::CheckContributions),
            3 => Ok(Self::Refund),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}