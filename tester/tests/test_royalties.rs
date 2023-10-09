use std::env;

use scrypto_unit::*;
use transaction::prelude::*;

use test_utils::{random_component_deploy_dir, random_component_process};

mod common;


#[test]
fn royalties_not_set() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (rc_component, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    // 1. Request mint - should return callback id: 1
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(2), royalties, "When unset, total royalties should be 2 XRD");
}

#[test]
fn some_royalties() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (rc_component, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    // 1. Set the royalties
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                rc_component,
                "update_caller_royalties",
                manifest_args!(example_component, 3u8),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    // 2. Request mint - should charge additional royalties
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(0.5), royalties, "Total royalties should be 0.5 XRD");
}

#[test]
fn update_royalties() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (rc_component, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    // 1. Set the royalties
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                rc_component,
                "update_caller_royalties",
                manifest_args!(example_component, 2u8),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    // 2. Update the royalties
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                rc_component,
                "update_caller_royalties",
                manifest_args!(example_component, 30u8),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    // 3. Request mint - should charge additional royalties
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(5), royalties, "Total royalties should be 5 XRD");
}

