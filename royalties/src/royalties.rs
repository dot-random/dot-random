use scrypto::prelude::*;

#[blueprint]
mod royalties {
    struct FeeAdvances {}

    impl FeeAdvances {
        pub fn instantiate(address_reservation: GlobalAddressReservation,
                           owner_badge_address: ResourceAddress, base_fee: Decimal) {
            let _comp = FeeAdvances {}
                .instantiate()
                .prepare_to_globalize(
                    OwnerRole::Fixed(
                        rule!(require(owner_badge_address))
                    )
                )
                .enable_component_royalties(component_royalties! {
                    init {
                        r1  => Usd(base_fee + dec!(0.01)), locked;
                        r2  => Usd(base_fee + dec!(0.02)), locked;
                        r3  => Usd(base_fee + dec!(0.03)), locked;
                        r4  => Usd(base_fee + dec!(0.04)), locked;
                        r5  => Usd(base_fee + dec!(0.05)), locked;
                        r6  => Usd(base_fee + dec!(0.06)), locked;
                    }
                })
                .with_address(address_reservation)
                .globalize();
        }

        pub fn r1(&self) {}
        pub fn r2(&self) {}
        pub fn r3(&self) {}
        pub fn r4(&self) {}
        pub fn r5(&self) {}
        pub fn r6(&self) {}
    }
}