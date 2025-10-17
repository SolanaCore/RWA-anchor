use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        mint_to, Mint, MintTo, Token, TokenAccount,
        transfer_checked as spl_transfer_checked,
        TransferChecked as SplTransferChecked,
    },
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
};

pub struct TokenInvoker {}

impl<'info>TokenInvoker {
    pub fn create_metadata_account(
        &self,
        name: &str,
        ticker: &str,
        uri: &str,
        token_metadata_program: &AccountInfo<'info>,
        payer: &AccountInfo<'info>,
        update_authority: &AccountInfo<'info>,
        mint: &AccountInfo<'info>,
        metadata: &AccountInfo<'info>,
        mint_authority: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>,
        rent: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let token_data = DataV2 {
            name: name.to_string(),
            symbol: ticker.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new_with_signer(
            token_metadata_program.clone(),
            CreateMetadataAccountsV3 {
                metadata: metadata.clone(),
                mint: mint.clone(),
                mint_authority: mint_authority.clone(),
                update_authority: update_authority.clone(),
                payer: payer.clone(),
                system_program: system_program.clone(),
                rent: rent.clone(),
            },
            signer_seeds,
        );

        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;
        Ok(())
    }

pub fn mint_token(
        &self,
        token_program: &AccountInfo<'info>,
        to: &AccountInfo<'info>,
        mint: &AccountInfo<'info>,
        mint_authority: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new_with_signer(
            token_program.clone(),
            MintTo {
                mint: mint.clone(),
                to: to.clone(),
                authority: mint_authority.clone(),
            },
            signer_seeds,
        );

        mint_to(cpi_ctx, 1_000_000_000_000_000)?;
        Ok(())
    }
}