use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("DJbDiPY8wJQRjCor1rywA4ZuwUSrybAcYzAgc9F6njox");

// Task status as an enum for better type safety
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Created,
    Accepted,
    Completed,
}

// For backward compatibility
pub mod status {
    use super::TaskStatus;
    pub const CREATED: u8 = TaskStatus::Created as u8;
    pub const ACCEPTED: u8 = TaskStatus::Accepted as u8;
    pub const COMPLETED: u8 = TaskStatus::Completed as u8;
}

// Task types
pub mod task_types {
    pub const SURVEILLANCE: u8 = 0;
    pub const DELIVERY: u8 = 1;
    pub const INSPECTION: u8 = 2;
    pub const MAPPING: u8 = 3;
    pub const PHOTOGRAPHY: u8 = 4;
}

// Helper functions module - outside the program module
mod helpers {
    use super::*;
    
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
}

#[program]
pub mod droneforce_contract {
    use super::*;

    pub fn create_task(
        ctx: Context<CreateTask>,
        task_id: String,
        location_lat: f64,
        location_lng: f64,
        area_size: u32,
        task_type: u8,
        altitude: u16,
        geofencing_enabled: bool,
        description: String,
    ) -> Result<()> {
        // Convert coordinates to fixed-point representation
        let lat_fixed = helpers::float_to_fixed(location_lat);
        let lng_fixed = helpers::float_to_fixed(location_lng);
        
        // Validate inputs first
        helpers::validate_task_input(&description, lat_fixed, lng_fixed)?;
        
        let task = &mut ctx.accounts.task;
        let bump = ctx.bumps.task;
        
        // Store task metadata
        task.location_lat = lat_fixed;
        task.location_lng = lng_fixed;
        task.area_size = area_size;
        task.task_type = task_type;
        task.altitude = altitude;
        task.geofencing_enabled = geofencing_enabled;
        task.description = description;
        task.bump = bump;
        
        // Initialize standard fields
        let timestamp = Clock::get()?.unix_timestamp;
        helpers::initialize_task_account(task, ctx.accounts.creator.key(), timestamp);
        
        emit!(TaskCreatedEvent {
            task_id: task_id.clone(),
            creator: ctx.accounts.creator.key(),
            timestamp: task.timestamp,
        });
        
        Ok(())
    }

    pub fn accept_task(ctx: Context<AcceptTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        
        // Status validation is now handled by account constraints
        
        // Update operator and status
        task.operator = ctx.accounts.operator.key();
        task.status = TaskStatus::Accepted;
        task.timestamp = Clock::get()?.unix_timestamp;
        
        emit!(TaskAcceptedEvent {
            operator: ctx.accounts.operator.key(),
            timestamp: task.timestamp,
        });
        
        Ok(())
    }



    pub fn complete_task(
        ctx: Context<CompleteTask>,
        arweave_tx_id: String,
        log_hash: [u8; 32],
        signature: [u8; 64],
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        
        // Status and operator validations are now handled by account constraints
        
        // Validate completion data
        helpers::validate_completion_data(&arweave_tx_id)?;
        
        // Update task with completion data
        task.arweave_tx_id = arweave_tx_id;
        task.log_hash = log_hash;
        task.signature = signature;
        task.status = TaskStatus::Completed;
        task.timestamp = Clock::get()?.unix_timestamp;
        
        emit!(TaskCompletedEvent {
            operator: ctx.accounts.operator.key(),
            arweave_tx_id: task.arweave_tx_id.clone(),
            timestamp: task.timestamp,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(task_id: String)]
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = creator,
        space = TaskAccount::space(),
        seeds = [b"task", task_id.as_bytes()],
        bump
    )]
    pub task: Account<'info, TaskAccount>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptTask<'info> {
    #[account(
        mut,
        constraint = task.status == TaskStatus::Created @ DroneforceError::InvalidTaskStatus
    )]
    pub task: Account<'info, TaskAccount>,
    
    pub operator: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(
        mut,
        constraint = task.status == TaskStatus::Accepted @ DroneforceError::InvalidTaskStatus,
        constraint = task.operator == operator.key() @ DroneforceError::UnauthorizedOperator
    )]
    pub task: Account<'info, TaskAccount>,
    
    pub operator: Signer<'info>,
}

#[account]
pub struct TaskAccount {
    pub creator: Pubkey,             // 32 bytes
    pub operator: Pubkey,            // 32 bytes
    pub status: TaskStatus,          // 1 byte (enum is stored as u8)
    pub arweave_tx_id: String,       // 4 + 64 bytes (max)
    pub log_hash: [u8; 32],          // 32 bytes
    pub signature: [u8; 64],         // 64 bytes
    pub timestamp: i64,              // 8 bytes
    pub location_lat: i64,           // 8 bytes (fixed-point: multiply by 10^7)
    pub location_lng: i64,           // 8 bytes (fixed-point: multiply by 10^7)
    pub area_size: u32,              // 4 bytes
    pub task_type: u8,               // 1 byte
    pub altitude: u16,               // 2 bytes
    pub geofencing_enabled: bool,    // 1 byte
    pub description: String,         // 4 + 256 bytes (max)
    pub bump: u8,                    // 1 byte (store bump for future validation)
}

impl TaskAccount {
    // Calculate space required for the account
    pub fn space() -> usize {
        8 +     // Discriminator
        32 +    // creator: Pubkey
        32 +    // operator: Pubkey
        1 +     // status: u8
        4 + 64 + // arweave_tx_id: String (max 64 chars)
        32 +    // log_hash: [u8; 32]
        64 +    // signature: [u8; 64]
        8 +     // timestamp: i64
        8 +     // location_lat: f64
        8 +     // location_lng: f64
        4 +     // area_size: u32
        1 +     // task_type: u8
        2 +     // altitude: u16
        1 +     // geofencing_enabled: bool
        4 + 256 + // description: String (max 256 chars)
        1       // bump: u8
    }
}

// Events
#[event]
pub struct TaskCreatedEvent {
    pub task_id: String,
    pub creator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TaskAcceptedEvent {
    pub operator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TaskCompletedEvent {
    pub operator: Pubkey,
    pub arweave_tx_id: String,
    pub timestamp: i64,
}

// Custom errors
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
}
