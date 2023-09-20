use std::env;

use scrypto_unit::*;
use transaction::prelude::*;
use test_utils::{random_component_process, random_component_deploy};


#[test]
fn test_request_mint_no_auth() {
    // dir is different in Debug mode
    let root_dir = env::current_dir().ok().unwrap().ends_with("dot-random");
    let dir_component = if root_dir { "./component" } else { "../component" };
    let dir_example = if root_dir { "./tester" } else { "../tester" };
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Deploy RandomComponent
    let (_, rc_component, _) = random_component_deploy(&mut test_runner, dir_component);

    // Deploy ExampleCaller
    let package_address2 = test_runner.compile_and_publish_retain_blueprints(
        dir_example,
        |blueprint, _| blueprint.eq("ExampleCallerNoAuth"),
    );
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                package_address2,
                "ExampleCallerNoAuth",
                "instantiate",
                manifest_args!(),
            )
            .build(), vec![]);

    let result = receipt.expect_commit_success();
    let example_component = result.new_component_addresses()[0];

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
