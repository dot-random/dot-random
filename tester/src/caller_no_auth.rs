use scrypto::prelude::*;
use random::Random;

#[blueprint]
mod caller_no_auth {
    extern_blueprint!(
        // "package_tdx_2_1pk56nm7yuy3dcjx6awtj72ykx5grte0vukd0j8vl8algxnphwe8yz7",
        "package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnnzj0hj",
        MyRandom as RandomComponent {
            fn request_random(&self, address: ComponentAddress, method_name: String, on_error: String,
                key: u32, badge_opt: Option<FungibleBucket>, expected_fee: u8) -> u32;
        }
    );

    const RNG: Global<RandomComponent> = global_component!(
        RandomComponent,
        // "component_tdx_2_1czgsfkdazhyhrs5238wh5phfk80ky8xzqvjwf7cpxwu76efl9jehcx"
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
            debug!("EXEC:ExampleCallerNoAuth::instantiate()");

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
            debug!("EXEC:ExampleCallerNoAuth::request_mint()");
            /* 1. consume payment for mint here */
            /* ... */

            // 2. Request Random
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
            // The auth badge. Will be returned fully with the callback
            let badge_opt: Option<FungibleBucket> = None;
            // How much you would expect the callback to cost, cents (e.g. test on Stokenet).
            // It helps to avoid a sharp increase in royalties during the first few invocations of `request_random()`
            // but is completely optional.
            let expected_fee = 0u8;

            return RNG.request_random(address, method_name, on_error, key, badge_opt, expected_fee);
        }

        /// Executed by our RandomWatcher off-ledger service (through [RandomComponent]).
        /// "nft_id" here is whatever was sent to RNG.request_random() above.
        pub fn do_mint(&mut self, nft_id: u32, random_seed: Vec<u8>) {
            debug!("EXEC:ExampleCallerNoAuth::do_mint({:?}, {:?})", nft_id, random_seed);
            // 2. seed the random
            let mut random: Random = Random::new(&random_seed);
            let random_traits = random.next_int::<u32>();

            self.nfts.insert(nft_id as u16, random_traits);
        }

        pub fn abort_mint(&mut self, _nft_id: u32) {
            // revert what you did in `request_mint()` here,
            // e.g. send the payment back to the user
        }
    }
}

