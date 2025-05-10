use anchor_lang::prelude::*;
use std::str::FromStr;
use crate::state::task::*;
use crate::error::DroneforceError;

// Convert floating point coordinates to fixed-point representation
// We multiply by 10^7 to preserve 7 decimal places of precision
// This gives ~1cm precision, which is more than enough for drone operations
pub fn float_to_fixed(value: f64) -> i64 {
    (value * 10_000_000.0) as i64
}

pub fn fixed_to_float(value: i64) -> f64 {
    (value as f64) / 10_000_000.0
}

pub fn validate_task_input(description: &str, lat: i64, lng: i64) -> Result<()> {
    // Validate description length to prevent account size issues
    require!(description.len() <= 256, DroneforceError::DescriptionTooLong);
    
    // Validate geographic coordinates
    // Valid latitude range: -90 to +90 degrees
    require!(
        lat >= -900_000_000 && lat <= 900_000_000,
        DroneforceError::InvalidLatitude
    );
    
    // Valid longitude range: -180 to +180 degrees
    require!(
        lng >= -1_800_000_000 && lng <= 1_800_000_000,
        DroneforceError::InvalidLongitude
    );
    
    Ok(())
}

pub fn initialize_task_account(task: &mut TaskAccount, creator: Pubkey, timestamp: i64) {
    // Set initial values
    task.status = TaskStatus::Created;
    task.operator = Pubkey::from_str("11111111111111111111111111111111").unwrap(); // Default to system program
    task.arweave_tx_id = String::new();
    task.log_hash = [0; 32];
    task.signature = [0; 64];
    task.timestamp = timestamp;
    task.creator = creator;
}

pub fn validate_completion_data(arweave_tx_id: &str) -> Result<()> {
    // Validate Arweave TX ID length
    require!(
        arweave_tx_id.len() <= 64,
        DroneforceError::ArweaveTxIdTooLong
    );
    
    Ok(())
}
