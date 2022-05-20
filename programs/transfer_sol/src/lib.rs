use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_native_sol(ctx: Context<TransferNativeSol>, amount_of_lamports: u64) -> Result<()> {

        let company_amount = (amount_of_lamports * 4)/100;

        let creator_amount = amount_of_lamports - company_amount;

        let first_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            creator_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &first_ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        );

        let second_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.company.key(),
            company_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &second_ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.company.to_account_info(),
            ],
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferNativeSol<'info> {
    #[account(mut)]
    /// CHECK:
    from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    to: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    company: AccountInfo<'info>,
    user: Signer<'info>,
    system_program: Program<'info, System>
}
