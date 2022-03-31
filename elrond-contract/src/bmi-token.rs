#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait BmiToken {
    #[init]
    fn init(&self, token_identifier: TokenIdentifier) {
        require!(
            token_identifier.is_valid_esdt_identifier(),
            "Invalid token provided"
        );
        self.token_identifier().set(&token_identifier);
    }

    #[only_owner]
    #[endpoint(mintTokens)]
    #[payable("*")]
    fn mint_tokens(&self,
      token_identifier: &TokenIdentifier,
      #[payment_amount] amount: BigUint
    ) {
        self.send().esdt_local_mint(token_identifier, 0, &amount);
        self.token_total_supply()
            .update(|total| *total += amount.clone());
        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += amount.clone());
    }

    #[view(getTokenTotalSupply)]
    #[storage_mapper("tokenTotalSupply")]
    fn token_total_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getTokenIdentifier)]
    #[storage_mapper("tokenIdentifier")]
    fn token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
