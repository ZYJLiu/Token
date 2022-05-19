use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("DwY4TVmavvs63Znj28ccT1ryw8u3Y2Ctz88JfdY4UP7i");

 // REPLACE ADDRESS of mint by running solana address -k .keys/usdc_mint.json
 // Replace for Devnet Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr
 // Replace for Localnet 8fFnX9WSPjJEADtG5jQvQQptzfFmmjd6hrW7HjuUT8ur
 // DIAM Mint: ER7ZCAjos9LHVekkY7R6JQCC4efgRa44qFkJTomEGS5H
pub const USDC_MINT_ADDRESS: &str = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr";
  

#[program]
pub mod tokens {
    use super::*;

    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        Ok(())
    }

    pub fn create_mint(ctx: Context<CreateMint>, name: String) -> Result<()> {
        let (pda, bump) =
            Pubkey::find_program_address(&[ctx.accounts.merchant.key().as_ref()], ctx.program_id);

        let merchant = &mut ctx.accounts.merchant;
        merchant.name = name;
        merchant.mint = pda;
        merchant.bump = bump;
        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        let merchant = ctx.accounts.merchant.key();


        let seeds = &[merchant.as_ref(), &[ctx.accounts.merchant.bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint_pda.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.mint_pda.to_account_info(),
            },
            &signer,
        );
        token::mint_to(cpi_ctx, amount)?;

        // transfer USDC from the User to Program
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_usdc_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.program_usdc_token.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;


        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64,) -> Result<()> {

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.mint_pda.to_account_info(),
                from: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::burn(cpi_ctx, amount)?;

        // transfer USDC merchant.
        let (usdc_pda, usdc_bump) = Pubkey::find_program_address(&[ctx.accounts.usdc_mint.key().as_ref()], ctx.program_id);
        let usdc_mint_address = ctx.accounts.usdc_mint.key();
        let seeds = &[usdc_mint_address.as_ref(), &[usdc_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.program_usdc_token.to_account_info(),
                authority: ctx.accounts.program_usdc_token.to_account_info(),
                to: ctx.accounts.merchant_usdc_token.to_account_info(),
            },
            &signer,
        );

        let usdc_amount = amount; // TODO: Change the formula
        token::transfer(cpi_ctx, usdc_amount)?;
        Ok(())
    }

    pub fn partial_payment(ctx: Context<PartialPayment>, token_amount: u64, usdc_amount: u64) -> Result<()> {

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.mint_pda.to_account_info(),
                from: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::burn(cpi_ctx, token_amount)?;

        // transfer USDC from Program to merchant.
        let (usdc_pda, usdc_bump) = Pubkey::find_program_address(&[ctx.accounts.usdc_mint.key().as_ref()], ctx.program_id);
        let usdc_mint_address = ctx.accounts.usdc_mint.key();
        let seeds = &[usdc_mint_address.as_ref(), &[usdc_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.program_usdc_token.to_account_info(),
                authority: ctx.accounts.program_usdc_token.to_account_info(),
                to: ctx.accounts.merchant_usdc_token.to_account_info(),
            },
            &signer,
        );

        let program_usdc_amount = token_amount; // TODO: Change the formula
        token::transfer(cpi_ctx, program_usdc_amount)?;

        // transfer USDC from user to merchant
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_usdc_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.merchant_usdc_token.to_account_info(),
            }
        );

        let program_usdc_amount = token_amount; // TODO: Change the formula
        token::transfer(cpi_ctx, program_usdc_amount)?;
        Ok(())


    }
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    // Create Program USDC Token Account using PDA
    #[account(
        init,
        payer = payer,
        seeds = [mint.key().as_ref() ],
        bump,
        token::mint = mint,
        token::authority = token_account,
    )]
    pub token_account: Account<'info, TokenAccount>,

    // "USDC" Mint
    // TODO: add check
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateMint<'info> {
    #[account(
        init,
        payer = user,
        space = 100 // TODO: calculate space
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        init,
        seeds = [merchant.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = 2,
        mint::authority = mint_pda, 
        
    )]
    pub mint_pda: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}


#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account()]
    pub merchant: Account<'info, Merchant>,
    #[account(mut)]
    pub mint_pda: Account<'info, Mint>,

    // Mint Tokens here
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    // USDC from here
    #[account(mut)]
    pub user_usdc_token: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,

    // USDC to here
    #[account(
        mut,
        // TODO: add check
        // seeds = [ usdc_mint.key().as_ref() ],
        // bump = program_usdc_bag_bump,
    )]
    pub program_usdc_token: Box<Account<'info, TokenAccount>>,

    // SPL Token Program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
// #[instruction(usdc_bump: u8, jun_bump:u8)]
pub struct Burn<'info> {
    //NEED TO CHECK
    // `token::Burn.mint`
    #[account(mut)]
    pub mint_pda: Box<Account<'info, Mint>>,

    // `token::Burn.to`
    #[account(mut)]
    pub user_token: Box<Account<'info, TokenAccount>>,

    // The authority allowed to mutate the above ⬆
    pub user: Signer<'info>,

    // see `token::Transfer.from`
    #[account(
        mut,
        seeds = [USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref()],
        bump,
    )]
    pub program_usdc_token: Box<Account<'info, TokenAccount>>,

    //NEED TO CHECK
    //Require for the PDA above 
    #[account(
        address = USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,

    // see `token::Transfer.to`
    #[account(mut)]
    pub merchant_usdc_token: Box<Account<'info, TokenAccount>>,

    // SPL Token Program
    pub token_program: Program<'info, Token>,

}

#[derive(Accounts)]
// #[instruction(usdc_bump: u8, jun_bump:u8)]
pub struct PartialPayment<'info> {
    //NEED TO CHECK
    // `token::Burn.mint`
    #[account(mut)]
    pub mint_pda: Box<Account<'info, Mint>>,

    // `token::Burn.to`
    #[account(mut)]
    pub user_token: Box<Account<'info, TokenAccount>>,

     #[account(mut)]
    pub user_usdc_token: Account<'info, TokenAccount>,

    // The authority allowed to mutate the above ⬆
    pub user: Signer<'info>,

    // see `token::Transfer.from`
    #[account(
        mut,
        seeds = [USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref()],
        bump,
    )]
    pub program_usdc_token: Box<Account<'info, TokenAccount>>,

    //NEED TO CHECK
    //Require for the PDA above 
    #[account(
        address = USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,

    // see `token::Transfer.to`
    #[account(mut)]
    pub merchant_usdc_token: Box<Account<'info, TokenAccount>>,

    // SPL Token Program
    pub token_program: Program<'info, Token>,

}


#[account]
pub struct Merchant {
    pub name: String,
    pub mint: Pubkey,
    pub bump: u8,
    pub discount: u8,
    pub cash_back: u8
}