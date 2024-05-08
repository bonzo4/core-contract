use anchor_lang::prelude::*;

pub fn is_program_owner(signer: Pubkey) -> bool {
    signer.to_string() == "9pT6i1LSxsFUd3jX8a3LfPV5A5UqS9mQdU3REPAM9Uev"
}