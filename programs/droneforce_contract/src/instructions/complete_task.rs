use anchor_lang::prelude::*;
use crate::state::task::*;
use crate::error::DroneforceError;
use crate::helpers;

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

pub fn handler(
    ctx: Context<CompleteTask>,
    arweave_tx_id: String,
    log_hash: [u8; 32],
    signature: [u8; 64],
) -> Result<()> {
    let task = &mut ctx.accounts.task;
    
    // Status and operator validations are handled by account constraints
    
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

// Event
#[event]
pub struct TaskCompletedEvent {
    pub operator: Pubkey,
    pub arweave_tx_id: String,
    pub timestamp: i64,
}
