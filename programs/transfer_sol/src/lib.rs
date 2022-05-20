use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn transfer_native_sol(ctx: Context<TransferNativeSol>) -> Result<()> {

        // let amount_of_lamports = 42; // could be an argument ;-)
        // let from = ctx.accounts.from.to_account_info();
        // let to = ctx.accounts.to.to_account_info();

        // // Debit from_account and credit to_account
        // **from.try_borrow_mut_lamports()? -= amount_of_lamports;
        // **to.try_borrow_mut_lamports()? += amount_of_lamports;

        let amount_of_lamports = 10000000;

        // let from  = ctx.accounts.from.to_account_info();
        // let to = ctx.accounts.to.to_account_info();

        // **from.try_borrow_mut_lamports()? -= amount_of_lamports;
        // **to.try_borrow_mut_lamports()? += amount_of_lamports;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount_of_lamports,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
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
    user: Signer<'info>,
    system_program: Program<'info, System>
}
