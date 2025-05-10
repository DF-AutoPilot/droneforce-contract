use anchor_lang::prelude::*;

#[error_code]
pub enum DroneforceError {
    #[msg("Description is too long")]
    DescriptionTooLong,
    
    #[msg("Arweave transaction ID is too long")]
    ArweaveTxIdTooLong,
    
    #[msg("Invalid task status for this operation")]
    InvalidTaskStatus,
    
    #[msg("Unauthorized operator")]
    UnauthorizedOperator,
    
    #[msg("Invalid latitude value")]
    InvalidLatitude,
    
    #[msg("Invalid longitude value")]
    InvalidLongitude,
    
    #[msg("Unauthorized validator")]
    UnauthorizedValidator,
    
    #[msg("Task has already been verified")]
    AlreadyVerified,
}
