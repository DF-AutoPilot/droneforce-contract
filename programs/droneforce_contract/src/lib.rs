use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Task status constants
pub mod status {
    pub const CREATED: u8 = 0;
    pub const ACCEPTED: u8 = 1;
    pub const COMPLETED: u8 = 2;
}

// Task types
pub mod task_types {
    pub const SURVEILLANCE: u8 = 0;
    pub const DELIVERY: u8 = 1;
    pub const INSPECTION: u8 = 2;
    pub const MAPPING: u8 = 3;
    pub const PHOTOGRAPHY: u8 = 4;
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
        let task = &mut ctx.accounts.task;
        
        // Set creator
        task.creator = ctx.accounts.creator.key();
        
        // Set initial values
        task.status = status::CREATED;
        task.operator = Pubkey::from_str("11111111111111111111111111111111").unwrap(); // Default to system program
        task.arweave_tx_id = String::new();
        task.log_hash = [0; 32];
        task.signature = [0; 64];
        task.timestamp = Clock::get()?.unix_timestamp;
        
        // Set task details
        task.location_lat = location_lat;
        task.location_lng = location_lng;
        task.area_size = area_size;
        task.task_type = task_type;
        task.altitude = altitude;
        task.geofencing_enabled = geofencing_enabled;
        task.description = description;
        
        // Validate description length to prevent account size issues
        require!(description.len() <= 1000, DroneforceError::DescriptionTooLong);
        
        emit!(TaskCreatedEvent {
            task_id: task_id.clone(),
            creator: ctx.accounts.creator.key(),
            timestamp: task.timestamp,
        });
        
        Ok(())
    }

    pub fn accept_task(ctx: Context<AcceptTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        
        // Validate current status
        require!(task.status == status::CREATED, DroneforceError::InvalidTaskStatus);
        
        // Update operator and status
        task.operator = ctx.accounts.operator.key();
        task.status = status::ACCEPTED;
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
        
        // Validate operator is the one who accepted the task
        require!(
            task.operator == ctx.accounts.operator.key(),
            DroneforceError::UnauthorizedOperator
        );
        
        // Validate current status
        require!(task.status == status::ACCEPTED, DroneforceError::InvalidTaskStatus);
        
        // Update task with completion data
        task.arweave_tx_id = arweave_tx_id;
        task.log_hash = log_hash;
        task.signature = signature;
        task.status = status::COMPLETED;
        task.timestamp = Clock::get()?.unix_timestamp;
        
        // Validate Arweave TX ID length
        require!(
            task.arweave_tx_id.len() <= 100,
            DroneforceError::ArweaveTxIdTooLong
        );
        
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
    #[account(mut)]
    pub task: Account<'info, TaskAccount>,
    
    pub operator: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub task: Account<'info, TaskAccount>,
    
    pub operator: Signer<'info>,
}

#[account]
pub struct TaskAccount {
    pub creator: Pubkey,             // 32 bytes
    pub operator: Pubkey,            // 32 bytes
    pub status: u8,                  // 1 byte
    pub arweave_tx_id: String,       // 4 + 100 bytes (max)
    pub log_hash: [u8; 32],          // 32 bytes
    pub signature: [u8; 64],         // 64 bytes
    pub timestamp: i64,              // 8 bytes
    pub location_lat: f64,           // 8 bytes
    pub location_lng: f64,           // 8 bytes
    pub area_size: u32,              // 4 bytes
    pub task_type: u8,               // 1 byte
    pub altitude: u16,               // 2 bytes
    pub geofencing_enabled: bool,    // 1 byte
    pub description: String,         // 4 + 1000 bytes (max)
}

impl TaskAccount {
    // Calculate space required for the account
    pub fn space() -> usize {
        8 +     // Discriminator
        32 +    // creator: Pubkey
        32 +    // operator: Pubkey
        1 +     // status: u8
        4 + 100 + // arweave_tx_id: String (max 100 chars)
        32 +    // log_hash: [u8; 32]
        64 +    // signature: [u8; 64]
        8 +     // timestamp: i64
        8 +     // location_lat: f64
        8 +     // location_lng: f64
        4 +     // area_size: u32
        1 +     // task_type: u8
        2 +     // altitude: u16
        1 +     // geofencing_enabled: bool
        4 + 1000  // description: String (max 1000 chars)
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
}
