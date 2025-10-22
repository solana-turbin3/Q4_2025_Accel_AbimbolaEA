pub mod contribute;
pub mod fundraise;

pub use contribute::*;
pub use fundraise::*;

pub enum FundraisingInstructions {
    Contribute = 0,
    CheckContributions = 1,
    Fundraise = 2
}

impl TryFrom<&u8> for FundraisingInstructions {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FundraisingInstructions::Contribute),
            1 => Ok(FundraisingInstructions::CheckContributions),
            2 => Ok(FundraisingInstructions::Fundraise),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}