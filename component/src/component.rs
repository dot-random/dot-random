use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Callback {
    address: ComponentAddress,
    method_name: String,
    resource: Option<ResourceAddress>,
    amount: Decimal,
    key: u32,
    size: u8,
}

#[blueprint]
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
            return Self::instantiate_local()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        /// Instantiate with address reservation - for unit tests
        pub fn instantiate_addr(
            address: GlobalAddressReservation,
        ) -> Global<RandomComponent> {
            Self::instantiate_local()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address)
                .globalize()
        }

        pub fn instantiate_local() -> Owned<RandomComponent> {
            let badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "A badge presented during the callback execution.", locked;
                    }
                ))
                .mint_initial_supply(100)
                .into();
            let badge_addr: ResourceAddress = badge.resource_address();
            debug!("badge_addr:\n{:?}\n", badge_addr );
            return Self {
                vaults: KeyValueStore::new(),
                queue: KeyValueStore::new(),

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
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, key: u32, badge: FungibleBucket, size: u8) -> u32 {
            debug!("EXEC:RandomComponent::request_random()\n");
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
            self.queue.insert(callback_id, Callback { address, method_name, key, resource, amount, size });
            return callback_id;
        }

        /**
         * Called by any external Component.
         * the Caller should protect access to <method_name>() with a badge from [badge_vault].
         */
        pub fn request_random2(&mut self, address: ComponentAddress, method_name: String, key: u32, size: u8) -> u32 {
            debug!("EXEC:RandomComponent::request_random2()\n");

            self.id_seq += 1;
            let callback_id: u32 = self.id_seq;
            let resource = None;
            let amount = Decimal::ZERO;
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
                let resource_opt = callback.resource;
                if let Some(resource) = resource_opt {
                    if callback.amount.is_positive() {
                        let opt = self.vaults.get_mut(&resource);
                        if let Some(mut v) = opt {
                            let bucket = v.take(callback.amount).as_fungible();
                            let comp: Global<AnyComponent> = Global::from(callback.address);
                            comp.call_raw::<u32>(callback.method_name.as_str(), scrypto_args!(callback.key, bucket, random_seed));
                        }
                    }
                } else {
                    let proof = self.badge_vault.as_fungible().create_proof_of_amount(Decimal::ONE);
                    proof.authorize(|| {
                        let comp: Global<AnyComponent> = Global::from(callback.address);
                        comp.call_raw::<u32>(callback.method_name.as_str(), scrypto_args!(callback.key, random_seed));
                    });
                }

                self.last_processed_id = id;
            }
        }
    }
}