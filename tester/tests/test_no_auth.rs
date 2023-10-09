use std::env;

use scrypto_unit::*;
use transaction::prelude::*;

use test_utils::{random_component_deploy_dir, random_component_process};

mod common;

#[test]
fn request_mint_no_auth() {
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
    let result = receipt.expect_commit_success();
    let out = result.outcome.expect_success();
    out[1].expect_return_value(&1u32);

    // 2. Watcher calls RandomComponent.process() to do the actual mint - should mint an NFT
    let random_seed: Vec<u8> = vec![1, 2, 3, 4, 5];
    random_component_process(&mut test_runner, rc_component, random_seed);

    // Assert
}
