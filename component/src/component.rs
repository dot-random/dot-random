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
#[types(u32, Callback, ResourceAddress, Vault, ComponentAddress, u8)]
mod component {
    extern_blueprint!(
        "package_rdx1p449l2st4nncankt3rn50dht7c2f3eqvdeumu2cmmgfe8tqz8dkaae",
        DynamicRoyalties {
            fn r1(&self);
            fn r2(&self);
            fn r3(&self);
            fn r4(&self);
            fn r5(&self);
            fn r6(&self);
        }
    );


    // The components that gathers royalties. Need separate components to charge dynamic royalties,
    // based on the known average execution cost of the `callback` and `on_error` handlers.
    //
    const C0: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1czmuevtw7kmj6p95udqzpjk6legf4ddxklkh8pcfrgvqsza8wq8r0y");
    const C1: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cqdkuneammh0y7s8a4lqylf9534cdnpumwhz80nkz9hkf9d6eufcnc");
    const C2: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cqsch2aae5wa03n37ss9wpkc6vpc3452edw9gvgc2xwjy20dn5fjv8");
    const C3: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cqmrd7yfz09h8ljss5h5h7c762zrwfrz537gsvtuxrtwldlpsjrnfg");
    const C4: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cpztduguv00pfsavzv0d2vu3yx0lpvtp83qzpdd6mp8wd7g7x4n2d9");
    const C5: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cr87ckeqsrl40474c3p9lvs84ev64tmxjsqeehzeqra7k274jv7t5j");
    const C6: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cqerkat7nv04z26jck4lfklvngvr86zwex6uaqkyn8435v0fukuxhq");
    const C7: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cretnv9nfyertg2fmp2e0sea7m9nu6etxr5ewlngzxp0s53m3g7yd0");
    const C8: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1cr442xkdwuxuwlne787n9xw9srnlphpuz2l0u5uqjyda6njwg6r606");
    const C9: Global<DynamicRoyalties> = global_component!(DynamicRoyalties,
                    "component_rdx1czgep7qujkhrjym5l93esy0v23y7cf46zv8crwjya5r76e736jn2v2");

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
            update_caller_royalties => restrict_to: [watcher];
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

        /// Royalties Level per known caller Component, cents. Should be [0, 60].
        caller_royalties: KeyValueStore<ComponentAddress, u8>,

        /// Callback ID sequence
        callback_seq: u32,
    }


    impl RandomComponent {
        /// Instantiate in Stokenet and Mainnet
        pub fn instantiate(owner_badge: ResourceAddress, watcher_badge: ResourceAddress) -> Global<RandomComponent> {
            return Self::do_instantiate(
                owner_badge, watcher_badge,
                None, None
            );
        }

        /// Instantiate with Component and Badge address reservation - for unit tests
        pub fn instantiate_addr_badge(
            owner_badge: ResourceAddress, watcher_badge: ResourceAddress,
            component_address: GlobalAddressReservation,
            component_badge_address: GlobalAddressReservation,
        ) -> Global<RandomComponent> {
            return Self::do_instantiate(
                owner_badge, watcher_badge,
                Some(component_badge_address),
                Some(component_address),
            );
        }

        fn do_instantiate(owner_badge: ResourceAddress,
                          watcher_badge: ResourceAddress,
                          comp_badge_address: Option<GlobalAddressReservation>,
                          component_addr: Option<GlobalAddressReservation>) -> Global<RandomComponent> {
            let comp_badge = Self::create_component_badge(owner_badge, comp_badge_address);
            debug!("comp_badge: {:?}", comp_badge.resource_address());

            let comp: Owned<RandomComponent> = Self {
                queue: KeyValueStore::new_with_registered_type(),
                vaults: KeyValueStore::new_with_registered_type(),

                badges: Vault::with_bucket(comp_badge),

                callback_seq: 0,

                caller_royalties: KeyValueStore::new_with_registered_type(),
            }.instantiate();
            let mut globalizing = comp
                .prepare_to_globalize(OwnerRole::Fixed(
                    rule!(require(owner_badge))
                ))
                .roles(roles!(
                    admin => rule!(require(owner_badge));
                    watcher => rule!(require(watcher_badge));
                ))
                .enable_component_royalties(component_royalties! {
                    init {
                        request_random => Usd(dec!(0.06)), locked;
                        execute => Free, locked;
                        handle_error => Free, locked;
                        evict => Free, locked;
                        update_caller_royalties => Free, locked;
                        withdraw => Free, locked;
                        withdraw_badges => Free, locked;
                    }
                });
            if component_addr.is_some() {
                globalizing = globalizing.with_address(component_addr.unwrap());
            }
            return globalizing.globalize();
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
                              key: u32, badge_opt: Option<FungibleBucket>, expected_fee: u8) -> u32 {
            debug!("EXEC:RandomComponent::request_random({:?}..{:?}, {:?}, {:?}, {:?}, {:?})", address, method_name, on_error, key, badge_opt, expected_fee);

            self.charge_royalty(&address, expected_fee);

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

        /// Updates the royalties for a specific component.
        /// Called by the off-ledger service to maintain the royalties gained from `request_random()`
        /// at a level matching the TX fees incurred when calling `execute()`.
        ///
        pub fn update_caller_royalties(&mut self, address: ComponentAddress, royalty: u8) {
            assert!(royalty <= 60, "Incorrect Royalty level: {}", royalty);
            self.caller_royalties.insert(address, royalty);
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

        fn charge_royalty(&mut self, address: &ComponentAddress, expected_fee: u8) {
            let option: Option<_> = self.caller_royalties.get(&address);
            let level = if option.is_some() {
                *option.unwrap()
            } else if expected_fee <= 60 {
                expected_fee
            } else {
                6u8 // 1 XRD by default
            };
            if level > 0u8 {
                let component_idx = (level - 1) / 6u8;
                let component = match component_idx {
                    0 => { C0 }
                    1 => { C1 }
                    2 => { C2 }
                    3 => { C3 }
                    4 => { C4 }
                    5 => { C5 }
                    6 => { C6 }
                    7 => { C7 }
                    8 => { C8 }
                    9 => { C9 }
                    _ => { panic!("No FeeAdvance component with idx: {:?}", component_idx) }
                };
                let additional_fee = (level - 1) % 6u8 + 1;
                debug!("EXEC:RandomComponent::charge_royalty(C{:?}.r{:?}() [{:?} cents])", component_idx, additional_fee, level);
                match additional_fee {
                    1 => { component.r1(); }
                    2 => { component.r2(); }
                    3 => { component.r3(); }
                    4 => { component.r4(); }
                    5 => { component.r5(); }
                    6 => { component.r6(); }
                    _ => { /* impossible */ }
                };
            } else {
                debug!("EXEC:RandomComponent::charge_royalty(SKIP)");
            }

        }

    }
}