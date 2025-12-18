#This is the steps to follow to make sure youre in the right path my boss has assigned me this tasks this is the description he gave me and im only working on the contracts only and make sure it runs and deploy successfully 
 so we are to create contracts for this with comments on each line for full understanding 

## 🔹 Overview
This is a **Solana-based Prediction Market Smart Contract** built using the **Anchor framework**. It enables users to predict outcomes of real-world events by trading "Yes" and "No" tokens.

---

## 🚀 Features
✅ **Decentralized Prediction Market** – Users can create their own market.

✅ **Deposite liquidity** – Users can deposite sol before start betting and once the liqudity amount reaches special amount they can start betting "Yes" and "No".

✅ **Betting** – Trade "Yes" and "No" tokens based on expected outcomes. Users can purchase "Yes" or "No" tokens based on their predictions, with token prices fluctuating dynamically according to probability. This probability is determined by the total number of tokens sold to users, ensuring a market-driven pricing mechanism.

✅ **Automated Settlement** – Resolves markets based on real-world data  

✅ **Switchboard Oracle Integration** – Fetches external data for outcome validation  



  DecentraPredict - A Decentralized Prediction Market

**A fully decentralized prediction market platform on Solana enabling users to create custom markets, add liquidity, place token-based bets, and automatically resolve outcomes using Switchboard oracles and Anchor smart contracts.**

DecentraPredict is an open-source decentralized prediction market built on Solana, allowing users to create, participate, add liquidity, and resolve prediction events using smart contracts.

>  Bet on real-world outcomes. Earn if you're right. Built for transparency, fairness, and community governance.

---

## Features

- **Create Custom Markets** – Users can create prediction markets with custom questions and outcomes
- **Add Liquidity** – Fund markets to increase liquidity and enable betting
- **Token-Based Betting** – Place bets on "Yes" or "No" outcomes using dynamic token pricing
- **Decentralized Smart Contracts** – Trustless and transparent market resolution on Solana
- **Oracle Integration** – Automatic result fetching using Switchboard oracles
- **Referral System** – Earn rewards through referral links
- **User Profiles** – Track your betting history and market participation
- **Real-time Market Data** – View active, pending, and resolved markets

---

# JobMarketZK – v0 Market Rules (Draft)

## Goal:  
Provide a simple, understandable rule set so the frontend, contracts and future contributors all agree on what a “market” is — without locking us into final tokenomics or legal structure.

This version is for **testnet / demo only**.

---

## 1. Market Types (v0)

For the first demo we only support **binary (YES/NO) markets** around jobs & labour signals.

Examples:

- “Will **Company X** announce layoffs of **> 5%** staff by **31 Dec 2025**?”
- “Will **Remote Developer Salaries** in **SF** fall below **$150k median** by **Q2 2026**?”
- “Will the **US unemployment rate** exceed **5.0%** in **Q1 2026**?”

No scalar markets, no multi outcome markets in v0.

---

## 2. Market Fields

Each market has the following core fields:

- `id` – unique identifier (PDA on-chain where PDA stands for Program Derived Address. No one has a private key for a PDA — it’s controlled by the program logic.)
- `title` – short readable question.
- `description` – 1–3 sentences of context.
- `category` – one of:
  - `LAYOFFS`
  - `HIRING`
  - `SALARIES`
  - `MACRO LABOUR` (macro stats like unemployment)
- `creator` – wallet address that created the market.
- `createdAt` – timestamp.
- `closeTime` – time after which trading is disabled.
- `resolveTime` – time when the result *should* be known.
- `resolutionSource` – free text reference (e.g. “US BLS”, “Company earnings report”, “Official HR announcement”).
- `status` – one of:
  - `OPEN` – trading allowed
  - `CLOSED` – trading stopped, awaiting resolution
  - `RESOLVED` – outcome fixed
  - `VOIDED` – cancelled / refunded
- `outcome` – one of:
  - `UNRESOLVED`
  - `YES`
  - `NO`
  - `VOID`

---

## 3. Positions & Payouts (Simplified)

- Users can buy **YES** or **NO** shares with a stable unit (for now on devnet this can be SOL or a test token).
- Price is determined by the underlying AMM / bonding curve from the reference contract.
- When **RESOLVED**:
  - Winning side gets **1 unit of payout per share** (minus protocol fee).
  - Losing side gets **0**.
- When **VOIDED**:
  - All buyers receive a refund of their original stake (minus transaction fees only).

We are **not** locking in permanent fee structure here — this is demo only.

---

## 4. Who Can Create Markets? (v0)

For demo/testnet:

- **Anyone** with a wallet can create a market - Only approved platform operators may publish a market to the blockchain.
Users may submit market ideas off-chain, but they cannot deploy markets directly.
- A **small fixed fee** (or minimum stake) *may* be required to avoid spam (implementation TBD).
- Markets must:
  - Have a clear, objective question.
  - Have a specific **closeTime** and **resolveTime**.
  - Name at least one **resolutionSource**.

Later this can be restricted to:
- curator set
- DAO approved creators
- partners, sponsors etc.

# create_market 
  - ⚠️ Permissioned Action — Admin Only

  - Only approved platform operators may publish a market to the blockchain.
  - Users may submit market ideas off-chain, but they cannot deploy markets directly.

  - This rule exists to ensure compliance, avoid gambling classification,
    and maintain verifiable resolution standards.


---

## 5. Who Resolves Markets? (v0)

For the first version we keep it simple:

- A designated **admin/oracle wallet** resolves markets on chain.
- That wallet sets `outcome` to `YES`, `NO`, or `VOID`.
- The frontend will show **who resolved** and **when**.

Later we can move to:
- multi-sig resolution
- ZK proofs
- oracle networks.

For testnet we just need:

- one `ORACLE_AUTHORITY` address in config/env.

---

## 6. Disputes (Placeholder)

For v0 **we do not implement an on-chain dispute process.**

- If the oracle makes a mistake, the market can be manually **VOIDED** and users refunded.
- Dispute governance will be designed later for mainnet.

---

## 7. Fees (Testnet Placeholder)

For the demo:

- Protocol may charge a **flat percentage fee** on winning payouts (e.g. 1–2%).
- A portion may be reserved for:
  - protocol treasury
  - future reward pool
  - oracle compensation

For now, treat this as:

- `protocol_fee_bps` field per market (basis points, e.g. 100 = 1%).

Exact numbers are **not final** and can be fine tuned later.

---

## 8. ZK / Identity / $PLAY Hooks (Future Fields)

We don’t implement full ZK yet, but we already plan for the fields:

- `creatorReputation` – off-chain / ZK score mapped to wallet. Sored on AWS or similar.
- `dataProofType` – e.g. `NONE`, `EMPLOYMENT_ZK`, `SALARY_ZK`.
- `playQuestId` – if the market is part of a Play<DEX> quest.
- `xpReward` – XP or points for correct participation.

For v0 these can be **optional / unused**, but we reserve them in the design so contracts and UI can evolve without a full rewrite.

---

## 9. Testnet Only / Legal Disclaimer

- This version is for **testnet / demo only**.
- No real money, prize, or financial return is guaranteed.
- Markets are experimental and may be reset, voided or redeployed at any time.

A short disclaimer should be shown in the UI banner:

> “JobMarketZK is currently running on testnet. This is an experimental demo, not a live financial product.”

Made by Life Bricks Global 
X/Twitter: CBNSMART

--



## JobMarketZK — Smart Contract IDL Preview (MVP)

    ⚠️ This is a preview only.
    Not final, not audited, not deployed.
    Used to guide UI → backend → smart contract alignment.

# Overview

The JobMarketZK protocol exposes a small, controlled instruction surface for an MVP:
- Users cannot create markets on-chain
- Users can place predictions and claim payouts
- Only authorized admins may create or resolve markets

This design aligns with required compliance patterns used by other platforms.

=> Users may submit market ideas off-chain, which admins can review and approve before publishing on-chain.

# Accounts / Entities

| Account          | Description                                                               |
| ---------------- | ------------------------------------------------------------------------- |
| `Market`         | PDA storing metadata + lifecycle state (Open / Closed / Resolved)         |
| `UserPosition`   | Tracks YES/NO share balances per user per market                          |
| `TreasuryVault`  | PDA escrow holding market liquidity                                       |
| `AdminAuthority` | Hardcoded or multisig signer who controls market publication & resolution |


# Instruction Set

{
  "version": "0.1.0",
  "name": "JobMarketZK",
  "instructions": [
    {
      "name": "createMarket",
      "description": "Admin-controlled deployment of a new market.",
      "permission": "⚠️ Admin Only",
      "details": "This action may ONLY be performed by the AdminAuthority. Users may only submit proposals off-chain.",
      "accounts": [
        { "name": "admin", "isSigner": true },
        { "name": "market", "isMut": true },
        { "name": "treasuryVault", "isMut": true },
        { "name": "systemProgram" }
      ],
      "args": [
        { "name": "title", "type": "string" },
        { "name": "category", "type": "string" },
        { "name": "endTime", "type": "i64" },
        { "name": "resolutionSource", "type": "string" }
      ]
    },
    {
      "name": "placeBet",
      "description": "User buys YES or NO shares.",
      "accounts": [
        { "name": "user", "isSigner": true },
        { "name": "market", "isMut": true },
        { "name": "userPosition", "isMut": true },
        { "name": "treasuryVault", "isMut": true },
        { "name": "systemProgram" }
      ],
      "args": [
        { "name": "side", "type": { "enum": ["YES", "NO"] }},
        { "name": "amount", "type": "u64" }
      ]
    },
    {
      "name": "closeMarket",
      "permission": "⚠️ Admin Only",
      "description": "Locks the market so no further trades can be made before resolution.",
      "accounts": [
        { "name": "admin", "isSigner": true },
        { "name": "market", "isMut": true }
      ]
    },
    {
      "name": "resolveMarket",
      "permission": "⚠️ Admin Only",
      "description": "Finalizes outcome based on trusted resolution source.",
      "accounts": [
        { "name": "admin", "isSigner": true },
        { "name": "market", "isMut": true },
        { "name": "treasuryVault", "isMut": true }
      ],
      "args": [
        { "name": "outcome", "type": { "enum": ["YES", "NO"] }}
      ]
    },
    {
      "name": "claimPayout",
      "description": "Users withdraw winnings after a resolved market.",
      "accounts": [
        { "name": "user", "isSigner": true },
        { "name": "market", "isMut": true },
        { "name": "userPosition", "isMut": true },
        { "name": "treasuryVault", "isMut": true }
      ]
    }
  ]
}



# Prediction Market Summary:

| Action                 | User                | Admin  |
| ---------------------- | ------------------- | -----  |
| Submit market idea     | ✅ (off-chain only) | —      |
| Create market on-chain | ❌                  | ✅     |
| Place prediction       | ✅                  | ✅     |
| Close market           | ❌                  | ✅     |
| Resolve & finalize     | ❌                  | ✅     |
| Claim payout           | ✅                  | ✅     |

# Roadmap Notes

| Later Upgrade                                    | Status     |
| ------------------------------------------------ | ---------- |
| DAO/Multisig governance for admin role           | 🚧 planned |
| Zero-Knowledge reputation scoring                | 🚧 planned |
| Multiple liquidity curves (CFMM, LMSR)           | 🚧 planned |
| User-generated markets pending approval workflow | 🚧 planned |

# Status
    Draft complete — approved for frontend integration.
    Next step: validation layer, event listeners, and testnet mocks.


    -------------



    so this is what is in the scafold he did on github but i dont want to use scaffold for the contracts i want us to do it in this new project we are only working on the smart contracts so everything which is smart contracts needs to be done and deployed on devent so we test together so u can do that 









    // use anchor_lang::prelude::*;
// use anchor_lang::solana_program::system_instruction;
// use anchor_lang::solana_program::program::invoke_signed;
// use std::mem::size_of;

// declare_id!("7xLwbdZU5iKGPns2nruxD98DwVbU2A27YebgsBXB6k3e");

// #[program]
// pub mod my_anchor_project {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>, admin: Pubkey, protocol_fee_bps: u16) -> Result<()> {
//         let config = &mut ctx.accounts.config;
//         config.admin = admin;
//         config.protocol_fee_bps = protocol_fee_bps;
//         Ok(())
//     }

//     pub fn create_market(
//         ctx: Context<CreateMarket>,
//         title: String,
//         category: String,
//         close_time: i64,
//         resolution_source: String,
//     ) -> Result<()> {
//         let market = &mut ctx.accounts.market;
//         let clock = Clock::get()?;

//         market.title = title;
//         market.category = category;
//         market.creator = *ctx.accounts.admin.key;
//         market.created_at = clock.unix_timestamp;
//         market.close_time = close_time;
//         market.resolve_time = 0;
//         market.resolution_source = resolution_source;
//         market.status = MarketStatus::Open as u8;
//         market.outcome = Outcome::Unresolved as u8;
//         market.yes_pool = 0;
//         market.no_pool = 0;
//         market.protocol_fee_bps = ctx.accounts.config.protocol_fee_bps;
//         market.treasury_bump = ctx.bumps.treasury_vault;

//         Ok(())
//     }

//     pub fn place_bet(ctx: Context<PlaceBet>, side: u8, amount: u64) -> Result<()> {
//         let market = &mut ctx.accounts.market;
//         require!(market.status == MarketStatus::Open as u8, MarketError::MarketNotOpen);
//         require!(amount > 0, MarketError::InvalidAmount);

//         let ix = system_instruction::transfer(
//             &ctx.accounts.user.key(),
//             &ctx.accounts.treasury_vault.key(),
//             amount,
//         );
//         anchor_lang::solana_program::program::invoke(
//             &ix,
//             &[
//                 ctx.accounts.user.to_account_info(),
//                 ctx.accounts.treasury_vault.to_account_info(),
//                 ctx.accounts.system_program.to_account_info(),
//             ],
//         )?;

//         let user_pos = &mut ctx.accounts.user_position;
//         match side {
//             0 => {
//                 market.yes_pool = market.yes_pool.checked_add(amount).ok_or(MarketError::Overflow)?;
//                 user_pos.yes_balance = user_pos.yes_balance.checked_add(amount).ok_or(MarketError::Overflow)?;
//             }
//             1 => {
//                 market.no_pool = market.no_pool.checked_add(amount).ok_or(MarketError::Overflow)?;
//                 user_pos.no_balance = user_pos.no_balance.checked_add(amount).ok_or(MarketError::Overflow)?;
//             }
//             _ => return err!(MarketError::InvalidAmount),
//         }

//         user_pos.market = market.key();
//         user_pos.owner = ctx.accounts.user.key();

//         Ok(())
//     }

//     pub fn close_market(ctx: Context<CloseMarket>) -> Result<()> {
//         let market = &mut ctx.accounts.market;
//         market.status = MarketStatus::Closed as u8;
//         Ok(())
//     }

//     pub fn resolve_market(ctx: Context<ResolveMarket>, outcome: u8) -> Result<()> {
//         let market = &mut ctx.accounts.market;
//         require!(market.status == MarketStatus::Closed as u8, MarketError::InvalidStatus);
//         market.outcome = outcome;
//         market.status = MarketStatus::Resolved as u8;
//         market.resolve_time = Clock::get()?.unix_timestamp;
//         Ok(())
//     }

//     pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
//         let market = &mut ctx.accounts.market;
//         require!(market.status == MarketStatus::Resolved as u8, MarketError::MarketNotResolved);

//         let user_pos = &mut ctx.accounts.user_position;
//         let total_pool = market.yes_pool.checked_add(market.no_pool).ok_or(MarketError::Overflow)?;
//         let fee_amount = (total_pool as u128)
//             .checked_mul(market.protocol_fee_bps as u128)
//             .ok_or(MarketError::Overflow)?
//             .checked_div(10000u128)
//             .ok_or(MarketError::Overflow)? as u64;
//         let payout_pool = total_pool.checked_sub(fee_amount).ok_or(MarketError::Overflow)?;

//         let (winning_shares, user_shares) = match Outcome::from_u8(market.outcome)? {
//             Outcome::Yes => (market.yes_pool, user_pos.yes_balance),
//             Outcome::No => (market.no_pool, user_pos.no_balance),
//             Outcome::Void | Outcome::Unresolved => return err!(MarketError::InvalidOutcome),
//         };

//         require!(winning_shares > 0, MarketError::NoWinners);
//         require!(user_shares > 0, MarketError::NoUserShares);

//         let payout = (user_shares as u128)
//             .checked_mul(payout_pool as u128)
//             .ok_or(MarketError::Overflow)?
//             .checked_div(winning_shares as u128)
//             .ok_or(MarketError::Overflow)? as u64;

//         user_pos.yes_balance = 0;
//         user_pos.no_balance = 0;
//         user_pos.claimed = true;

//         // Fixed seed construction
//         let market_key = market.key();
//         let seeds = &[
//             b"treasury".as_ref(),
//             market_key.as_ref(),
//             &[market.treasury_bump],
//         ];
//         let signer_seeds = &[&seeds[..]];

//         let ix = system_instruction::transfer(
//             &ctx.accounts.treasury_vault.key(),
//             &ctx.accounts.user.key(),
//             payout,
//         );
//         invoke_signed(
//             &ix,
//             &[
//                 ctx.accounts.treasury_vault.to_account_info(),
//                 ctx.accounts.user.to_account_info(),
//                 ctx.accounts.system_program.to_account_info(),
//             ],
//             signer_seeds,
//         )?;

//         Ok(())
//     }

//     pub fn void_market(ctx: Context<VoidMarket>) -> Result<()> {
//         let market = &mut ctx.accounts.market;
//         market.status = MarketStatus::Voided as u8;
//         market.outcome = Outcome::Void as u8;
//         Ok(())
//     }
// }

// // Account structs remain unchanged from previous version (all /// CHECK: comments included)
// #[derive(Accounts)]
// #[instruction(admin: Pubkey, protocol_fee_bps: u16)]
// pub struct Initialize<'info> {
//     #[account(init, payer = payer, space = 8 + size_of::<Config>())]
//     pub config: Account<'info, Config>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// #[instruction(title: String)]
// pub struct CreateMarket<'info> {
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     #[account(constraint = config.admin == admin.key() @ MarketError::Unauthorized)]
//     pub config: Account<'info, Config>,
//     #[account(
//         init,
//         payer = admin,
//         space = 8 + Market::MAX_SIZE,
//         seeds = [b"market", admin.key().as_ref(), title.as_bytes()],
//         bump
//     )]
//     pub market: Account<'info, Market>,
//     /// CHECK: Validated by seeds and bump; owned by system program
//     #[account(
//         init,
//         payer = admin,
//         space = 8,
//         seeds = [b"treasury", market.key().as_ref()],
//         bump
//     )]
//     pub treasury_vault: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct PlaceBet<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(mut)]
//     pub market: Account<'info, Market>,
//     #[account(
//         init,
//         payer = user,
//         space = 8 + UserPosition::MAX_SIZE,
//         seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
//         bump
//     )]
//     pub user_position: Account<'info, UserPosition>,
//     /// CHECK: Validated by seeds and bump stored in market
//     #[account(
//         mut,
//         seeds = [b"treasury", market.key().as_ref()],
//         bump = market.treasury_bump
//     )]
//     pub treasury_vault: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct CloseMarket<'info> {
//     pub admin: Signer<'info>,
//     #[account(mut, constraint = market.creator == admin.key() @ MarketError::Unauthorized)]
//     pub market: Account<'info, Market>,
// }

// #[derive(Accounts)]
// pub struct ResolveMarket<'info> {
//     pub admin: Signer<'info>,
//     #[account(mut, constraint = market.creator == admin.key() @ MarketError::Unauthorized)]
//     pub market: Account<'info, Market>,
//     /// CHECK: Validated by seeds and bump
//     #[account(
//         mut,
//         seeds = [b"treasury", market.key().as_ref()],
//         bump = market.treasury_bump
//     )]
//     pub treasury_vault: UncheckedAccount<'info>,
// }

// #[derive(Accounts)]
// pub struct ClaimPayout<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(mut)]
//     pub market: Account<'info, Market>,
//     #[account(
//         mut,
//         seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
//         bump
//     )]
//     pub user_position: Account<'info, UserPosition>,
//     /// CHECK: Validated by seeds and signed with bump
//     #[account(
//         mut,
//         seeds = [b"treasury", market.key().as_ref()],
//         bump = market.treasury_bump
//     )]
//     pub treasury_vault: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct VoidMarket<'info> {
//     pub admin: Signer<'info>,
//     #[account(mut, constraint = market.creator == admin.key() @ MarketError::Unauthorized)]
//     pub market: Account<'info, Market>,
// }

// // Data structures and enums (unchanged)
// #[account]
// pub struct Config {
//     pub admin: Pubkey,
//     pub protocol_fee_bps: u16,
// }

// #[account]
// pub struct Market {
//     pub title: String,
//     pub category: String,
//     pub creator: Pubkey,
//     pub created_at: i64,
//     pub close_time: i64,
//     pub resolve_time: i64,
//     pub resolution_source: String,
//     pub status: u8,
//     pub outcome: u8,
//     pub yes_pool: u64,
//     pub no_pool: u64,
//     pub protocol_fee_bps: u16,
//     pub treasury_bump: u8,
// }

// impl Market {
//     pub const MAX_TITLE_LEN: usize = 128;
//     pub const MAX_CATEGORY_LEN: usize = 32;
//     pub const MAX_SOURCE_LEN: usize = 256;
//     pub const MAX_SIZE: usize =
//         4 + Self::MAX_TITLE_LEN +
//         4 + Self::MAX_CATEGORY_LEN +
//         32 + 8 + 8 + 8 +
//         4 + Self::MAX_SOURCE_LEN +
//         1 + 1 + 8 + 8 + 2 + 1;
// }

// #[account]
// pub struct UserPosition {
//     pub market: Pubkey,
//     pub owner: Pubkey,
//     pub yes_balance: u64,
//     pub no_balance: u64,
//     pub claimed: bool,
// }

// impl UserPosition {
//     pub const MAX_SIZE: usize = 32 + 32 + 8 + 8 + 1;
// }

// #[repr(u8)]
// pub enum MarketStatus {
//     Open = 0,
//     Closed = 1,
//     Resolved = 2,
//     Voided = 3,
// }

// #[repr(u8)]
// pub enum Outcome {
//     Unresolved = 0,
//     Yes = 1,
//     No = 2,
//     Void = 3,
// }

// impl Outcome {
//     pub fn from_u8(v: u8) -> Result<Self> {
//         match v {
//             0 => Ok(Outcome::Unresolved),
//             1 => Ok(Outcome::Yes),
//             2 => Ok(Outcome::No),
//             3 => Ok(Outcome::Void),
//             _ => err!(MarketError::InvalidOutcome),
//         }
//     }
// }

// #[error_code]
// pub enum MarketError {
//     #[msg("Unauthorized action")]
//     Unauthorized,
//     #[msg("Market not open for betting")]
//     MarketNotOpen,
//     #[msg("Invalid amount")]
//     InvalidAmount,
//     #[msg("Overflow")]
//     Overflow,
//     #[msg("Invalid status for this action")]
//     InvalidStatus,
//     #[msg("Market not resolved")]
//     MarketNotResolved,
//     #[msg("Invalid outcome")]
//     InvalidOutcome,
//     #[msg("No winners")]
//     NoWinners,
//     #[msg("User has no shares")]
//     NoUserShares,
//     #[msg("Market not found")]
//     MarketNotFound,
// }

