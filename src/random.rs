use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Callback {
    address: ComponentAddress,
    method_name: String,
    key: u32,
}

#[blueprint]
mod airdrop {
    struct Random {
        queue: KeyValueStore<u32, Callback>,
        id_seq: u32,
    }

    impl Random {
        pub fn instantiate() -> Global<Random> {
            return Self::instantiate_local()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        pub fn instantiate_local() -> Owned<Random> {
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
        pub fn request_random(&mut self, address: ComponentAddress, method_name: String, key: u32) -> u32 {
            self.id_seq += 1;
            let callback_id: u32 = self.id_seq;
            let _ = self.queue.insert(callback_id, Callback { address, method_name, key });
            return callback_id;
        }

        /**
         * Called by the Watcher service. TODO: Will be protected by badges.
         */
        pub fn process(&mut self, id: u32, random_seed: u64) {
            let res = self.queue.remove(&id);
            match res {
                Some(callback) => {
                    let comp: Global<AnyComponent> = Global::from(callback.address);
                    comp.call_raw::<u16>(callback.method_name.as_str(), scrypto_args!(callback.key, random_seed));
                }
                None => return, // TODO: proper error handling
            }
        }
    }
}