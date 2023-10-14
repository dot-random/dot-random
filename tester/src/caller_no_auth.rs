use scrypto::prelude::*;

use random::Random;

#[blueprint]
mod caller_no_auth {
    extern_blueprint!(
        // "package_tdx_2_1pk56nm7yuy3dcjx6awtj72ykx5grte0vukd0j8vl8algxnphwe8yz7",
        "package_sim1p5qqqqqqqyqszqgqqqqqqqgpqyqsqqqqxumnwqgqqqqqqycnnzj0hj",
        RandomComponent {
            fn register_caller(&self, address: ComponentAddress, method_name: String, on_error: String,
                               bucket_resource: Option<ResourceAddress>, royalties_level: u8) -> u16;
            fn request_random(&self, caller_id: u16, key: u32, badge_opt: Option<FungibleBucket>) -> u32;
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
        // Caller ID.
        caller_id: u16,
    }

    impl ExampleCallerNoAuth {
        pub fn instantiate() -> Global<ExampleCallerNoAuth> {
            debug!("EXEC:ExampleCallerNoAuth::instantiate()");

            // Prepare the address of your Component
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(<ExampleCallerNoAuth>::blueprint_id());
            // The method on your component to call back
            let method_name: String = "do_mint".into();
            // The method on yor component that will be called if do_mint() panics
            let on_error: String = "abort_mint".into();
            // The token that you are going to send in `request_random()`. None if you send None there.
            let bucket_resource: Option<ResourceAddress> = None;
            // Royalties level.
            let royalties_level: u8 = 0;

            // Register your Component and obtain a `caller_id`.
            let caller_id = RNG.register_caller(component_address, method_name, on_error, bucket_resource, royalties_level);

            return Self {
                next_id: 1,
                nfts: KeyValueStore::new(),
                // store the Caller ID. It doesn't change unless you upgrade to a new version of the RandomComponent (upgrade TBD).
                caller_id
            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address_reservation)
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
            // A key that will be sent back to you with the callback
            let key = nft_id.into();

            return RNG.request_random(self.caller_id, key, None);
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

