use anchor_lang::prelude::*;
use crate::state::task::*;
use crate::error::DroneforceError;

#[derive(Accounts)]
#[instruction(task_id: String)]
pub struct RecordVerification<'info> {
    #[account(
        mut,
        seeds = [b"task", task_id.as_bytes()],
        bump = task.bump,
        constraint = validator.key() == task.validator_pubkey @ DroneforceError::UnauthorizedValidator,
        constraint = task.verification_result == false @ DroneforceError::AlreadyVerified
    )]
    pub task: Account<'info, TaskAccount>,
    
    pub validator: Signer<'info>,
}

pub fn handler(
    ctx: Context<RecordVerification>,
    _task_id: String,
    verification_result: bool,
    verification_report_hash: [u8; 32],
) -> Result<()> {
    let task = &mut ctx.accounts.task;
    
    // Update task with verification data
    task.verification_result = verification_result;
    task.verification_report_hash = verification_report_hash;
    task.timestamp = Clock::get()?.unix_timestamp; // Update timestamp to when verification occurred
    
    // Emit verification event
    emit!(TaskVerifiedEvent {
        task_id: _task_id,
        validator: ctx.accounts.validator.key(),
        verification_result,
        timestamp: task.timestamp,
    });
    
    Ok(())
}

// Event
#[event]
pub struct TaskVerifiedEvent {
    pub task_id: String,
    pub validator: Pubkey,
    pub verification_result: bool,
    pub timestamp: i64,
}
