use scrypto::prelude::*;

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

#[blueprint]
#[types(u32, Callback, ResourceAddress, Vault)]
mod component {
    enable_method_auth! {
        roles {
            admin => updatable_by: [];
            watcher => updatable_by: [];
        },
        methods {
            request_random => PUBLIC;
            execute => restrict_to: [watcher];
            handle_error => restrict_to: [watcher];
            evict => restrict_to: [watcher];
            withdraw => restrict_to: [admin];
            withdraw_badges => restrict_to: [admin];
        }
    }
    struct RandomComponent {
        /// Enqueued callbacks to process.
        queue: KeyValueStore<u32, Callback>,

        /// Holds the tokens that should be sent back to the Caller to auth the `callback` and `on_error`.
        vaults: KeyValueStore<ResourceAddress, Vault>,

        /// Holds the badge that we present when executing the `callback` and `on_error`.
        badges: Vault,

        /// Callback ID sequence
        callback_seq: u32,
    }


    impl RandomComponent {
        /// Instantiate in Stokenet and Mainnet
        pub fn instantiate() -> (Global<RandomComponent>, Bucket, Bucket) {
            return Self::do_instantiate(
                None, None
            );
        }

        /// Instantiate with Component and Badge address reservation - for unit tests
        pub fn instantiate_addr_badge(
            component_address: GlobalAddressReservation,
            component_badge_address: GlobalAddressReservation,
        ) -> (Global<RandomComponent>, Bucket, Bucket) {
            return Self::do_instantiate(
                Some(component_badge_address),
                Some(component_address),
            );
        }

        fn do_instantiate(comp_badge_address: Option<GlobalAddressReservation>,
                          component_addr: Option<GlobalAddressReservation>) -> (Global<RandomComponent>, Bucket, Bucket) {
            let owner_badge = Self::create_owner_badge();
            let owner_badge_addr = owner_badge.resource_address();
            debug!("owner_badge: {:?}", owner_badge_addr);

            let watcher_badge = Self::create_watcher_badge(owner_badge_addr);
            let watcher_badge_addr = watcher_badge.resource_address();
            debug!("watcher_badge: {:?}", watcher_badge_addr);

            let comp_badge = Self::create_component_badge(owner_badge_addr, comp_badge_address);
            debug!("comp_badge: {:?}", comp_badge.resource_address());

            let comp: Owned<RandomComponent> = Self {
                queue: KeyValueStore::new_with_registered_type(),
                vaults: KeyValueStore::new_with_registered_type(),

                badges: Vault::with_bucket(comp_badge),

                callback_seq: 0,
            }.instantiate();
            let mut globalizing = comp
                .prepare_to_globalize(OwnerRole::Fixed(
                    rule!(require(owner_badge_addr))
                ))
                .roles(roles!(
                    admin => rule!(require(owner_badge_addr));
                    watcher => rule!(require(watcher_badge_addr));
                ))
                .enable_component_royalties(component_royalties! {
                    init {
                        request_random => Usd(dec!(0.06)), locked;
                        execute => Free, locked;
                        handle_error => Free, locked;
                        evict => Free, locked;
                        withdraw => Free, locked;
                        withdraw_badges => Free, locked;
                    }
                });
            if component_addr.is_some() {
                globalizing = globalizing.with_address(component_addr.unwrap());
            }
            return (globalizing.globalize(), owner_badge, watcher_badge);
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


        /**
         * Called by any external Component.
         * the Caller should also pass a badge that controls access to <method_name>().
         */
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, on_error: String,
                              key: u32, badge_opt: Option<FungibleBucket>) -> u32 {
            debug!("EXEC:RandomComponent::request_random({:?}..{:?}, {:?}, {:?}, {:?})", address, method_name, on_error, key, badge_opt);

            let mut resource: Option<ResourceAddress> = None;
            let mut amount = Decimal::ZERO;
            if badge_opt.is_some() {
                let badge = badge_opt.unwrap();
                let res: ResourceAddress = badge.resource_address();
                amount = badge.amount();

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

                resource = Some(res);
            }

            self.callback_seq += 1;
            let callback_id: u32 = self.callback_seq;
            self.queue.insert(callback_id, Callback { address, method_name, on_error, key, resource, amount });
            return callback_id;
        }

        /**
         * Execute a specific callback. Will be used until we can reliably process the whole queue.
         * Also called to preview the execution result (Success/Failure) of a specific Callback.
         */
        pub fn execute(&mut self, callback_id: u32, random_seed: Vec<u8>) {
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
                    let proof = self.badges.as_fungible().create_proof_of_amount(Decimal::ONE);
                    proof.authorize(|| {
                        let comp: Global<AnyComponent> = Global::from(callback.address);
                        comp.call_ignore_rtn(callback.method_name.as_str(), &(callback.key, random_seed));
                    });
                }
            }
        }

        /// In case calling `method_name()` on a specific callback panics.
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
                        let proof = self.badges.as_fungible().create_proof_of_amount(Decimal::ONE);
                        proof.authorize(|| {
                            let comp: Global<AnyComponent> = Global::from(callback.address);
                            comp.call_ignore_rtn(callback.on_error.as_str(), &(callback.key));
                        });
                    }
                }
            }
        }

        /// Evicts a faulty callback from the queue.
        /// A callback is considered faulty when both <method_name> and <on_error> panic during the execution.
        pub fn evict(&mut self, callback_id: u32) {
            self.queue.remove(&callback_id);
        }

        /// Withdraw the assets, e.g. when both callback and error handler failed, and we had to evict it.
        pub fn withdraw(&mut self, resource: ResourceAddress, amount: Decimal) -> Bucket {
            let opt = self.vaults.get_mut(&resource);
            return opt.unwrap().take(amount);
        }

        /// Withdraw badges, e.g. to migrate to a new version of the component
        pub fn withdraw_badges(&mut self, amount: Decimal) -> Bucket {
            return self.badges.take(amount);
        }

    }
}