use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Callback {
    address: ComponentAddress,
    method_name: String,
    on_error: String,
    resource: Option<ResourceAddress>,
    amount: Decimal,
    key: u32,
}

const MAX_BATCH_SIZE: u32 = 10;

#[blueprint]
#[types(u32, Callback, ResourceAddress, Vault)]
mod component {
    struct RandomComponent {
        vaults: KeyValueStore<ResourceAddress, Vault>,
        queue: KeyValueStore<u32, Callback>,

        badge_vault: Vault,

        id_seq: u32,
        last_processed_id: u32,
    }


    impl RandomComponent {
        pub fn instantiate() -> Global<RandomComponent> {
            return Self::instantiate_local(None)
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        /// Instantiate with Component address reservation - for unit tests
        pub fn instantiate_addr(
            address: GlobalAddressReservation,
        ) -> Global<RandomComponent> {
            Self::instantiate_local(None)
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address)
                .globalize()
        }

        /// Instantiate with Component and Badge address reservation - for unit tests
        pub fn instantiate_addr_badge(
            component_address: GlobalAddressReservation,
            badge_address: GlobalAddressReservation,
        ) -> Global<RandomComponent> {
            Self::instantiate_local(Some(badge_address))
                .prepare_to_globalize(OwnerRole::None)
                .with_address(component_address)
                .globalize()
        }

        pub fn instantiate_local(resource_address: Option<GlobalAddressReservation>) -> Owned<RandomComponent> {
            let mut builder = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "A badge presented during the callback execution.", locked;
                    }
                ));

            if resource_address.is_some() {
                builder = builder.with_address(resource_address.unwrap());
            }
            let badge: Bucket = builder
                .mint_initial_supply(100)
                .into();
            debug!("badge_addr:\n{:?}", badge.resource_address() );
            return Self {
                vaults: KeyValueStore::new_with_registered_type(),
                queue: KeyValueStore::new_with_registered_type(),

                badge_vault: Vault::with_bucket(badge),

                id_seq: 0,
                last_processed_id: 0,
            }
                .instantiate();
        }

        /**
         * Called by any external Component.
         * the Caller should also pass a badge that controls access to <method_name>().
         */
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, on_error: String, key: u32, badge: FungibleBucket) -> u32 {
            debug!("EXEC:RandomComponent::request_random()");

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
         * the Caller should protect access to <method_name>() with a badge from [badge_vault].
         */
        pub fn request_random2(&mut self, address: ComponentAddress, method_name: String, on_error: String, key: u32) -> u32 {
            debug!("EXEC:RandomComponent::request_random2()");

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

            let end = self.last_processed_id + MAX_BATCH_SIZE;
            while self.last_processed_id < self.id_seq && self.last_processed_id < end  {
                let id = self.last_processed_id + 1;

                self.do_process(id, random_seed.clone());
                self.last_processed_id = id;
            }
        }

        /**
         * Process a specific callback. Will be used until we can reliably process the whole queue.
         * Also called to preview the execution result (Success/Failure) of a specific Callback.
         */
        pub fn process_one(&mut self, callback_id: u32, random_seed: Vec<u8>) {
            debug!("EXEC:RandomComponent::process_one({:?})", callback_id);
            self.do_process(callback_id, random_seed);
        }

        pub fn handle_error(&mut self, callback_id: u32) {
            debug!("EXEC:RandomComponent::handle_error({:?})", callback_id);
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
                            comp.call_ignore_rtn(callback.on_error.as_str(), &(callback.key, bucket));
                        }
                    }
                } else {
                    let proof = self.badge_vault.as_fungible().create_proof_of_amount(Decimal::ONE);
                    proof.authorize(|| {
                        let comp: Global<AnyComponent> = Global::from(callback.address);
                        comp.call_ignore_rtn(callback.on_error.as_str(), &(callback.key));
                    });
                }
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
                    let proof = self.badge_vault.as_fungible().create_proof_of_amount(Decimal::ONE);
                    proof.authorize(|| {
                        let comp: Global<AnyComponent> = Global::from(callback.address);
                        comp.call_ignore_rtn(callback.method_name.as_str(), &(callback.key, random_seed));
                    });
                }
            }
        }

        /// Evicts a faulty callback from the queue.
        /// A callback is considered faulty when both <method_name> and <on_error> panic during the simulation.
        pub fn evict(&mut self, callback_id: u32) {
            self.queue.remove(&callback_id);
        }
    }
}