use scrypto_test::prelude::*;

mod common;


#[test]
fn royalties_not_set() {
    // Arrange
    let mut test_runner = LedgerSimulatorBuilder::new().build();

    let (_random_env, example_component) = common::deploy_component_and_caller(&mut test_runner);

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
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(1), royalties, "When unset, total royalties should be 2 XRD");
}

#[test]
fn some_royalties() {
    // Arrange
    let mut test_runner = LedgerSimulatorBuilder::new().build();

    let (random_env, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    // 1. Set the royalties
    random_env.update_royalties(&mut test_runner, example_component, 3u8);

    // 2. Request mint - should charge additional royalties
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(1.5), royalties, "Total royalties should be 1+0.5 XRD");
}

#[test]
fn update_royalties() {
    // Arrange
    let mut test_runner = LedgerSimulatorBuilder::new().build();
    let (random_env, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    // 1. Set the royalties
    random_env.update_royalties(&mut test_runner, example_component, 2u8);

    // 2. Update the royalties
    random_env.update_royalties(&mut test_runner, example_component, 9u8);

    // 3. Request mint - should charge additional royalties
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(2.5), royalties, "Total royalties should be 1+1.5 XRD");
}

#[test]
fn initial_royalties() {
    // Arrange
    let mut test_runner = LedgerSimulatorBuilder::new().build();
    let (random_env, example_component) = common::deploy_component_and_caller(&mut test_runner);
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_method(
                random_env.component,
                "request_random",
                manifest_args!(example_component, "test", "test_on_error", 123u32, None::<u8>, 21u8),
            )
            .build(), vec![]);

    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(4.5), royalties, "Total royalties should be 1+3.5 XRD");
}
