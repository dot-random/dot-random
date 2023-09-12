use scrypto::prelude::*;
use random::Random;

#[blueprint]
mod example {
    extern_blueprint!(
        "package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqqyqszqgqqqqqqqgpwgs6ac",
        MyRandom as RandomComponent {
            fn request_random(&self, address: ComponentAddress, method_name: String, id: u32) -> u32;
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

        /**
         * Request random mint. Called by the User.
         */
        pub fn request_mint(&mut self) -> u32 {
            debug!("EXEC:ExampleCaller::request_mint()\n");
            /* 1. consume payment for mint here */
            /* ... */

            // 2. assign mint badge to RNG.
            // TODO

            // 3. Request mint
            let nft_id = self.next_id;
            self.next_id += 1;
            return RNG.request_random(Runtime::global_component().address(), "do_mint".into(), nft_id.into());
        }

        pub fn do_mint(&mut self, nft_id: u32, random_seed: u64) -> u32 {
            debug!("EXEC:ExampleCaller::do_mint({:?}, {:?})\n", nft_id, random_seed);
            // 1. check permissions - todo.

            // 2. seed the random
            let rng: Random = Random::new(random_seed);
            let random_traits = rng.next_int32();

            self.nfts.insert(nft_id as u16, random_traits);
            // TODO: figure out how to `call_raw()` without return type.
            return nft_id;
        }
    }
}