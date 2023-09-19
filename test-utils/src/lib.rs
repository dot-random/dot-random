use radix_engine::vm::NativeVmExtension;
use scrypto_unit::*;
use transaction::prelude::*;

const RANDOM_PACKAGE: [u8; NodeId::LENGTH] = [
    13, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 0, 0, 19, 19,
];
const RANDOM_COMPONENT: [u8; NodeId::LENGTH] = [
    192, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 0, 0, 19, 19,
];
const RANDOM_BADGE: [u8; NodeId::LENGTH] = [
    93, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 55, 55, 55, 1, 0, 0, 0, 0, 19, 19,
];

pub fn deploy_random_component<E: NativeVmExtension, D: TestDatabase>(dir_component: &str, test_runner: &mut TestRunner<E, D>)
                                                                  -> (PackageAddress, ComponentAddress, ResourceAddress) {
    let rc_package = PackageAddress::new_or_panic(RANDOM_PACKAGE);
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
                 GlobalAddress::new_or_panic(RANDOM_COMPONENT),
             )
                 .into(),
             (
                 BlueprintId::new(&RESOURCE_PACKAGE, FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_owned()),
                 GlobalAddress::new_or_panic(RANDOM_BADGE),
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