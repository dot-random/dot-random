use scrypto::prelude::*;
use random::Random;

#[blueprint]
mod example {
    extern_blueprint!(
        "package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqqyqszqgqqqqqqqgpwgs6ac",
        MyRandom as RandomComponent {
            fn request_random(&self, address: ComponentAddress, method_name: String, key: u32, size: u8) -> u32;
        }
    );
    const RNG: Global<RandomComponent> = global_component!(
        RandomComponent,
        "component_sim1crasxvah2fntfgx78qljfyg9k04yajd7wjl36ma8q0nq84pthzr2dg"
    );

    struct ExampleCaller {
        // nft id, e.g. 1-1000
        next_id: u16,
        // all traits (in this demo - just a raw random number) by id
        nfts: KeyValueStore<u16, u32>,
    }

    impl ExampleCaller {
        pub fn instantiate() -> Global<ExampleCaller> {
            return Self {
                next_id: 1,
                nfts: KeyValueStore::new(),
            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        /// Request random mint. Called by the User.
        pub fn request_mint(&mut self) -> u32 {
            debug!("EXEC:ExampleCaller::request_mint()\n");
            /* 1. consume payment for mint here */
            /* ... */

            // 2. assign mint badge to RNG.
            // TODO

            // 3. Request mint
            let nft_id = self.next_id;
            self.next_id += 1;
            // The address of your Component
            let address = Runtime::global_component().address();
            // Method on your component to call back
            let method_name = "do_mint".into();
            // a key that will be sent back to you with the callback
            let key = nft_id.into();
            // how many random bytes you need.
            // getting a random u32 costs 4 bytes, u16 - 2 bytes, u8 or bool - 1 byte.
            // you should request as few bytes as possible, as long as it covers your needs (otherwise it will panic).
            // should be in range [1, 32].
            let size: u8 = 4u8;
            return RNG.request_random(address, method_name, key, size);
        }

        /// Executed by our RandomWatcher off-ledger service (through [RandomComponent]).
        /// "nft_id" here is whatever was sent to RNG.request_random() above.
        pub fn do_mint(&mut self, nft_id: u32, random_seed: Vec<u8>) -> u32 {
            debug!("EXEC:ExampleCaller::do_mint({:?}, {:?})\n", nft_id, random_seed);
            // 1. check permissions - todo.

            // 2. seed the random
            let mut random: Random = Random::new(random_seed.as_slice());
            let random_traits = random.next_int::<u32>();

            self.nfts.insert(nft_id as u16, random_traits);
            // TODO: figure out how to `call_raw()` without return type.
            return nft_id;
        }
    }
}