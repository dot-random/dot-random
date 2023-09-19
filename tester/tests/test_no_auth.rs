use std::env;
use radix_engine::vm::NativeVmExtension;

// use radix_engine::types::*;
use scrypto_unit::*;
use transaction::prelude::*;


const PRE_ALLOCATED_PACKAGE: [u8; NodeId::LENGTH] = [
    13, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 0, 0, 19, 19,
];
const PRE_ALLOCATED_COMPONENT: [u8; NodeId::LENGTH] = [
    192, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 0, 0, 19, 19,
];
const PRE_ALLOCATED_BADGE: [u8; NodeId::LENGTH] = [
    93, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 0, 0, 19, 19,
];

fn deploy_random_component<E: NativeVmExtension, D: TestDatabase>(dir_component: &str, test_runner: &mut TestRunner<E, D>)
    -> (PackageAddress, ComponentAddress, ResourceAddress) {
    let rc_package = PackageAddress::new_or_panic(PRE_ALLOCATED_PACKAGE);
    test_runner
        .compile_and_publish_at_address(dir_component, rc_package);

    let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        vec![InstructionV1::CallFunction {
            package_address: DynamicPackageAddress::Static(rc_package),
            blueprint_name: "RandomComponent".to_string(),
            function_name: "instantiate_addr_badge".to_string(),
            args: manifest_args!(ManifestAddressReservation(0), ManifestAddressReservation(1)).into(),
        }],
        vec![(
                 BlueprintId::new(&rc_package, "RandomComponent"),
                 GlobalAddress::new_or_panic(PRE_ALLOCATED_COMPONENT),
             )
                 .into(),
             (
                 BlueprintId::new(&RESOURCE_PACKAGE, FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_owned()),
                 GlobalAddress::new_or_panic(PRE_ALLOCATED_BADGE),
             )
                 .into()],
        btreeset!(),
    );
    let res = receipt.expect_commit_success();
    let rc_component = res.new_component_addresses()[0];
    let rc_badge = res.new_resource_addresses()[0];

    let encoder = AddressBech32Encoder::for_simulator();
    let package_addr = encoder.encode(rc_package.as_ref());
    let component_addr = encoder.encode(rc_component.as_ref());
    let badge_addr = encoder.encode(rc_badge.as_ref());
    println!("RandomComponent:package_addr: {:?}\n", package_addr);
    println!("RandomComponent:component_addr: {:?}\n", component_addr);
    println!("RandomComponent:resource_addr: {:?}\n", badge_addr);

    return (rc_package, rc_component, rc_badge);
}

#[test]
fn test_request_mint_no_auth() {
    // dir is different in Debug mode
    let root_dir = env::current_dir().ok().unwrap().ends_with("dot-random");
    let dir_component = if root_dir { "./component" } else { "../component" };
    let dir_example = if root_dir { "./tester" } else { "../tester" };
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Deploy RandomComponent
    let (_, rc_component, _) = deploy_random_component(dir_component, &mut test_runner);

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
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                rc_component,
                "process",
                manifest_args!(random_seed),
            )
            .build(), vec![]);
    let result = receipt.expect_commit_success();
    result.outcome.expect_success();

    // Assert
}
