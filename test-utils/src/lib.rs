use std::path::PathBuf;

use radix_engine::transaction::{CommitResult, TransactionReceipt};
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

pub struct RandomTestUtil {
    pub package: PackageAddress,
    pub component: ComponentAddress,
    pub badge: ResourceAddress,

    pub owner_badge: ResourceAddress,
    pub owner_account: ComponentAddress,
    pub owner_pk: Secp256k1PublicKey,
    pub watcher_badge: ResourceAddress,
}

impl RandomTestUtil {
    pub fn process<E: NativeVmExtension, D: TestDatabase>(&self, test_runner: &mut TestRunner<E, D>, random_bytes: Vec<u8>) -> CommitResult {
        let receipt = self.try_process(test_runner, self.component, random_bytes);
        let result = receipt.expect_commit_success();
        result.outcome.expect_success();
        return result.clone();
    }

    pub fn try_process<E: NativeVmExtension, D: TestDatabase>(&self, test_runner: &mut TestRunner<E, D>,
                                                                               rc_component: ComponentAddress, random_bytes: Vec<u8>) -> TransactionReceipt {
        let receipt = test_runner.execute_manifest_ignoring_fee(
            ManifestBuilder::new()
                .create_proof_from_account_of_amount(self.owner_account, self.watcher_badge, Decimal::ONE )
                .call_method(
                    rc_component,
                    "process",
                    manifest_args!(random_bytes),
                )
                .build(), vec![NonFungibleGlobalId::from_public_key(&self.owner_pk)]);
        return receipt;
    }

    pub fn process_one<E: NativeVmExtension, D: TestDatabase>(&self, test_runner: &mut TestRunner<E, D>,
                                                                               rc_component: ComponentAddress, callback_id: u32, random_bytes: Vec<u8>) -> CommitResult {
        let receipt = self.try_process_one(test_runner, rc_component, callback_id, random_bytes);
        let result = receipt.expect_commit_success();
        result.outcome.expect_success();
        return result.clone();
    }

    pub fn try_process_one<E: NativeVmExtension, D: TestDatabase>(&self, test_runner: &mut TestRunner<E, D>,
                                                                                   rc_component: ComponentAddress, callback_id: u32, random_bytes: Vec<u8>) -> TransactionReceipt {
        let receipt = test_runner.execute_manifest_ignoring_fee(
            ManifestBuilder::new()
                .call_method(
                    rc_component,
                    "process_one",
                    manifest_args!(callback_id, random_bytes),
                )
                .build(), vec![]);
        return receipt;
    }

    pub fn handle_error<E: NativeVmExtension, D: TestDatabase>(&self, test_runner: &mut TestRunner<E, D>,
                                                                                rc_component: ComponentAddress, callback_id: u32) -> CommitResult {
        let receipt = test_runner.execute_manifest_ignoring_fee(
            ManifestBuilder::new()
                .call_method(
                    rc_component,
                    "handle_error",
                    manifest_args!(callback_id),
                )
                .build(), vec![]);
        let result = receipt.expect_commit_success();
        result.outcome.expect_success();
        return result.clone();
    }
}

pub fn random_component_deploy<E: NativeVmExtension, D: TestDatabase>(test_runner: &mut TestRunner<E, D>, commit_hash: &str)
                                                                      -> RandomTestUtil {
    let component_path = get_component_dir("dot-random", commit_hash);
    let dir_component = component_path.to_str().unwrap();
    return random_component_deploy_dir(test_runner, dir_component);
}

pub fn random_component_deploy_dir<E: NativeVmExtension, D: TestDatabase>(test_runner: &mut TestRunner<E, D>, dir_component: &str)
                                                                                 -> RandomTestUtil {
    let rc_package = PackageAddress::new_or_panic(RANDOM_PACKAGE);
    test_runner.compile_and_publish_at_address(dir_component, rc_package);

    let (public_key, _, owner_account) = test_runner.new_allocated_account();

    let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        vec![
            InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(rc_package),
                blueprint_name: "RandomComponent".to_string(),
                function_name: "instantiate_addr_badge".to_string(),
                args: manifest_args!(ManifestAddressReservation(0), ManifestAddressReservation(1)).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(owner_account.into())),
                method_name: "deposit_batch".to_string(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
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
        btreeset!(NonFungibleGlobalId::from_public_key(&public_key)),
    );

    let res = receipt.expect_commit_success();
    let rc_component = res.new_component_addresses()[0];
    let owner_badge = res.new_resource_addresses()[0];
    let component_badge = res.new_resource_addresses()[1];
    let watcher_badge = res.new_resource_addresses()[2];

    let encoder = AddressBech32Encoder::for_simulator();
    let package_addr = encoder.encode(rc_package.as_ref());
    let component_addr = encoder.encode(rc_component.as_ref());
    let badge_addr = encoder.encode(component_badge.as_ref());
    println!("RandomComponent:package_addr: {:?}\n", package_addr);
    println!("RandomComponent:component_addr: {:?}\n", component_addr);
    println!("RandomComponent:resource_addr: {:?}\n", badge_addr);

    return RandomTestUtil {
        package: rc_package,
        component: rc_component,
        badge: component_badge,

        owner_badge,
        owner_account,
        owner_pk: public_key,
        watcher_badge
    };
}

fn add_dir(p: PathBuf, dir: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push("/");
    p.push(dir);
    return p.into();
}

pub fn get_dependency_dir(repo_name: &str, commit_hash: &str) -> Option<PathBuf> {
    assert_eq!(7, commit_hash.len(), "Commit hash should be 7 chars!");
    let git_dir = add_dir(home::cargo_home().unwrap(), "git/checkouts");
    let option = std::fs::read_dir(git_dir).ok();
    let mut commit_dir: Option<PathBuf> = None;
    for entry in option.unwrap() {
        let path = entry.ok()?.path();
        if path.is_dir() && path.iter().last().unwrap().to_str().unwrap().starts_with(repo_name) {
            commit_dir = Some(add_dir(path.clone(), commit_hash));
        }
    }
    assert!(commit_dir.is_some(), "Can't find a repository '{:?}' or commit '{:?}' in Cargo cache!", repo_name, commit_hash);
    return commit_dir;
}

fn get_component_dir(repo_name: &str, commit_hash: &str) -> PathBuf {
    let dot_random_dir = get_dependency_dir(repo_name, commit_hash).unwrap();
    let component_path = add_dir(dot_random_dir, "component");
    return component_path;
}