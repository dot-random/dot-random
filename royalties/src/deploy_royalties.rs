use scrypto::prelude::*;
use crate::royalties::royalties::{DynamicRoyalties, DynamicRoyaltiesFunctions};

#[blueprint]
mod deploy_royalties {
    struct Deployer {}

    impl Deployer {
        /// Instantiate in Stokenet and Mainnet
        pub fn instantiate() -> (Global<Deployer>, Bucket, Bucket) {
            let mut addresses: Vec<GlobalAddressReservation> = Vec::new();
            for _ in 0..10 {
                let (address_reservation, _) =
                    Runtime::allocate_component_address(<DynamicRoyalties>::blueprint_id());
                addresses.push(address_reservation);
            }

            return Self::instantiate_with_addresses(addresses);
        }

        /// Instantiate with Component and Badge address reservation - for unit tests
        pub fn instantiate_with_addresses(addresses: Vec<GlobalAddressReservation>) -> (Global<Deployer>, Bucket, Bucket) {
            let owner_badge = Self::create_owner_badge();
            let owner_badge_addr = owner_badge.resource_address();
            debug!("owner_badge: {:?}", owner_badge_addr);

            let watcher_badge = Self::create_watcher_badge(owner_badge_addr);
            debug!("watcher_badge: {:?}", watcher_badge.resource_address());

            let base_fees = vec!(
                dec!(0), dec!(0.06), dec!(0.12), dec!(0.18), dec!(0.24), dec!(0.30), dec!(0.36), dec!(0.42), dec!(0.48), dec!(0.54)
            );

            let mut i: usize = 0;
            for address in addresses {
                Blueprint::<DynamicRoyalties>::instantiate(address, owner_badge_addr, base_fees[i]);
                i += 1;
            }

            return (
                Self {}.instantiate().prepare_to_globalize(OwnerRole::None).globalize(),
                owner_badge, watcher_badge
            );
        }

        fn create_owner_badge() -> Bucket {
            return ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "Random Component Owner", locked;
                        "symbol" => "RCOWNER", locked;
                        "description" => "A badge that allows managing the RandomComponent", locked;
                        "tags" => vec!("badge", "rng", ".random", "Random-Component"), updatable;
                    }
                ))
                .mint_initial_supply(10)
                .into();
        }

        fn create_watcher_badge(owner_badge_addr: ResourceAddress) -> Bucket {
            return ResourceBuilder::new_fungible(
                OwnerRole::Fixed(rule!(require(owner_badge_addr))))
                .mint_roles(mint_roles! {
                    minter => rule!(require(owner_badge_addr));
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(owner_badge_addr));
                    burner_updater => rule!(deny_all);
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(require(owner_badge_addr));
                    recaller_updater => rule!(deny_all);
                })
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "Random Component Watcher", locked;
                        "symbol" => "RCWATCH", locked;
                        "description" => "A badge that allows executing ", locked;
                        "tags" => vec!("badge", "rng", ".random", "Random-Component", "bot"), updatable;
                    }
                ))
                .mint_initial_supply(2)
                .into();
        }
    }
}