use anchor_lang::prelude::*;
use crate::state::task::*;
use crate::error::DroneforceError;
use crate::helpers;

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

pub fn handler(
    ctx: Context<CreateTask>,
    task_id: String,
    location_lat: f64,
    location_lng: f64,
    area_size: u32,
    task_type: u8,
    altitude: u16,
    geofencing_enabled: bool,
    description: String,
    validator_pubkey: Pubkey,
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
    task.validator_pubkey = validator_pubkey;
    task.verification_result = false;
    task.verification_report_hash = [0u8; 32];
    
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

// Event
#[event]
pub struct TaskCreatedEvent {
    pub task_id: String,
    pub creator: Pubkey,
    pub timestamp: i64,
}
