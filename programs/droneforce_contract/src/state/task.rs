use anchor_lang::prelude::*;

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
    pub validator_pubkey: Pubkey,    // 32 bytes (validator assigned to verify this task)
    pub verification_result: bool,   // 1 byte (whether verification passed or failed)
    pub verification_report_hash: [u8; 32], // 32 bytes (hash of the verification report)
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
        8 +     // location_lat: i64
        8 +     // location_lng: i64
        4 +     // area_size: u32
        1 +     // task_type: u8
        2 +     // altitude: u16
        1 +     // geofencing_enabled: bool
        4 + 256 + // description: String (max 256 chars)
        1 +     // bump: u8
        32 +    // validator_pubkey: Pubkey
        1 +     // verification_result: bool
        32      // verification_report_hash: [u8; 32]
    }
}
