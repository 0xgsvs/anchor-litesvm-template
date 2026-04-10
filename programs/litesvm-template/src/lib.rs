pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
// pub use state::*;

declare_id!("H9z1rBzgEXyhWtsBHMoH8ccNXGeL1zAoRVuRBGbttSRF");

#[program]
pub mod litesvm_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
