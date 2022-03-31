use anchor_lang::prelude::*;
use anchor_lang::{
    solana_program::{self, entrypoint::ProgramResult, instruction::Instruction},
    InstructionData,
};
use std::convert::TryInto;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod if_needed_test {

    use super::*;

    pub fn claim(ctx: Context<Claim>, should_panic: bool) -> Result<()> {
        let mut did_init = false;
        if ctx.accounts.claim_account.data_len() == 0 {
            let (address, bump) = Pubkey::find_program_address(
                &[ClaimAccount::SEED, ctx.accounts.item_mint.key().as_ref()],
                ctx.program_id,
            );
            assert!(ctx.accounts.claim_account.key().eq(&address));
            let init_claim_acct_ix = if_needed_test::instruction::InitializeClaimAccount {};
            let ix = Instruction::new_with_bytes(
                id(),
                &init_claim_acct_ix.data(),
                ctx.accounts.into_init_claim_metas().to_vec(),
            );
            solana_program::program::invoke_signed(
                &ix,
                &ctx.accounts.into_init_claim_accounts(),
                &[&[
                    ClaimAccount::SEED,
                    ctx.accounts.item_mint.key().as_ref(),
                    &[bump],
                ]],
            )?;
            did_init = true;
        }
        let mut claim_account = Account::<ClaimAccount>::try_from(&ctx.accounts.claim_account)?;
        if !did_init {
            let address = Pubkey::create_program_address(
                &[
                    ClaimAccount::SEED,
                    ctx.accounts.item_mint.key().as_ref(),
                    &[claim_account.bump],
                ],
                ctx.program_id,
            )
            .map_err(|_| error!(ErrorCode::InvalidNonce))?;
            assert!(ctx.accounts.claim_account.key().eq(&address));
        }

        //now do whatever u want with it
        claim_account.amount_claimed = 5000;

        let info = claim_account.to_account_info();
        let mut data = info.try_borrow_mut_data()?;
        let dst: &mut [u8] = &mut data;
        let mut cursor = std::io::Cursor::new(dst);
        claim_account.try_serialize(&mut cursor)?;

        Ok(())
    }

    pub fn initialize_claim_account(ctx: Context<InitializeClaimAccount>) -> Result<()> {
        ctx.accounts.claim_account.bump = *ctx.bumps.get("claim_account").unwrap();
        Ok(())
    }
}

impl<'info> Claim<'info> {
    fn into_init_claim_metas(&self) -> [AccountMeta; 4] {
        [
            AccountMeta::new(self.payer.key(), true),
            AccountMeta::new_readonly(self.item_mint.key(), false),
            AccountMeta::new(self.claim_account.key(), true),
            AccountMeta::new_readonly(self.system_program.key(), false),
        ]
    }
    fn into_init_claim_accounts(&self) -> [AccountInfo<'info>; 4] {
        [
            self.payer.to_account_info(),
            self.item_mint.to_account_info(),
            self.claim_account.to_account_info(),
            self.system_program.to_account_info(),
        ]
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("invalid nonce")]
    InvalidNonce,
}

#[derive(Accounts)]
pub struct InitializeClaimAccount<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    ///CHECK: in ix, see seeds above
    item_mint: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [ClaimAccount::SEED, item_mint.key().as_ref()],
        bump,
        space = ClaimAccount::SIZE,
        payer = payer,
    )]
    claim_account: Account<'info, ClaimAccount>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    ///CHECK: verified in ix
    #[account(mut)]
    claim_account: UncheckedAccount<'info>,
    ///CHECK: tada
    item_mint: UncheckedAccount<'info>,
    ///CHECK: tada
    claim_program: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct ClaimAccount {
    pub amount_claimed: u64,
    pub bump: u8,
}

impl ClaimAccount {
    pub const SEED: &'static [u8] = b"claim";
    pub const SIZE: usize = 8 + 8 + 1;
}

/*
   #[account(
        init_if_needed,
        seeds = [ClaimAccount::SEED, item_mint.key().as_ref()],
        bump, //can't pass bump on init. recomputes every time. option is to pass
        space = ClaimAccount::SIZE,
        payer = payer,
    )]
    claim_account: Account<'info, ClaimAccount>,
*/
//initialize(ctx.accounts.into_init_context());

// if ctx.accounts.claim_account.data_len() == 0 {
//     let (address, bump) = Pubkey::find_program_address(
//         &[ClaimAccount::SEED, ctx.accounts.item_mint.key().as_ref()],
//         ctx.program_id,
//     );
//     assert!(ctx.accounts.claim_account.key().eq(&address));
//     let mint_key = ctx.accounts.item_mint.key();
//     let signer_seeds = &[ClaimAccount::SEED, mint_key.as_ref(), &[bump]];
//     let __anchor_rent = Rent::get()?;
//     let lamports = __anchor_rent.minimum_balance(ClaimAccount::SIZE);
//     let cpi_accounts = anchor_lang::system_program::CreateAccount {
//         from: ctx.accounts.payer.to_account_info(),
//         to: ctx.accounts.claim_account.to_account_info(),
//     };
//     let cpi_context = anchor_lang::context::CpiContext::new(
//         ctx.accounts.system_program.to_account_info(),
//         cpi_accounts,
//     );
//     anchor_lang::system_program::create_account(
//         cpi_context.with_signer(&[signer_seeds]),
//         lamports,
//         ClaimAccount::SIZE.try_into().unwrap(),
//         ctx.program_id,
//     )?;
// } else {
//     //get the account
// }
// if let Some(length) = len {
//     msg!("length {}", length)
// } else {
//     msg!("no length")
// }
//let claim_bump = ctx.bumps.get("claim_account");
// if let Some(bump) = claim_bump {
//     msg!("we have a bump");
//     ctx.accounts.claim_account.bump = *bump;
// } else {
//     msg!("no bump");
// }
// if should_panic {
//     panic!();
// }
// let ix = Instruction::new_with_bytes(
//     *ctx.program_id,
//     &init_ix.data(),
//     [
//         AccountMeta::new(ctx.accounts.payer.key(), true),
//         AccountMeta::new_readonly(ctx.accounts.item_mint.key(), false),
//         AccountMeta::new(ctx.accounts.claim_account.key(), true),
//         AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
//     ]
//     .to_vec(),
// );
// solana_program::program::invoke_signed(
//     &ix,
//     &[
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.item_mint.to_account_info(),
//         ctx.accounts.claim_account.to_account_info(),
//         ctx.accounts.system_program.to_account_info(),
//     ],
//     &[signer_seeds],
// )?;
