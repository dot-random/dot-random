use radix_engine::types::*;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test_request_random() {
    // Set up environment.
    let mut test_runner = TestRunnerBuilder::new().build();

    // Create an account
    let (public_key, _private_key, account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Test the `instantiate` function.
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_function(
                package_address,
                "RandomComponent",
                "instantiate",
                manifest_args!(),
            )
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    println!("instantiate receipt:\n{:?}\n", receipt);
    let commit = receipt.expect_commit_success();

    // Test actual request_random
    let component = commit.new_component_addresses()[0];

    let resource = test_runner.create_freely_mintable_fungible_resource(OwnerRole::None, Some(Decimal::ONE), DIVISIBILITY_NONE, account);

    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .take_all_from_worktop(resource, "bucket1")
            .with_name_lookup(|builder, lookup| {
                builder.call_method(
                    component,
                    "request_random",
                    (account, "test", "test_on_error", 123u32, lookup.bucket("bucket1")))
            })

            .build(),
        vec![],
    );
    println!("request_random receipt:\n{:?}\n", receipt);
    receipt.expect_commit_success();
}
