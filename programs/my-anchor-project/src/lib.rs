use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke_signed;
use std::mem::size_of;

declare_id!("7xLwbdZU5iKGPns2nruxD98DwVbU2A27YebgsBXB6k3e");

#[program]
pub mod my_anchor_project {
    use super::*;

    /// Initializes the program config with admin and protocol fee. Admin pays for creation.
    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey, protocol_fee_bps: u16) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = admin;
        config.protocol_fee_bps = protocol_fee_bps;
        Ok(())
    }

    /// Creates a new market. Admin pays rent for market and treasury accounts.
    pub fn create_market(
        ctx: Context<CreateMarket>,
        title: String,
        category: String,
        close_time: i64,
        resolution_source: String,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let clock = Clock::get()?;

        market.title = title;
        market.category = category;
        market.creator = *ctx.accounts.admin.key;
        market.created_at = clock.unix_timestamp;
        market.close_time = close_time;
        market.resolve_time = 0;
        market.resolution_source = resolution_source;
        market.status = MarketStatus::Open as u8;
        market.outcome = Outcome::Unresolved as u8;
        market.yes_pool = 0;
        market.no_pool = 0;
        market.protocol_fee_bps = ctx.accounts.config.protocol_fee_bps;
        market.treasury_bump = ctx.bumps.treasury_vault;

        Ok(())
    }

    /// Places a bet on YES (0) or NO (1). User pays for position account if first bet.
    pub fn place_bet(ctx: Context<PlaceBet>, side: u8, amount: u64) -> Result<()> {
        let market = &mut ctx.accounts.market;
        require!(market.status == MarketStatus::Open as u8, MarketError::MarketNotOpen);
        require!(amount > 0, MarketError::InvalidAmount);

        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.treasury_vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.treasury_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let user_pos = &mut ctx.accounts.user_position;
        match side {
            0 => {
                market.yes_pool = market.yes_pool.checked_add(amount).ok_or(MarketError::Overflow)?;
                user_pos.yes_balance = user_pos.yes_balance.checked_add(amount).ok_or(MarketError::Overflow)?;
            }
            1 => {
                market.no_pool = market.no_pool.checked_add(amount).ok_or(MarketError::Overflow)?;
                user_pos.no_balance = user_pos.no_balance.checked_add(amount).ok_or(MarketError::Overflow)?;
            }
            _ => return err!(MarketError::InvalidAmount),
        }

        user_pos.market = market.key();
        user_pos.owner = ctx.accounts.user.key();

        Ok(())
    }

    /// Closes the market to new bets.
    pub fn close_market(ctx: Context<CloseMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.status = MarketStatus::Closed as u8;
        Ok(())
    }

    /// Resolves the market with outcome (1=Yes, 2=No, 3=Void).
    pub fn resolve_market(ctx: Context<ResolveMarket>, outcome: u8) -> Result<()> {
        let market = &mut ctx.accounts.market;
        require!(market.status == MarketStatus::Closed as u8, MarketError::InvalidStatus);
        market.outcome = outcome;
        market.status = MarketStatus::Resolved as u8;
        market.resolve_time = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Claims payout for user after resolution.
    pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        require!(market.status == MarketStatus::Resolved as u8, MarketError::MarketNotResolved);

        let user_pos = &mut ctx.accounts.user_position;
        let total_pool = market.yes_pool.checked_add(market.no_pool).ok_or(MarketError::Overflow)?;
        let fee_amount = (total_pool as u128)
            .checked_mul(market.protocol_fee_bps as u128)
            .ok_or(MarketError::Overflow)?
            .checked_div(10000u128)
            .ok_or(MarketError::Overflow)? as u64;
        let payout_pool = total_pool.checked_sub(fee_amount).ok_or(MarketError::Overflow)?;

        let (winning_shares, user_shares) = match Outcome::from_u8(market.outcome)? {
            Outcome::Yes => (market.yes_pool, user_pos.yes_balance),
            Outcome::No => (market.no_pool, user_pos.no_balance),
            Outcome::Void | Outcome::Unresolved => return err!(MarketError::InvalidOutcome),
        };

        require!(winning_shares > 0, MarketError::NoWinners);
        require!(user_shares > 0, MarketError::NoUserShares);

        let payout = (user_shares as u128)
            .checked_mul(payout_pool as u128)
            .ok_or(MarketError::Overflow)?
            .checked_div(winning_shares as u128)
            .ok_or(MarketError::Overflow)? as u64;

        user_pos.yes_balance = 0;
        user_pos.no_balance = 0;
        user_pos.claimed = true;

        let market_key = market.key();
        let seeds = &[
            b"treasury".as_ref(),
            market_key.as_ref(),
            &[market.treasury_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ix = system_instruction::transfer(
            &ctx.accounts.treasury_vault.key(),
            &ctx.accounts.user.key(),
            payout,
        );
        invoke_signed(
            &ix,
            &[
                ctx.accounts.treasury_vault.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }

    /// Voids the market (for refunds, add a separate refund instruction if needed).
    pub fn void_market(ctx: Context<VoidMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.status = MarketStatus::Voided as u8;
        market.outcome = Outcome::Void as u8;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(admin: Pubkey, protocol_fee_bps: u16)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + size_of::<Config>())]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(constraint = config.admin == admin.key() @ MarketError::Unauthorized)]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = 8 + Market::MAX_SIZE,
        seeds = [b"market", admin.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,
    /// CHECK: PDA vault for holding lamports, validated by seeds; admin pays rent
    #[account(
        init,
        payer = admin,
        space = 8,
        seeds = [b"treasury", market.key().as_ref()],
        bump
    )]
    pub treasury_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(
        init,
        payer = user,
        space = 8 + UserPosition::MAX_SIZE,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,
    /// CHECK: PDA vault, validated by seeds
    #[account(
        mut,
        seeds = [b"treasury", market.key().as_ref()],
        bump = market.treasury_bump
    )]
    pub treasury_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseMarket<'info> {
    pub admin: Signer<'info>,
    #[account(mut, constraint = market.creator == admin.key() @ MarketError::Unauthorized)]
    pub market: Account<'info, Market>,
}

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    pub admin: Signer<'info>,
    #[account(mut, constraint = market.creator == admin.key() @ MarketError::Unauthorized)]
    pub market: Account<'info, Market>,
    /// CHECK: PDA vault, validated by seeds
    #[account(
        mut,
        seeds = [b"treasury", market.key().as_ref()],
        bump = market.treasury_bump
    )]
    pub treasury_vault: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ClaimPayout<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(
        mut,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,
    /// CHECK: PDA vault, validated by seeds
    #[account(
        mut,
        seeds = [b"treasury", market.key().as_ref()],
        bump = market.treasury_bump
    )]
    pub treasury_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoidMarket<'info> {
    pub admin: Signer<'info>,
    #[account(mut, constraint = market.creator == admin.key() @ MarketError::Unauthorized)]
    pub market: Account<'info, Market>,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub protocol_fee_bps: u16,
}

#[account]
pub struct Market {
    pub title: String,
    pub category: String,
    pub creator: Pubkey,
    pub created_at: i64,
    pub close_time: i64,
    pub resolve_time: i64,
    pub resolution_source: String,
    pub status: u8,
    pub outcome: u8,
    pub yes_pool: u64,
    pub no_pool: u64,
    pub protocol_fee_bps: u16,
    pub treasury_bump: u8,
}

impl Market {
    pub const MAX_TITLE_LEN: usize = 128;
    pub const MAX_CATEGORY_LEN: usize = 32;
    pub const MAX_SOURCE_LEN: usize = 256;
    pub const MAX_SIZE: usize =
        4 + Self::MAX_TITLE_LEN +
        4 + Self::MAX_CATEGORY_LEN +
        32 + 8 + 8 + 8 +
        4 + Self::MAX_SOURCE_LEN +
        1 + 1 + 8 + 8 + 2 + 1;
}

#[account]
pub struct UserPosition {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub yes_balance: u64,
    pub no_balance: u64,
    pub claimed: bool,
}

impl UserPosition {
    pub const MAX_SIZE: usize = 32 + 32 + 8 + 8 + 1;
}

#[repr(u8)]
pub enum MarketStatus {
    Open = 0,
    Closed = 1,
    Resolved = 2,
    Voided = 3,
}

#[repr(u8)]
pub enum Outcome {
    Unresolved = 0,
    Yes = 1,
    No = 2,
    Void = 3,
}

impl Outcome {
    pub fn from_u8(v: u8) -> Result<Self> {
        match v {
            0 => Ok(Outcome::Unresolved),
            1 => Ok(Outcome::Yes),
            2 => Ok(Outcome::No),
            3 => Ok(Outcome::Void),
            _ => err!(MarketError::InvalidOutcome),
        }
    }
}

#[error_code]
pub enum MarketError {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Market not open for betting")]
    MarketNotOpen,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid status for this action")]
    InvalidStatus,
    #[msg("Market not resolved")]
    MarketNotResolved,
    #[msg("Invalid outcome")]
    InvalidOutcome,
    #[msg("No winners")]
    NoWinners,
    #[msg("User has no shares")]
    NoUserShares,
    #[msg("Market not found")]
    MarketNotFound,
}