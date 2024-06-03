use scrypto_test::prelude::*;

mod common;

#[test]
fn request_mint_no_auth() {
    // Arrange
    let mut test_runner = LedgerSimulatorBuilder::new().build();

    let (mut random_env, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    // 1. Request mint - should return callback id: 1
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    let result = receipt.expect_commit_success();
    let out = result.outcome.expect_success();
    out[1].expect_return_value(&1u32);

    // 2. Watcher calls RandomComponent.execute() to do the actual mint - should mint an NFT
    random_env.execute_next(&mut test_runner, 1);

    // Assert
}
