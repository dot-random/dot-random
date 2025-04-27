use std::env;
use scrypto_test::prelude::*;

const ROYAL_PACKAGE: [u8; NodeId::LENGTH] = [
    13, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 127, 127, 127, 19, 19,
];
const ROYAL_ADDRESS: [u8; NodeId::LENGTH] = [
    192, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 55, 55, 55, 55,
];


#[test]
fn test_request_random() {
    // dir is different in Debug mode
    let root_dir = env::current_dir().ok().unwrap().ends_with("dot-random");
    let dir_royal = if root_dir { "./royalties" } else { "../royalties" };

    // Set up environment.
    let mut test_runner = LedgerSimulatorBuilder::new().build();

    // Create an account
    let (public_key, _private_key, account) = test_runner.new_allocated_account();

    // Publish royalties package
    let royalties_package = PackageAddress::new_or_panic(ROYAL_PACKAGE);
    test_runner.compile_and_publish_at_address(dir_royal, royalties_package);

    // Instantiate the DynamicRoyalties.
    let mut builder = ManifestBuilder::<SystemTransactionManifestV1>::new_typed();

    let mut manifest_reservations: Vec<String> = Vec::new();
    for i in 0..10u8 {
        let reservation = i.to_string();
        manifest_reservations.push(reservation.clone());
        let mut addr = ROYAL_ADDRESS.clone();
        addr[addr.len() - 5] = i;
        builder = builder.preallocate_address(reservation, GlobalAddress::new_or_panic(addr), royalties_package, "DynamicRoyalties");
    }

    let receipt = test_runner.execute_system_transaction(
        builder.with_name_lookup(|builder, lookup| {
            let args: Vec<ManifestAddressReservation> = manifest_reservations.into_iter()
                .map(|s| lookup.address_reservation(s))
                .collect();
            builder.call_function(
                DynamicPackageAddress::Static(royalties_package),
                "Deployer".to_string(),
                "instantiate_with_addresses".to_string(),
                manifest_args!(args)
            )
        })
            .call_method(
                DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(account.into())),
                "deposit_batch".to_string(),
                manifest_args!(ManifestExpression::EntireWorktop)
            ).build(),
        btreeset!(NonFungibleGlobalId::from_public_key(&public_key))
    );

    println!("instantiate receipt:\n{:?}\n", receipt);
    let commit = receipt.expect_commit_success();
    let owner_badge: ResourceAddress = commit.new_resource_addresses()[0];
    let watcher_badge: ResourceAddress = commit.new_resource_addresses()[1];

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Instantiate the Component.
    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_function(
                package_address,
                "RandomComponent",
                "instantiate",
                manifest_args!(owner_badge, watcher_badge),
            )
            .deposit_entire_worktop(account)
            .build(),
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    println!("instantiate receipt:\n{:?}\n", receipt);
    let commit = receipt.expect_commit_success();

    // Test actual request_random
    let component = commit.new_component_addresses()[0];

    let resource = test_runner.create_freely_mintable_fungible_resource(OwnerRole::None, Some(Decimal::ONE), DIVISIBILITY_NONE, account);

    let receipt = test_runner.execute_manifest(
        ManifestBuilder::new()
            .lock_fee_from_faucet()
            .take_all_from_worktop(resource, "bucket1")
            .with_name_lookup(|builder, lookup| {
                builder.call_method(
                    component,
                    "request_random",
                    (account, "test", "test_on_error", 123u32, Some(lookup.bucket("bucket1")), 0u8))
            })

            .build(),
        vec![],
    );
    println!("request_random receipt:\n{:?}\n", receipt);
    receipt.expect_commit_success();
}
