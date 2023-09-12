use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Callback {
    address: ComponentAddress,
    method_name: String,
    key: u32,
    size: u8,
}

#[blueprint]
mod component {
    struct RandomComponent {
        queue: KeyValueStore<u32, Callback>,
        id_seq: u32,
    }

    impl RandomComponent {
        pub fn instantiate() -> Global<RandomComponent> {
            return Self::instantiate_local()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        pub fn instantiate_local() -> Owned<RandomComponent> {
            return Self {
                queue: KeyValueStore::new(),
                id_seq: 0,
            }
                .instantiate();
        }

        /**
         * Called by any external Component.
         * the Caller should also pass a badge that controls access to <method_name>().
         */
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, key: u32, size: u8) -> u32 {
            debug!("EXEC:RandomComponent::request_random()\n");
            self.id_seq += 1;
            let callback_id: u32 = self.id_seq;
            let _ = self.queue.insert(callback_id, Callback { address, method_name, key, size });
            return callback_id;
        }

        /**
         * Called by the Random Watcher off-ledger service. TODO: Will be protected by badges.
         */
        pub fn process(&self, id: u32, random_seed: Vec<u8>) {
            debug!("EXEC:RandomComponent::process({:?}, {:?})\n", id, random_seed);
            let res = self.queue.remove(&id);
            match res {
                Some(callback) => {
                    let comp: Global<AnyComponent> = Global::from(callback.address);
                    comp.call_raw::<u32>(callback.method_name.as_str(), scrypto_args!(callback.key, random_seed));
                }
                None => return, // TODO: proper error handling
            }
        }
    }
}