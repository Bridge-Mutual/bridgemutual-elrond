use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/bmi-token");

    blockchain.register_contract_builder("file:output/bmi-token.wasm", bmi_token::ContractBuilder);
    blockchain
}

#[test]
fn bmi_token_rs() {
    elrond_wasm_debug::mandos_rs("mandos/bmi-token.scen.json", world());
}
