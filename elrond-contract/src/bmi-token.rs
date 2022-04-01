#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait BmiToken {
    #[init]
    fn init(&self, amount: BigUint) {
        self.token_total_supply()
            .update(|total| *total += amount.clone());
    }

    #[only_owner]
    #[endpoint(mintTokens)]
    #[payable("*")]
    fn mint_tokens(&self,
                   #[payment_amount] amount: BigUint
    ) {
        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += amount.clone());
    }

    #[view(getTokenTotalSupply)]
    #[storage_mapper("tokenTotalSupply")]
    fn token_total_supply(&self) -> SingleValueMapper<BigUint>;


    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
