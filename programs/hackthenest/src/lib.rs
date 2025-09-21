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


const AUTHORITY_PUBKEY: &str = &"7TrgC92RhkRhxmKTFvjaeaujGkobt2Pp2Y4UKcpG4wFw";

declare_id!("GCWPmqUDrnKuEG6qVQ556m2W6N1LkT5qGuqdT4LyebSS");

#[program]
pub mod hackthenest {
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



    pub fn withdraw_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {

        // Make it so you can't withdraw whenever;









        Ok(())
    }

    // fn pick_index(list_len: usize) -> Result<usize> {
       

    //     Ok(idx)
    // }


    pub fn do_raffle(ctx: Context<DoRaffle>) -> Result<()> {
        let raffle_account = &ctx.accounts.user;

        if raffle_account.key.to_string() == AUTHORITY_PUBKEY {
            let list_len = ctx.accounts.registry.keys.len();
            // This means that the user is the right user (the one with the cronjob)
            let clock = Clock::get()?;
            let timestamp = clock.unix_timestamp;
                 // Get cluster time
            let ts = clock.unix_timestamp;         // i64

    // Convert timestamp to positive u64 (avoid negatives)
            let seed = ts.unsigned_abs();

    // Map into range 0..list_len
            let idx = (seed % list_len as u64) as usize;

            let random_num: usize = idx as usize;


            // The amount you pay doesn't increase your chances right now. It's okay. I'll add that feature later.

            // This represents the total amount of tokens
            let winner = ctx.accounts.registry.keys[random_num];

            // winner.
            
            

            // ctx.accounts.registry.keys





            



        }


        // First, let's confirm that this is the right public Key that we want
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
pub struct DoRaffle<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // Example: whoever calls the function
    pub system_program: Program<'info, System>,
    
    

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>, // CPI usage
    

    #[account(seeds = [b"registry_seed"], bump)]
    pub registry: Account<'info, Registry>,
  
}


#[error_code]
pub enum HackError {
    #[msg("Unauthorized user")]
    Unauthorized,
}

