use anchor_lang::prelude::*;
use anchor_spl:: token::TokenAccount;
use anchor_lang::Accounts;

declare_id!("CRQXfRGq3wTkjt7JkqhojPLiKLYLjHPGLebnfiiQB46T");

use state::SwapState;
use error::ErrorCode; 

pub mod error; 
pub mod state; 
pub mod ix_data;
pub mod swaps; 

pub use swaps::*; 

#[program]
pub mod tmp {
    use super::*;

    pub fn init_program(ctx: Context<InitSwapState>) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        swap_state.swap_input = 0;
        swap_state.is_valid = false;  
        swap_state.input_token = input_token;
        swap_state.current_token = input_token;
        Ok(())
    }
    
    pub fn start_swap(ctx: Context<TokenAndSwapState>, swap_input: u64) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        swap_state.start_balance = ctx.accounts.src.amount; // ! 
        swap_state.swap_input = swap_input; // ! 
        swap_state.is_valid = true;  
        Ok(())
    }

    pub fn profit_or_revert(ctx: Context<TokenAndSwapState>, path: Vec<ArbitrageStep>) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state; 
        swap_state.is_valid = false;
    
        let init_balance = swap_state.start_balance;
        let final_balance = ctx.accounts.src.amount;
        
        let total_fees: u64 = path.iter().map(|step| calculate_fees(step, init_balance)).sum();
        
        msg!("old = {:?}; new = {:?}; diff = {:?}; fees = {:?}", init_balance, final_balance, final_balance - init_balance, total_fees);
        
        require!(final_balance > init_balance + total_fees, ErrorCode::NoProfit);
    
        Ok(())
    }

    pub enum ArbitrageStep {
        Orca(u64, u64),
        Raydium(u64, u64),
        Meteora(u64, u64),
        Phoenix(u64, u64),
        Lifinity(u64, u64),
        Jupiter(u64, u64),
    }


    pub fn execute_arbitrage(ctx: Context<ExecuteArbitrage>, path: Vec<ArbitrageStep>) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        
        for step in path {
            match step {
                ArbitrageStep::Orca(amount_in, minimum_amount_out) => {
                    orca_swap(ctx.accounts.orca.clone(), amount_in, minimum_amount_out)?
                },
                ArbitrageStep::Raydium(amount_in, minimum_amount_out) => {
                    raydium_swap(ctx.accounts.raydium.clone(), amount_in, minimum_amount_out)?
                },
                ArbitrageStep::Meteora(amount_in, minimum_amount_out) => {
                    meteora_swap(ctx.accounts.meteora.clone(), amount_in, minimum_amount_out)?
                },
                ArbitrageStep::Phoenix(amount_in, minimum_amount_out) => {
                    phoenix_swap(ctx.accounts.phoenix.clone(), amount_in, minimum_amount_out)?
                },
                ArbitrageStep::Lifinity(amount_in, minimum_amount_out) => {
                    lifinity_swap(ctx.accounts.lifinity.clone(), amount_in, minimum_amount_out)?
                },
                ArbitrageStep::Jupiter(amount_in, minimum_amount_out) => {
                    jupiter_swap(ctx.accounts.jupiter.clone(), amount_in, minimum_amount_out)?
                },
            }
            
            swap_state.current_token = step.get_output_token();
        }
        
        Ok(())
    }

   
    /// Convenience API to initialize an open orders account on the Serum DEX.
    /// 
    pub fn init_open_order(ctx: Context<InitOpenOrder>) -> Result<()> {
        _init_open_order(ctx)
    }
    
    pub fn orca_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, OrcaSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        basic_pool_swap!(_orca_swap, OrcaSwap<'info>)(ctx, amount_in, minimum_amount_out)
    }
    
    pub fn raydium_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, RaydiumSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        basic_pool_swap!(_raydium_swap, RaydiumSwap<'info>)(ctx, amount_in, minimum_amount_out)
    }
    
    pub fn meteora_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, MeteoraSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        basic_pool_swap!(_meteora_swap, MeteoraSwap<'info>)(ctx, amount_in, minimum_amount_out)
    }
    
    pub fn phoenix_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, PhoenixSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        basic_pool_swap!(_phoenix_swap, PhoenixSwap<'info>)(ctx, amount_in, minimum_amount_out)
    }
    
    pub fn lifinity_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, LifinitySwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        basic_pool_swap!(_lifinity_swap, LifinitySwap<'info>)(ctx, amount_in, minimum_amount_out)
    }

    pub fn jupiter_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, JupiterSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        basic_pool_swap!(_jupiter_swap, JupiterSwap<'info>)(ctx, amount_in, minimum_amount_out)
    }


    pub fn aldrin_swap_v2<'info>(ctx: Context<'_, '_, '_, 'info, AldrinSwapV2<'info>>,  is_inverted: bool) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;

        _aldrin_swap_v2(&ctx, amount_in, is_inverted)?;

        // end swap 
        let user_dst = match is_inverted {
            true => &mut ctx.accounts.user_quote_ata,
            false => &mut ctx.accounts.user_base_ata 
        };
        let swap_state = &mut ctx.accounts.swap_state;
        end_swap(swap_state, user_dst)?;

        Ok(())
    }
    
    pub fn aldrin_swap_v1<'info>(ctx: Context<'_, '_, '_, 'info, AldrinSwapV1<'info>>,  is_inverted: bool) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;

        _aldrin_swap_v1(&ctx, amount_in, is_inverted)?;

        // end swap 
        let user_dst = match is_inverted {
            true => &mut ctx.accounts.user_quote_ata,
            false => &mut ctx.accounts.user_base_ata 
        };
        let swap_state = &mut ctx.accounts.swap_state;
        end_swap(swap_state, user_dst)?;

        Ok(())
    }
    
    pub fn serum_swap<'info>(ctx: Context<'_, '_, '_, 'info, SerumSwap<'info>>, side: Side) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;
        let is_bid = match side {
            Side::Bid => true,
            Side::Ask => false,
        };

        _serum_swap(&ctx, amount_in, side)?;
        
        // end swap 
        let user_dst = match is_bid {
            true => &mut ctx.accounts.market.coin_wallet,
            false => &mut ctx.accounts.pc_wallet,
        };
        let swap_state = &mut ctx.accounts.swap_state;
        end_swap(swap_state, user_dst)?;

        Ok(())

    }

}

#[macro_export]
macro_rules! basic_pool_swap {
    ($swap_fcn:expr, $typ:ident < $tipe:tt > ) => {{
        |ctx: Context<'_, '_, '_, 'info, $typ<$tipe>>, amount_in: u64, minimum_amount_out: u64| -> Result<()> {
            // save the amount of input swap
            let amount_in = prepare_swap(&ctx.accounts.swap_state).unwrap();

            // do swap 
            $swap_fcn(&ctx, amount_in, minimum_amount_out).unwrap();

            // update the swap output amount (to be used as input to next swap)
            let swap_state = &mut ctx.accounts.swap_state;
            let user_dst = &mut ctx.accounts.user_dst;
            end_swap(swap_state, user_dst).unwrap();

            Ok(())
        }
    }};
}

// Usage in your swap functions:


pub fn end_swap(
    swap_state: &mut Account<SwapState>,
    user_dst: &mut Account<TokenAccount>
) -> Result<()> {
    let dst_start_balance = user_dst.amount;
    user_dst.reload()?;
    let dst_end_balance = user_dst.amount;
    let swap_amount_out = dst_end_balance - dst_start_balance;
    msg!("swap amount out: {:?} for token: {:?}", swap_amount_out, swap_state.current_token);
    swap_state.swap_input = swap_amount_out;
    Ok(())
}

pub fn prepare_swap(
    swap_state: &Account<SwapState>,
) -> Result<u64> {
    require!(swap_state.is_valid, ErrorCode::InvalidState);
    let amount_in = swap_state.swap_input;
    msg!("swap amount in: {:?} for token: {:?}", amount_in, swap_state.current_token);
    Ok(amount_in)
}

pub fn calculate_fees(dex: &ArbitrageStep, amount: u64) -> u64 {
    match dex {
        ArbitrageStep::Orca(_, _) => amount * 30 / 10000, // 0.3% fee
        ArbitrageStep::Raydium(_, _) => amount * 25 / 10000, // 0.25% fee
        ArbitrageStep::Meteora(_, _) => amount * 20 / 10000, // 0.2% fee
        ArbitrageStep::Phoenix(_, _) => amount * 15 / 10000, // 0.15% fee
        ArbitrageStep::Lifinity(_, _) => amount * 35 / 10000, // 0.35% fee
        ArbitrageStep::Jupiter(_, _) => amount * 10 / 10000, // 0.1% fee
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)] // Add this line
pub struct InitSwapState<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + SwapState::LEN, // Add this line
        seeds = [b"swap_state"],
        bump
    )]
    pub swap_state: Account<'info, SwapState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct TokenAndSwapState<'info> {
    src: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"swap_state"], bump)] 
    pub swap_state: Account<'info, SwapState>,
}

#[derive(Accounts)]
pub struct ExecuteArbitrage<'info> {
    #[account(mut)]
    pub swap_state: Account<'info, SwapState>,
    pub orca: OrcaSwap<'info>,
    pub raydium: RaydiumSwap<'info>,
    pub meteora: MeteoraSwap<'info>,
    pub phoenix: PhoenixSwap<'info>,
    pub lifinity: LifinitySwap<'info>,
    pub jupiter: JupiterSwap<'info>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ArbitrageStep {
    Orca(u64, u64),
    Raydium(u64, u64),
    Meteora(u64, u64),
    Phoenix(u64, u64),
    Lifinity(u64, u64),
    Jupiter(u64, u64),
}

impl ArbitrageStep {
    fn get_output_token(&self) -> Pubkey {
        // Implement logic to return the output token for each DEX
        unimplemented!()
    }
}

