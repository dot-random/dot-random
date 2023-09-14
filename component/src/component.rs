use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Callback {
    address: ComponentAddress,
    method_name: String,
    resource: ResourceAddress,
    amount: Decimal,
    key: u32,
    size: u8,
}

#[blueprint]
mod component {
    struct RandomComponent {
        vaults: KeyValueStore<ResourceAddress, Vault>,
        queue: KeyValueStore<u32, Callback>,
        id_seq: u32,
        last_processed_id: u32,
    }

    impl RandomComponent {
        pub fn instantiate() -> Global<RandomComponent> {
            return Self::instantiate_local()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        pub fn instantiate_local() -> Owned<RandomComponent> {
            return Self {
                vaults: KeyValueStore::new(),
                queue: KeyValueStore::new(),
                id_seq: 0,
                last_processed_id: 0,
            }
                .instantiate();
        }

        // fn do_register(&mut self, address: ComponentAddress, method_name: String, vault: Option<Vault>) {
        //     debug!("EXEC:RandomComponent::do_register({:?}, {:?}, {:?})\n", address, method_name, vault);
        //     self.r_seq += 1;
        //     let id = self.r_seq;
        //     let _ = self.registry.insert(id, RandomCaller { address, method_name, vault });
        //     return id;
        // }
        //
        // pub fn register_method(&mut self, address: ComponentAddress, method_name: String, badge: Bucket) -> u16 {
        //     debug!("EXEC:RandomComponent::register_method({:?}, {:?}, {:?})\n", address, method_name, badge);
        //
        //     let vault = Vault::with_bucket(badge);
        //     return self.do_register(address, method_name, vault);
        // }

        /**
         * Called by any external Component.
         * the Caller should also pass a badge that controls access to <method_name>().
         */
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, key: u32, badge: FungibleBucket, size: u8) -> u32 {
            debug!("EXEC:RandomComponent::request_random()\n");
            let resource: ResourceAddress = badge.resource_address();
            let amount: Decimal = badge.amount();
            let vault;
            {
                let opt = self.vaults.get_mut(&resource);
                let bucket = badge.into();
                if let Some(mut v) = opt {
                    v.put(bucket);
                    vault = None;
                } else {
                    vault = Some(Vault::with_bucket(bucket));
                }
            }

            if vault.is_some() {
                self.vaults.insert(resource, vault.unwrap());
            }

            self.id_seq += 1;
            let callback_id: u32 = self.id_seq;
            self.queue.insert(callback_id, Callback { address, method_name, key, resource, amount, size });
            return callback_id;
        }

        /**
         * Called by the Random Watcher off-ledger service. TODO: Will be protected by badges.
         */
        pub fn process(&mut self, random_seed: Vec<u8>) {
            debug!("EXEC:RandomComponent::process({:?}..{:?}, {:?})\n", self.last_processed_id, self.id_seq, random_seed);
            if self.last_processed_id < self.id_seq {
                let id = self.last_processed_id + 1;
                let callback = self.queue.remove(&id).unwrap();
                if callback.amount.is_positive() {
                    let opt = self.vaults.get_mut(&callback.resource);
                    if let Some(mut v) = opt {
                        let bucket = v.take(callback.amount).as_fungible();
                        let comp: Global<AnyComponent> = Global::from(callback.address);
                        comp.call_raw::<u32>(callback.method_name.as_str(), scrypto_args!(callback.key, bucket, random_seed));
                    }
                }
                self.last_processed_id = id;
            }
        }
    }
}