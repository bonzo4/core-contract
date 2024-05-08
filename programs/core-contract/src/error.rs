use anchor_lang::error_code;

#[error_code]
pub enum CoreContractErrors {
    #[msg("Uknown Error.")]
    UnknownError,
    #[msg("You are not Authorized to perform this action.")]
    NotAuthorized,
    #[msg("Balance too low.")]
    BalanceTooLow
}