use std::env;

use radix_engine::types::*;
use scrypto_unit::*;
use transaction::prelude::*;

const PRE_ALLOCATED_PACKAGE: [u8; NodeId::LENGTH] = [
    13, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
];

#[test]
fn test_request_mint() {
    // dir is different in Debug mode
    let root_dir = env::current_dir().ok().unwrap().ends_with("dot-random");
    let dir_component = if root_dir { "./component" } else { "../component" };
    let dir_example = if root_dir { "./example" } else { "../example" };
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    // Deploy RandomComponent
    let package_address = PackageAddress::new_or_panic(PRE_ALLOCATED_PACKAGE);
    test_runner
        .compile_and_publish_at_address(dir_component, package_address);
    let receipt = test_runner.call_function(
        DynamicPackageAddress::Static(package_address),
        "RandomComponent",
        "instantiate",
        manifest_args!(),
    );
    let res = receipt.expect_commit_success();
    let random_component = res.new_component_addresses()[0];

    let encoder = AddressBech32Encoder::for_simulator();
    let package_addr = encoder.encode(package_address.as_ref());
    let component_addr = encoder.encode(random_component.as_ref());
    println!("package_addr:\n{:?}\n", package_addr);
    println!("component_addr:\n{:?}\n", component_addr);

    // Deploy ExampleCaller
    let package_address2 = test_runner.compile_and_publish_retain_blueprints(
        dir_example,
        |blueprint, _| blueprint.eq("ExampleCaller"),
    );
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                package_address2,
                "ExampleCaller",
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
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                random_component,
                "process",
                manifest_args!(1u32, 16u64),
            )
            .build(), vec![]);
    let result = receipt.expect_commit_success();
    result.outcome.expect_success();

    // Assert
}


// #[test]
// fn test_create_additional_admin() {
//     // Set up environment.
//     let mut test_runner = TestRunnerBuilder::new().build();
//
//     PackageAddress::try_from_bech32( "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet");
//
//     // Create an account
//     let (public_key, _private_key, account_component) = test_runner.new_allocated_account();
//
//     // Publish package
//     let package_address = test_runner.compile_and_publish(this_package!());
//
//     // Test the `instantiate_flat_admin` function.
//     let manifest1 = ManifestBuilder::new()
//         .call_function(
//             package_address,
//             "FlatAdmin",
//             "instantiate_flat_admin",
//             manifest_args!("test"),
//         )
//         .call_method(
//             account_component,
//             "deposit_batch",
//             manifest_args!(ManifestExpression::EntireWorktop),
//         )
//         .build();
//     let receipt1 = test_runner.execute_manifest_ignoring_fee(
//         manifest1,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );
//     println!("{:?}\n", receipt1);
//     receipt1.expect_commit_success();
//
//     // Test the `create_additional_admin` method.
//     let flat_admin = receipt1.expect_commit(true).new_component_addresses()[0];
//
//     let admin_badge = receipt1.expect_commit(true).new_resource_addresses()[1];
//
//     let manifest2 = ManifestBuilder::new()
//         .create_proof_from_account_of_amount(account_component, admin_badge, dec!("1"))
//         .call_method(flat_admin, "create_additional_admin", manifest_args!())
//         .call_method(
//             account_component,
//             "deposit_batch",
//             manifest_args!(ManifestExpression::EntireWorktop),
//         )
//         .build();
//     let receipt2 = test_runner.execute_manifest_ignoring_fee(
//         manifest2,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );
//     println!("{:?}\n", receipt2);
//     receipt2.expect_commit_success();
// }