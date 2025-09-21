use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self,        // for CPI functions like mint_to
    Mint,        // Token mint account (for CPI)
    TokenAccount,// Token account (for CPI)
    TokenInterface, // The program itself
    MintTo,      // CPI struct for minting
    TransferChecked, // CPI struct for transfers
};
use anchor_lang::prelude::Clock;
// use solana_program::pubkey::Pubkey;

const AUTHORITY_PUBKEY: &str = &"7TrgC92RhkRhxmKTFvjaeaujGkobt2Pp2Y4UKcpG4wFw";

declare_id!("GCWPmqUDrnKuEG6qVQ556m2W6N1LkT5qGuqdT4LyebSS");

#[program]
pub mod hackthenest {
    use crate::instruction::StakeCoin;

    use super::*;

    pub fn initialize(ctx: Context<InitializeRegistry>) -> Result<()> {
        
        // Only the user should use initialize
        
        if ctx.accounts.user.key().to_string() == AUTHORITY_PUBKEY {
            // ctx
            let registry = &mut ctx.accounts.registry;


            registry.authority = ctx.accounts.user.key();

            registry.keys = Vec::new();


            
        } else {
            return Err(HackError::Unauthorized.into());
        }


        



        msg!("Program initialized: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {


        
        msg!("Created Mint: {:?}", ctx.accounts.mint.key());
        
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        // let registry = &mut ctx.accounts.registry;
        // 1️⃣ Transfer SOL to treasury PDA
        let price_per_token: u64 = 10_000_000; // 0.01 SOL in lamports
        let lamports: u64 = amount
        .checked_mul(price_per_token)
        .expect("Overflow when calculating lamports");

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.treasury.key(),
            lamports,
        );
        

        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // 2️⃣ Mint tokens to buyer
        let seeds: &[&[u8]] = &[b"mint_auth", &[ctx.bumps.mint_authority]];
        let signer_seeds: &[&[&[u8]]] = &[seeds];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token_interface::mint_to(cpi_ctx, amount)?;


        if ctx.accounts.registry.keys.contains(ctx.accounts.buyer.key) == false {
            ctx.accounts.registry.keys.push(*ctx.accounts.buyer.key)
        }

        

        Ok(())
    }

    use anchor_spl::token_interface::{mint_to, MintTo};

pub fn stake_coin(ctx: Context<StakeCoins>, amount: u64) -> Result<()> {
    let interest_rate = 0.03;
    let new_supply = ctx.accounts.mint.supply as f64 * (1.0 + interest_rate);
    msg!("New projected supply: {}", new_supply);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(), // where tokens go
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    mint_to(cpi_ctx, amount)?; // mint "amount" tokens to user's ATA

    Ok(())
}

    pub fn withdraw_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {

        // Make it so you can't withdraw whenever;









        Ok(())
    }

    // fn pick_index(list_len: usize) -> Result<usize> {
       

    //     Ok(idx)
    // }



pub fn do_raffle(ctx: Context<DoRaffle>) -> Result<()> {
    let registry = &ctx.accounts.registry;
    let clock = Clock::get()?;
    let idx = (clock.unix_timestamp.unsigned_abs() % registry.keys.len() as u64) as usize;

    let winner_pubkey = registry.keys[idx];

    // Make sure the passed winner account matches the one selected
    require!(ctx.accounts.winner.key() == winner_pubkey, ErrorCode::InvalidWinner);

    msg!("Winner is: {}", winner_pubkey);

    // TODO: send tokens or do any other logic

    Ok(())
}






}

// ================== ACCOUNT STRUCTS ==================

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>, // Example: whoever calls the function
//     pub system_program: Program<'info, System>,
//     pub registry: Account<'info, Registry>,

// }

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 * 1000,seeds = [b"registry_seed"], bump)]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub user: Signer<'info>, // the authority

    pub system_program: Program<'info, System>,
}



#[account]
pub struct Registry {
    // Example field: you can adjust as needed
    pub authority: Pubkey,
    pub keys: Vec<Pubkey>,
    // Add more fields as needed for your registry logic
}

/// ✅ Create a new mint account (must use standard `Account` type)
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority
    )]
    pub mint: Account<'info, anchor_spl::token::Mint>, // standard Account for init

    /// CHECK: PDA to act as mint authority
    #[account(seeds = [b"mint_auth"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>, // CPI usage

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>, // CPI usage

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: PDA declared as mint authority
    #[account(seeds = [b"mint_auth"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"registry_seed"], bump)]
    pub registry: Account<'info, Registry>,

    /// CHECK: Treasury PDA that holds collected SOL
    #[account(mut, seeds = [b"treasury"], bump)]
    pub treasury: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SellTokens<'info> {

     #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: PDA declared as mint authority
    #[account(seeds = [b"mint_auth"], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"registry_seed"], bump)]
    pub registry: Account<'info, Registry>,


    /// CHECK: Treasury PDA that holds collected SOL
    #[account(mut, seeds = [b"treasury"], bump)]
    pub treasury: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct StakeCoins<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>, // the token itself

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, // user’s ATA

    /// CHECK: Must be the mint authority signer
    pub mint_authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}


  #[derive(Accounts)]
pub struct DoRaffle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,       // The cronjob authority
    #[account(mut)]
    pub registry: Account<'info, Registry>, // Contains `keys: Vec<Pubkey>`
    #[account(mut)]
    /// CHECK: The winner account must be provided
    pub winner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}


#[error_code]
pub enum HackError {
    #[msg("Unauthorized user")]
    Unauthorized,
}



#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Winner")]
    InvalidWinner,
}