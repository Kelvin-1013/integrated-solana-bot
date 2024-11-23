use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::instruction::{AccountMeta, Instruction};
use crate::state::SwapState;

pub fn _raydium_swap<'info>(
    ctx: &Context<'_, '_, '_, 'info, RaydiumSwap<'info>>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    let ix = raydium_contract::instruction::swap(
        ctx.accounts.amm_program.key,
        ctx.accounts.amm_authority.key,
        ctx.accounts.amm_open_orders.key,
        ctx.accounts.amm_target_orders.key,
        ctx.accounts.pool_coin_token_account.key,
        ctx.accounts.pool_pc_token_account.key,
        ctx.accounts.serum_program.key,
        ctx.accounts.serum_market.key,
        ctx.accounts.serum_bids.key,
        ctx.accounts.serum_asks.key,
        ctx.accounts.serum_event_queue.key,
        ctx.accounts.serum_coin_vault_account.key,
        ctx.accounts.serum_pc_vault_account.key,
        ctx.accounts.serum_vault_signer.key,
        ctx.accounts.user_source_token_account.key,
        ctx.accounts.user_destination_token_account.key,
        ctx.accounts.user_source_owner.key,
        amount_in,
        minimum_amount_out,
    )?;

    solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.amm_program.to_account_info(),
            ctx.accounts.amm_authority.to_account_info(),
            ctx.accounts.amm_open_orders.to_account_info(),
            ctx.accounts.amm_target_orders.to_account_info(),
            ctx.accounts.pool_coin_token_account.to_account_info(),
            ctx.accounts.pool_pc_token_account.to_account_info(),
            ctx.accounts.serum_program.to_account_info(),
            ctx.accounts.serum_market.to_account_info(),
            ctx.accounts.serum_bids.to_account_info(),
            ctx.accounts.serum_asks.to_account_info(),
            ctx.accounts.serum_event_queue.to_account_info(),
            ctx.accounts.serum_coin_vault_account.to_account_info(),
            ctx.accounts.serum_pc_vault_account.to_account_info(),
            ctx.accounts.serum_vault_signer.to_account_info(),
            ctx.accounts.user_source_token_account.to_account_info(),
            ctx.accounts.user_destination_token_account.to_account_info(),
            ctx.accounts.user_source_owner.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub amm_program: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_program: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_market: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub serum_vault_signer: AccountInfo<'info>,
    pub user_source_token_account: Account<'info, TokenAccount>,
    pub user_destination_token_account: Account<'info, TokenAccount>,
    pub user_source_owner: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub swap_state: Account<'info, SwapState>,
}