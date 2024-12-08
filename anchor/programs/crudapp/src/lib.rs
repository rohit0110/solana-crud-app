#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("HagG5yb5HwAE8PUinaDLGTWK2rYDNyj4oj6kdeE1D4WF");

#[program]
pub mod crudapp {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.title = title;
      journal_entry.message = message;
      journal_entry.owner = *ctx.accounts.owner.key;
      Ok(())
    }

    pub fn update_joural_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry;
      
      journal_entry.message = message;
      Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {
      Ok(())
    }
  
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
  owner: Pubkey,
  #[max_len(25)]
  title: String,
  #[max_len(100)]

  message: String
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
  #[account(
    init,
    payer = owner,
    seeds = [title.as_bytes(),owner.key().as_ref()],
    bump,
    space = 8 + JournalEntryState::INIT_SPACE
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  #[account(mut)]
  pub owner: Signer<'info>,
  pub system_program: Program<'info,System>
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(),owner.key().as_ref()],
    bump,
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::zero = true,
    realloc::payer = owner
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  #[account(mut)]
  pub owner: Signer<'info>,
  pub system_program: Program<'info,System>
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(),owner.key().as_ref()],
    bump,
    close = owner
  )]
  pub journal_entry: Account<'info, JournalEntryState>,
  #[account(mut)]
  pub owner: Signer<'info>,
  pub system_program: Program<'info,System>
}
