use scrypto::prelude::*;
use random::Random;

#[blueprint]
mod caller_no_auth {
    extern_blueprint!(
        // "package_tdx_e_1p5jt8mth9qfzaj4x6ned23dcss6wudjgxgwyr5uc6f6gja9gwz4nxh",
        "package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnnzj0hj",
        MyRandom as RandomComponent {
            fn request_random2(&self, address: ComponentAddress, method_name: String, on_error: String, key: u32) -> u32;
        }
    );

    const RNG: Global<RandomComponent> = global_component!(
        RandomComponent,
        // "component_tdx_e_1cpdq9kvkhnv2yp53zylvlmq74hp90263hxa3zxewxtsxmpdwqhvsxa"
        "component_sim1cqqqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnf7v0gx"
    );

    struct ExampleCallerNoAuth {
        // nft id, e.g. 1-1000
        next_id: u16,
        // all traits (in this demo - just a raw random number) by id
        nfts: KeyValueStore<u16, u32>,
    }

    impl ExampleCallerNoAuth {
        pub fn instantiate() -> Global<ExampleCallerNoAuth> {
            debug!("EXEC:ExampleCallerNoAuth::instantiate()\n");

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
            debug!("EXEC:ExampleCallerNoAuth::request_mint()\n");
            /* 1. consume payment for mint here */
            /* ... */

            // 2. Request mint
            let nft_id = self.next_id;
            self.next_id += 1;
            // The address of your Component
            let address = Runtime::global_component().address();
            // The method on your component to call back
            let method_name = "do_mint".into();
            // The method on yor component that will be called if do_mint() panics
            let on_error = "abort_mint".into();
            // A key that will be sent back to you with the callback
            let key = nft_id.into();

            return RNG.request_random2(address, method_name, on_error, key);
        }

        /// Executed by our RandomWatcher off-ledger service (through [RandomComponent]).
        /// "nft_id" here is whatever was sent to RNG.request_random() above.
        pub fn do_mint(&mut self, nft_id: u32, random_seed: Vec<u8>) {
            debug!("EXEC:ExampleCallerNoAuth::do_mint({:?}, {:?})\n", nft_id, random_seed);
            // 2. seed the random
            let mut random: Random = Random::new(&random_seed);
            let random_traits = random.next_int::<u32>();

            self.nfts.insert(nft_id as u16, random_traits);
        }

        pub fn abort_mint(&mut self, nft_id: u32) {
            // revert what you did in `request_mint()` here
        }
    }
}

