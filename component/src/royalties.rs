use scrypto::prelude::*;

#[blueprint]
mod royalties {
    struct FeeAdvances {}

    impl FeeAdvances {
        pub fn instantiate(owner_badge_address: ResourceAddress) -> ComponentAddress {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(<FeeAdvances>::blueprint_id());

            let _comp = FeeAdvances {}
                .instantiate()
                .prepare_to_globalize(
                    OwnerRole::Fixed(
                        rule!(require(owner_badge_address))
                    )
                )
                .enable_component_royalties(component_royalties! {
                    init {
                        dynamic_royalty_1  => Usd(dec!(0.03)), locked; // 0.5 XRD
                        dynamic_royalty_2  => Usd(dec!(0.06)), locked; // 1 XRD
                        dynamic_royalty_3  => Usd(dec!(0.09)), locked; // 1.5 XRD
                        dynamic_royalty_4  => Usd(dec!(0.12)), locked; // 2 XRD
                        dynamic_royalty_5  => Usd(dec!(0.18)), locked; // 3 XRD
                        dynamic_royalty_6  => Usd(dec!(0.24)), locked; // 4 XRD
                        dynamic_royalty_7  => Usd(dec!(0.30)), locked; // 5 XRD
                        dynamic_royalty_8  => Usd(dec!(0.42)), locked; // 7 XRD
                        dynamic_royalty_10 => Usd(dec!(0.60)), locked; // 10 XRD
                    }
                })
                .with_address(address_reservation)
                .globalize();

            return component_address;
        }

        pub fn dynamic_royalty_1(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_1()");
        }

        pub fn dynamic_royalty_2(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_2()");
        }

        pub fn dynamic_royalty_3(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_3()");
        }

        pub fn dynamic_royalty_4(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_4()");
        }

        pub fn dynamic_royalty_5(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_5()");
        }

        pub fn dynamic_royalty_6(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_6()");
        }

        pub fn dynamic_royalty_7(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_7()");
        }

        pub fn dynamic_royalty_8(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_8()");
        }

        pub fn dynamic_royalty_10(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_10()");
        }
    }
}