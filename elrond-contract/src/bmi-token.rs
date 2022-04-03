#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
/// 626d69746f6b656e20424d492036 -> name: bmitoken, ticket: BMI, decimals: 6
#[elrond_wasm::contract]
pub trait BmiToken {
    #[init]
    fn init(&self) {
    }

    #[only_owner]
    #[endpoint(mintTokens)]
    #[payable("*")]
    fn mint_tokens(&self,
                   tokens_mint: BigUint
    ) {
        let token_identifier = &self.token_identifier().get();
        self.token_total_supply()
            .update(|total| *total += tokens_mint.clone());
        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += tokens_mint.clone());
        self.send().esdt_local_mint(token_identifier, 0, &tokens_mint);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(
            !self.token_identifier().is_empty(),
            "Must issue token first"
        );

        let roles = [EsdtLocalRole::Mint, EsdtLocalRole::Burn];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.token_identifier().get(),
                roles[..].iter().cloned(),
            )
            .async_call()
            .call_and_exit()
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(issueTokens)]
    fn issue_tokens(&self, token_display_name: ManagedBuffer, token_ticket: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        let caller = self.blockchain().get_caller();
        let initial_supply = BigUint::from(1u32);
        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticket,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 6,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();

        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.token_identifier().set(&token_identifier);
            },
            ManagedAsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, &message.err_msg);

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

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