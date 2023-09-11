use std::path::Path;
use std::process::Command;
use radix_engine::types::*;
// use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::model::InstructionV1;
use transaction::prelude::*;

const PACKAGE_ADDRESS: PackageAddress = PackageAddress::new_or_panic([
    13, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
]);

const COMPONENT_ADDRESS: GlobalAddress = GlobalAddress::new_or_panic([
    195, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 5,
]);



#[test]
fn test_request_mint() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    test_runner
        .compile_and_publish_at_address("../component", PACKAGE_ADDRESS);
    let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        vec![InstructionV1::CallFunction {
            package_address: DynamicPackageAddress::Static(PACKAGE_ADDRESS),
            blueprint_name: "RandomComponent".to_string(),
            function_name: "instantiate_addr".to_string(),
            args: manifest_args!(ManifestAddressReservation(0)).into(),
        }],
        vec![(
            BlueprintId::new(&PACKAGE_ADDRESS, "RandomComponent"),
            COMPONENT_ADDRESS,
        ).into()],
        btreeset!(),
    );
    receipt.expect_commit_success();

    // Act
    let package_address2 = test_runner.compile_and_publish_retain_blueprints(
        "./src",
        |blueprint, _| blueprint.eq("ExampleCaller"),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address2,
            "ExampleCaller",
            "request_mint",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    let result = receipt.expect_commit_success();
    let output = result.outcome.expect_success();
    output[1].expect_return_value(&"my_secret".to_string());
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