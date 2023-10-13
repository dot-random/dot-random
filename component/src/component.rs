use scrypto::prelude::*;
use crate::royalties::royalties::{FeeAdvances, FeeAdvancesFunctions};

#[derive(ScryptoSbor)]
pub struct Callback {
    /// The caller component
    address: ComponentAddress,
    /// also referred to as the `callback`.
    method_name: String,
    /// executed when the callback panics.
    on_error: String,
    /// resource to be sent back with the `callback` and `on_error`.
    /// If missing - we will just show our own badge.
    resource: Option<ResourceAddress>,
    /// The amount of the above resource.
    amount: Decimal,
    /// The first argument of the `callback` and `on_error` methods.
    key: u32,
}

const MAX_BATCH_SIZE: u32 = 16;

#[blueprint]
#[types(u32, Callback, ResourceAddress, Vault, ComponentAddress, u8)]
mod component {
    enable_method_auth! {
        roles {
            admin => updatable_by: [];
            watcher => updatable_by: [];
        },
        methods {
            request_random => PUBLIC;
            request_random2 => PUBLIC;
            process => restrict_to: [watcher];
            process_one => restrict_to: [watcher];
            evict => restrict_to: [watcher];
            handle_error => restrict_to: [watcher];
            update_caller_royalties => restrict_to: [watcher];
        }
    }
    struct RandomComponent {
        /// Enqueued callbacks to process.
        queue: KeyValueStore<u32, Callback>,
        /// Holds the tokens that should be sent back to the Caller to auth the `callback` and `on_error`.
        vaults: KeyValueStore<ResourceAddress, Vault>,

        /// Holds the badge that we present when executing the `callback` and `on_error`.
        component_badges: Vault,

        id_seq: u32,
        last_processed_id: u32,

        /// The component that gathers royalties. Need a separate component to charge dynamic royalties,
        /// based on the known average execution cost of the `callback` and `on_error` handlers.
        royalty_address: ComponentAddress,
        /// Royalties Level per known caller Component, cents. Should be [0-8, 10].
        caller_royalties: KeyValueStore<ComponentAddress, u8>,
    }


    impl RandomComponent {
        /// Instantiate in Stokenet and Mainnet
        pub fn instantiate() -> (Global<RandomComponent>, Bucket, Bucket) {
            return Self::globalize(
                Self::instantiate_local(None),
                None,
            );
        }

        /// Instantiate with Component and Badge address reservation - for unit tests
        pub fn instantiate_addr_badge(
            component_address: GlobalAddressReservation,
            component_badge_address: GlobalAddressReservation,
        ) -> (Global<RandomComponent>, Bucket, Bucket) {
            return Self::globalize(
                Self::instantiate_local(Some(component_badge_address)),
                Some(component_address),
            );
        }

        fn globalize((comp, owner_badge, watcher_badge): (Owned<RandomComponent>, Bucket, Bucket),
                     component_addr: Option<GlobalAddressReservation>) -> (Global<RandomComponent>, Bucket, Bucket) {
            let owner_resource: ResourceAddress = owner_badge.resource_address();
            let watcher_resource: ResourceAddress = watcher_badge.resource_address();
            let mut globalizing = comp
                .prepare_to_globalize(OwnerRole::Fixed(
                    rule!(require(owner_resource))
                ))
                .roles(roles!(
                    admin => rule!(require(owner_resource));
                    watcher => rule!(require(watcher_resource));
                ))
                .enable_component_royalties(component_royalties! {
                    init {
                        request_random => Usd(dec!(0.12)), locked;
                        request_random2 => Usd(dec!(0.12)), locked;
                        process => Free, locked;
                        process_one => Free, locked;
                        evict => Free, locked;
                        handle_error => Free, locked;
                        update_caller_royalties => Free, locked;
                    }
                });
            if component_addr.is_some() {
                globalizing = globalizing.with_address(component_addr.unwrap());
            }
            let global = globalizing.globalize();
            return (global, owner_badge, watcher_badge);
        }

        fn instantiate_local(comp_badge_address: Option<GlobalAddressReservation>) -> (Owned<RandomComponent>, Bucket, Bucket) {
            let owner_badge = Self::create_owner_badge();
            let owner_badge_addr = owner_badge.resource_address();
            debug!("owner_badge:\n{:?}\n", owner_badge_addr);

            let comp_badge = Self::create_component_badge(owner_badge_addr, comp_badge_address);
            let comp_badge_addr = comp_badge.resource_address();
            debug!("comp_badge:\n{:?}\n", comp_badge_addr);

            let watcher_badge = Self::create_watcher_badge(owner_badge_addr);
            debug!("watcher_badge:\n{:?}\n", watcher_badge.resource_address());

            let royalty_address: ComponentAddress = Blueprint::<FeeAdvances>::instantiate(comp_badge_addr);

            let comp: Owned<RandomComponent> = Self {
                queue: KeyValueStore::new_with_registered_type(),
                vaults: KeyValueStore::new_with_registered_type(),

                component_badges: Vault::with_bucket(comp_badge),

                id_seq: 0,
                last_processed_id: 0,

                royalty_address,
                caller_royalties: KeyValueStore::new_with_registered_type(),
            }.instantiate();
            return (comp, owner_badge, watcher_badge);
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

        fn create_component_badge(owner_badge_addr: ResourceAddress, badge_address: Option<GlobalAddressReservation>) -> Bucket {
            let mut builder = ResourceBuilder::new_fungible(
                OwnerRole::Fixed(rule!(require(owner_badge_addr))))
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "Random Component Badge", locked;
                        "symbol" => "RC", locked;
                        "description" => "A badge presented during the callback execution", locked;
                        "tags" => vec!("badge", "rng", ".random", "Random-Component"), updatable;
                    }
                ));

            if badge_address.is_some() {
                builder = builder.with_address(badge_address.unwrap());
            }
            return builder
                .mint_initial_supply(100)
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

        /**
         * Called by any external Component.
         * the Caller should also pass a badge that controls access to <method_name>().
         */
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, on_error: String, key: u32, badge: FungibleBucket) -> u32 {
            debug!("EXEC:RandomComponent::request_random()");

            self.charge_royalty(&address);

            let res: ResourceAddress = badge.resource_address();
            let amount: Decimal = badge.amount();
            let vault;
            {
                let opt = self.vaults.get_mut(&res);
                let bucket = badge.into();
                if let Some(mut v) = opt {
                    v.put(bucket);
                    vault = None;
                } else {
                    vault = Some(Vault::with_bucket(bucket));
                }
            }

            if vault.is_some() {
                self.vaults.insert(res, vault.unwrap());
            }

            let resource = Some(res);

            self.id_seq += 1;
            let callback_id: u32 = self.id_seq;
            self.queue.insert(callback_id, Callback { address, method_name, on_error, key, resource, amount });
            return callback_id;
        }

        /**
         * Called by any external Component.
         * the Caller should protect access to <method_name>() with a badge from [component_badges].
         */
        pub fn request_random2(&mut self, address: ComponentAddress, method_name: String, on_error: String, key: u32) -> u32 {
            debug!("EXEC:RandomComponent::request_random2()");

            self.charge_royalty(&address);

            self.id_seq += 1;
            let callback_id: u32 = self.id_seq;
            let resource = None;
            let amount = Decimal::ZERO;
            self.queue.insert(callback_id, Callback { address, method_name, on_error, key, resource, amount });
            return callback_id;
        }

        /**
         * Will be called by the Random Watcher off-ledger service sometime in future.
         * For now it's just a template.
         * TODO: Will be protected by badges.
         */
        pub fn process(&mut self, random_seed: Vec<u8>) {
            debug!("EXEC:RandomComponent::process({:?}..{:?}, {:?})", self.last_processed_id, self.id_seq, random_seed);

            let start = self.last_processed_id;
            let end = self.last_processed_id + MAX_BATCH_SIZE;
            let mut seed = random_seed.clone();
            while self.last_processed_id < self.id_seq && self.last_processed_id < end {
                if start != self.last_processed_id {
                    seed.rotate_left(7);
                };

                let id = self.last_processed_id + 1;

                self.do_process(id, seed.clone());
                self.last_processed_id = id;
            }
        }

        /**
         * Process a specific callback. Will be used until we can reliably process the whole queue.
         * Also called to preview the execution result (Success/Failure) of a specific Callback.
         */
        pub fn process_one(&mut self, callback_id: u32, random_seed: Vec<u8>) {
            self.do_process(callback_id, random_seed);
        }

        pub fn handle_error(&mut self, callback_id: u32) {
            let queue_item: Option<Callback> = self.queue.remove(&callback_id);
            if queue_item.is_some() {
                let callback = queue_item.unwrap();
                if !callback.on_error.is_empty() {
                    let resource_opt = callback.resource;
                    if let Some(resource) = resource_opt {
                        if callback.amount.is_positive() {
                            let opt = self.vaults.get_mut(&resource);
                            if let Some(mut v) = opt {
                                let bucket = v.take(callback.amount).as_fungible();
                                let comp: Global<AnyComponent> = Global::from(callback.address);
                                comp.call_ignore_rtn(callback.on_error.as_str(), &(callback.key, bucket));
                            }
                        }
                    } else {
                        let proof = self.component_badges.as_fungible().create_proof_of_amount(Decimal::ONE);
                        proof.authorize(|| {
                            let comp: Global<AnyComponent> = Global::from(callback.address);
                            comp.call_ignore_rtn(callback.on_error.as_str(), &(callback.key));
                        });
                    }
                }
            }
        }

        /// Evicts a faulty callback from the queue.
        /// A callback is considered faulty when both <method_name> and <on_error> panic during the simulation.
        pub fn evict(&mut self, callback_id: u32) {
            self.queue.remove(&callback_id);
        }

        /// Updates the royalties for a specific component.
        /// Called by the off-ledger service to maintain the royalties gained from `request_random()`
        /// at a level matching the TX fees incurred when calling `process()`.
        ///
        pub fn update_caller_royalties(&mut self, address: ComponentAddress, royalty: u8) {
            assert!(royalty < 11 && royalty != 9, "Incorrect Royalty level: {}", royalty);
            self.caller_royalties.insert(address, royalty);
        }


        fn charge_royalty(&mut self, address: &ComponentAddress) {
            let option: Option<_> = self.caller_royalties.get(&address);
            let level = if option.is_some() { *option.unwrap() } else { 0u8 };
            if level > 0u8 {
                let method = format!("dynamic_royalty_{}", level);
                let comp: Global<AnyComponent> = Global::from(self.royalty_address);
                comp.call_ignore_rtn(method.as_str(), &());
            }
        }

        fn do_process(&mut self, callback_id: u32, random_seed: Vec<u8>) {
            let queue_item: Option<Callback> = self.queue.remove(&callback_id);
            if queue_item.is_some() {
                let callback = queue_item.unwrap();
                let resource_opt = callback.resource;
                if let Some(resource) = resource_opt {
                    if callback.amount.is_positive() {
                        let opt = self.vaults.get_mut(&resource);
                        if let Some(mut v) = opt {
                            let bucket = v.take(callback.amount).as_fungible();
                            let comp: Global<AnyComponent> = Global::from(callback.address);
                            comp.call_ignore_rtn(callback.method_name.as_str(), &(callback.key, bucket, random_seed));
                        }
                    }
                } else {
                    let proof = self.component_badges.as_fungible().create_proof_of_amount(Decimal::ONE);
                    proof.authorize(|| {
                        let comp: Global<AnyComponent> = Global::from(callback.address);
                        comp.call_ignore_rtn(callback.method_name.as_str(), &(callback.key, random_seed));
                    });
                }
            }
        }
    }
}