use anchor_lang::prelude::*;
use crate::state::task::*;
use crate::error::DroneforceError;

#[derive(Accounts)]
pub struct AcceptTask<'info> {
    #[account(
        mut,
        constraint = task.status == TaskStatus::Created @ DroneforceError::InvalidTaskStatus
    )]
    pub task: Account<'info, TaskAccount>,
    
    pub operator: Signer<'info>,
}

pub fn handler(ctx: Context<AcceptTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    
    // Status validation is handled by account constraints
    
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

// Event
#[event]
pub struct TaskAcceptedEvent {
    pub operator: Pubkey,
    pub timestamp: i64,
}
