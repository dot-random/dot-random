use std::marker::PhantomData;

use radix_engine::transaction::{CommitResult, TransactionReceipt};
use radix_engine::vm::NativeVmExtension;
use scrypto_unit::*;
use transaction::prelude::*;
pub mod cargo;
use crate::cargo::*;
pub mod constants;
use crate::constants::*;


pub struct RandomTestEnv<E: NativeVmExtension, D: TestDatabase> {
    pub package: PackageAddress,
    pub component: ComponentAddress,
    pub badge: ResourceAddress,

    pub owner_badge: ResourceAddress,
    pub owner_account: ComponentAddress,
    pub owner_pk: Secp256k1PublicKey,
    pub watcher_badge: ResourceAddress,

    pub callback_seq: u32,
    phantom: PhantomData<E>,
    phantom2: PhantomData<D>,
}

impl<E: NativeVmExtension, D: TestDatabase> RandomTestEnv<E, D> {

    pub fn execute_with_seed(&mut self, test_runner: &mut TestRunner<E, D>, random_bytes: Vec<u8>) -> CommitResult {
        let receipt = self.try_execute_next(test_runner, random_bytes);
        return receipt.expect_commit_success().clone();
    }

    pub fn execute_next(&mut self, test_runner: &mut TestRunner<E, D>, random_number: u32) -> CommitResult {
        let random_bytes: Vec<u8> = Self::deterministic_bytes_from_number(random_number);
        let receipt = self.try_execute_next(test_runner, random_bytes);
        return receipt.expect_commit_success().clone();
    }

    pub fn try_execute_next(&mut self, test_runner: &mut TestRunner<E, D>, random_bytes: Vec<u8>) -> TransactionReceipt {
        self.callback_seq += 1;
        return self.try_execute(test_runner, self.callback_seq, random_bytes);
    }

    pub fn execute(&self, test_runner: &mut TestRunner<E, D>,
                       callback_id: u32, random_bytes: Vec<u8>) -> CommitResult {
        let receipt = self.try_execute(test_runner, callback_id, random_bytes);
        return receipt.expect_commit_success().clone();
    }

    pub fn try_execute(&self, test_runner: &mut TestRunner<E, D>, callback_id: u32, random_bytes: Vec<u8>) -> TransactionReceipt {
        let receipt = test_runner.execute_manifest_ignoring_fee(
            ManifestBuilder::new()
                .create_proof_from_account_of_amount(self.owner_account, self.watcher_badge, Decimal::ONE)
                .call_method(
                    self.component,
                    "execute",
                    manifest_args!(callback_id, random_bytes),
                )
                .build(), vec![NonFungibleGlobalId::from_public_key(&self.owner_pk)]);
        return receipt;
    }

    pub fn handle_error(&self, test_runner: &mut TestRunner<E, D>, callback_id: u32) -> CommitResult {
        let receipt = test_runner.execute_manifest_ignoring_fee(
            ManifestBuilder::new()
                .create_proof_from_account_of_amount(self.owner_account, self.watcher_badge, Decimal::ONE)
                .call_method(
                    self.component,
                    "handle_error",
                    manifest_args!(callback_id),
                )
                .build(), vec![NonFungibleGlobalId::from_public_key(&self.owner_pk)]);
        let result = receipt.expect_commit_success();
        result.outcome.expect_success();
        return result.clone();
    }

    pub fn update_royalties(&self, test_runner: &mut TestRunner<E, D>, caller_component: ComponentAddress, royalty_level: u8) -> CommitResult {
        let receipt = self.try_update_royalties(test_runner, caller_component, royalty_level);
        return receipt.expect_commit_success().clone();
    }

    pub fn try_update_royalties(&self, test_runner: &mut TestRunner<E, D>, caller_component: ComponentAddress, royalty_level: u8) -> TransactionReceipt {
        let receipt = test_runner.execute_manifest_ignoring_fee(
            ManifestBuilder::new()
                .create_proof_from_account_of_amount(self.owner_account, self.watcher_badge, Decimal::ONE )
                .call_method(
                    self.component,
                    "update_caller_royalties",
                    manifest_args!(caller_component, royalty_level),
                )
                .build(), vec![NonFungibleGlobalId::from_public_key(&self.owner_pk)]);
        return receipt;
    }

    fn deterministic_bytes_from_number(a_number: u32) -> Vec<u8> {
        let seed = hash(a_number.to_be_bytes());
        let random_bytes = seed.to_vec();

        println!("RandomTestEnv:random_bytes: {:?}", random_bytes);
        return random_bytes;
    }
}

pub fn deploy_random_component<E: NativeVmExtension, D: TestDatabase>
(test_runner: &mut TestRunner<E, D>, commit_hash: &str) -> RandomTestEnv<E, D> {
    let royalties_path = get_repo_sub_dir("dot-random", commit_hash, "royalties");
    let dir_royalties = royalties_path.to_str().unwrap();
    let component_path = get_repo_sub_dir("dot-random", commit_hash, "component");
    let dir_component = component_path.to_str().unwrap();

    return deploy_random_component_from_dir(test_runner, dir_royalties, dir_component);
}

pub fn deploy_random_component_from_dir<E: NativeVmExtension, D: TestDatabase>
(test_runner: &mut TestRunner<E, D>, dir_royalties: &str, dir_component: &str) -> RandomTestEnv<E, D> {
    let encoder = AddressBech32Encoder::for_simulator();
    let (public_key, _, owner_account) = test_runner.new_allocated_account();

    let ro_package = PackageAddress::new_or_panic(ROYAL_PACKAGE);
    test_runner.compile_and_publish_at_address(dir_royalties, ro_package);

    let mut manifest_reservations: Vec<ManifestAddressReservation> = Vec::new();
    let mut pre_allocated_addresses: Vec<PreAllocatedAddress> = Vec::new();
    for i in 0..10u8 {
        manifest_reservations.push(ManifestAddressReservation(i.into()));
        let mut addr = ROYAL_ADDRESS.clone();
        addr[addr.len() - 5] = i;
        pre_allocated_addresses.push((
            BlueprintId::new(&ro_package, "DynamicRoyalties"),
            GlobalAddress::new_or_panic(addr),
        ).into());
    }

    // Instantiate the DynamicRoyalties.
    let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        vec![
            InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(ro_package),
                blueprint_name: "Deployer".to_string(),
                function_name: "instantiate_with_addresses".to_string(),
                args: manifest_args!(
                    manifest_reservations
                ).into(),
            },
            InstructionV1::CallMethod {
                address: DynamicGlobalAddress::Static(GlobalAddress::new_or_panic(owner_account.into())),
                method_name: "deposit_batch".to_string(),
                args: manifest_args!(ManifestExpression::EntireWorktop).into(),
            }],
        pre_allocated_addresses,
        btreeset!(NonFungibleGlobalId::from_public_key(&public_key)),
    );

    let commit = receipt.expect_commit_success();
    let owner_badge: ResourceAddress = commit.new_resource_addresses()[0];
    let watcher_badge: ResourceAddress = commit.new_resource_addresses()[1];

    let package_addr = encoder.encode(ro_package.as_ref());
    println!("DynamicRoyalties:package_addr: {:?}", package_addr);
    for i in 0..commit.new_component_addresses().len() {
        let component_addr = encoder.encode(commit.new_component_addresses()[i].as_ref());
        println!("DynamicRoyalties:component_addr: {:?}", component_addr);
    }

    let rc_package = PackageAddress::new_or_panic(RANDOM_PACKAGE);
    test_runner.compile_and_publish_at_address(dir_component, rc_package);

    let receipt = test_runner.execute_system_transaction_with_preallocated_addresses(
        vec![
            InstructionV1::CallFunction {
                package_address: DynamicPackageAddress::Static(rc_package),
                blueprint_name: "RandomComponent".to_string(),
                function_name: "instantiate_addr_badge".to_string(),
                args: manifest_args!(owner_badge, watcher_badge, ManifestAddressReservation(0), ManifestAddressReservation(1)).into(),
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
    // since we used AddressReservation, component badge addr got created first. Not the case for `instantiate()`.
    let component_badge = res.new_resource_addresses()[0];

    let encoder = AddressBech32Encoder::for_simulator();
    let package_addr = encoder.encode(rc_package.as_ref());
    let component_addr = encoder.encode(rc_component.as_ref());
    let badge_addr = encoder.encode(component_badge.as_ref());
    println!("RandomComponent:package_addr: {:?}", package_addr);
    println!("RandomComponent:component_addr: {:?}", component_addr);
    println!("RandomComponent:resource_addr: {:?}", badge_addr);

    return RandomTestEnv {
        package: rc_package,
        component: rc_component,
        badge: component_badge,

        owner_badge,
        owner_account,
        owner_pk: public_key,
        watcher_badge,

        callback_seq: 0,

        phantom: PhantomData,
        phantom2: PhantomData,
    };
}
