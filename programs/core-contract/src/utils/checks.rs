use anchor_lang::prelude::*;

pub fn is_program_owner(signer: Pubkey) -> bool {
    signer.to_string() == "9pT6i1LSxsFUd3jX8a3LfPV5A5UqS9mQdU3REPAM9Uev"
}

pub fn is_program_manager(signer: Pubkey) -> bool {
    signer.to_string() == "2Xv68eQ72VpvC5J52deTYE8Ch8LYjJC1WKBtmSgistTS"
}