use std::env;

use radix_engine::vm::NoExtension;
use scrypto_test::prelude::InMemorySubstateDatabase;
use scrypto_unit::*;
use transaction::prelude::*;

use test_utils::{random_component_deploy_dir, RandomTestUtil};

pub fn deploy_component_and_caller(test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>) -> (RandomTestUtil, ComponentAddress) {
    // dir is different in Debug mode
    let root_dir = env::current_dir().ok().unwrap().ends_with("dot-random");
    let dir_royal = if root_dir { "./royalties" } else { "../royalties" };
    let dir_component = if root_dir { "./component" } else { "../component" };
    let dir_example = if root_dir { "./tester" } else { "../tester" };

    // Deploy RandomComponent
    let env = random_component_deploy_dir(test_runner, dir_royal, dir_component);

    // Deploy ExampleCaller
    let example_component = deploy_caller_no_auth(test_runner, dir_example);
    return (env, example_component);
}

pub fn deploy_caller_no_auth(test_runner: &mut TestRunner<NoExtension, InMemorySubstateDatabase>, dir_example: &str) -> ComponentAddress {
    let package_address2 = test_runner.publish_retain_blueprints(
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
    example_component
}

pub const EPSILON: Decimal = dec!(0.00000000000000001);
#[allow(dead_code)]
pub fn assert_equal(x: Decimal, y: Decimal, m: &str) {
    if !(x - y <= EPSILON && y - x <= EPSILON) {
        // basically panic with a pretty message
        assert_eq!(x, y, "{}", m);
    }
}
