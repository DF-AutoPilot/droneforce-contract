use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;
mod helpers;

use instructions::*;
use state::task::*;

declare_id!("DJbDiPY8wJQRjCor1rywA4ZuwUSrybAcYzAgc9F6njox");

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
        validator_pubkey: Pubkey,
    ) -> Result<()> {
        create_task::handler(
            ctx,
            task_id,
            location_lat,
            location_lng,
            area_size,
            task_type,
            altitude,
            geofencing_enabled,
            description,
            validator_pubkey
        )
    }

    pub fn accept_task(ctx: Context<AcceptTask>) -> Result<()> {
        accept_task::handler(ctx)
    }

    pub fn complete_task(
        ctx: Context<CompleteTask>,
        arweave_tx_id: String,
        log_hash: [u8; 32],
        signature: [u8; 64],
    ) -> Result<()> {
        complete_task::handler(ctx, arweave_tx_id, log_hash, signature)
    }
    
    pub fn record_verification(
        ctx: Context<RecordVerification>,
        task_id: String,
        verification_result: bool,
        verification_report_hash: [u8; 32],
    ) -> Result<()> {
        record_verification::handler(ctx, task_id, verification_result, verification_report_hash)
    }
}


