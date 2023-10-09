use scrypto::prelude::*;

#[blueprint]
mod royalties {
    struct FeeAdvances {}

    impl FeeAdvances {
        pub fn instantiate(owner_badge_address: ResourceAddress) -> Global<FeeAdvances> {
            let (address_reservation, _) =
                Runtime::allocate_component_address(<FeeAdvances>::blueprint_id());

            let comp = FeeAdvances {}
                .instantiate()
                .prepare_to_globalize(
                    OwnerRole::Fixed(
                        rule!(require(owner_badge_address))
                    )
                )
                .enable_component_royalties(component_royalties! {
                    init {
                        dynamic_royalty_1 => Usd(dec!(0.01)), locked;
                        dynamic_royalty_2 => Usd(dec!(0.02)), locked;
                        dynamic_royalty_3 => Usd(dec!(0.03)), locked;
                        dynamic_royalty_4 => Usd(dec!(0.04)), locked;
                        dynamic_royalty_5 => Usd(dec!(0.05)), locked;
                        dynamic_royalty_6 => Usd(dec!(0.06)), locked;
                        dynamic_royalty_7 => Usd(dec!(0.07)), locked;
                        dynamic_royalty_8 => Usd(dec!(0.08)), locked;
                        dynamic_royalty_9 => Usd(dec!(0.09)), locked;
                        dynamic_royalty_10 => Usd(dec!(0.10)), locked;
                        dynamic_royalty_11 => Usd(dec!(0.11)), locked;
                        dynamic_royalty_12 => Usd(dec!(0.12)), locked;
                        dynamic_royalty_13 => Usd(dec!(0.13)), locked;
                        dynamic_royalty_14 => Usd(dec!(0.14)), locked;
                        dynamic_royalty_15 => Usd(dec!(0.15)), locked;
                        dynamic_royalty_16 => Usd(dec!(0.16)), locked;
                        dynamic_royalty_17 => Usd(dec!(0.17)), locked;
                        dynamic_royalty_18 => Usd(dec!(0.18)), locked;
                        dynamic_royalty_19 => Usd(dec!(0.19)), locked;
                        dynamic_royalty_20 => Usd(dec!(0.20)), locked;
                        dynamic_royalty_21 => Usd(dec!(0.21)), locked;
                        dynamic_royalty_22 => Usd(dec!(0.22)), locked;
                        dynamic_royalty_23 => Usd(dec!(0.23)), locked;
                        dynamic_royalty_24 => Usd(dec!(0.24)), locked;
                        dynamic_royalty_25 => Usd(dec!(0.25)), locked;
                        dynamic_royalty_26 => Usd(dec!(0.26)), locked;
                        dynamic_royalty_27 => Usd(dec!(0.27)), locked;
                        dynamic_royalty_28 => Usd(dec!(0.28)), locked;
                        dynamic_royalty_29 => Usd(dec!(0.29)), locked;
                        dynamic_royalty_30 => Usd(dec!(0.30)), locked;
                        dynamic_royalty_31 => Usd(dec!(0.31)), locked;
                        dynamic_royalty_32 => Usd(dec!(0.32)), locked;
                        dynamic_royalty_33 => Usd(dec!(0.33)), locked;
                        dynamic_royalty_34 => Usd(dec!(0.34)), locked;
                        dynamic_royalty_35 => Usd(dec!(0.35)), locked;
                        dynamic_royalty_36 => Usd(dec!(0.36)), locked;
                        dynamic_royalty_37 => Usd(dec!(0.37)), locked;
                        dynamic_royalty_38 => Usd(dec!(0.38)), locked;
                        dynamic_royalty_39 => Usd(dec!(0.39)), locked;
                        dynamic_royalty_40 => Usd(dec!(0.40)), locked;
                        dynamic_royalty_41 => Usd(dec!(0.41)), locked;
                        dynamic_royalty_42 => Usd(dec!(0.42)), locked;
                        dynamic_royalty_43 => Usd(dec!(0.43)), locked;
                        dynamic_royalty_44 => Usd(dec!(0.44)), locked;
                        dynamic_royalty_45 => Usd(dec!(0.45)), locked;
                        dynamic_royalty_46 => Usd(dec!(0.46)), locked;
                        dynamic_royalty_47 => Usd(dec!(0.47)), locked;
                        dynamic_royalty_48 => Usd(dec!(0.48)), locked;
                        dynamic_royalty_49 => Usd(dec!(0.49)), locked;
                        dynamic_royalty_50 => Usd(dec!(0.50)), locked;
                        dynamic_royalty_51 => Usd(dec!(0.51)), locked;
                        dynamic_royalty_52 => Usd(dec!(0.52)), locked;
                        dynamic_royalty_53 => Usd(dec!(0.53)), locked;
                        dynamic_royalty_54 => Usd(dec!(0.54)), locked;
                        dynamic_royalty_55 => Usd(dec!(0.55)), locked;
                        dynamic_royalty_56 => Usd(dec!(0.56)), locked;
                        dynamic_royalty_57 => Usd(dec!(0.57)), locked;
                        dynamic_royalty_58 => Usd(dec!(0.58)), locked;
                        dynamic_royalty_59 => Usd(dec!(0.59)), locked;
                        dynamic_royalty_60 => Usd(dec!(0.60)), locked;
                    }
                })
                .with_address(address_reservation)
                .globalize();

            return comp;
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

        pub fn dynamic_royalty_9(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_9()");
        }

        pub fn dynamic_royalty_10(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_10()");
        }

        pub fn dynamic_royalty_11(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_11()");
        }

        pub fn dynamic_royalty_12(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_12()");
        }

        pub fn dynamic_royalty_13(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_13()");
        }

        pub fn dynamic_royalty_14(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_14()");
        }

        pub fn dynamic_royalty_15(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_15()");
        }

        pub fn dynamic_royalty_16(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_16()");
        }

        pub fn dynamic_royalty_17(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_17()");
        }

        pub fn dynamic_royalty_18(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_18()");
        }

        pub fn dynamic_royalty_19(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_19()");
        }

        pub fn dynamic_royalty_20(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_20()");
        }

        pub fn dynamic_royalty_21(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_21()");
        }

        pub fn dynamic_royalty_22(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_22()");
        }

        pub fn dynamic_royalty_23(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_23()");
        }

        pub fn dynamic_royalty_24(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_24()");
        }

        pub fn dynamic_royalty_25(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_25()");
        }

        pub fn dynamic_royalty_26(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_26()");
        }

        pub fn dynamic_royalty_27(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_27()");
        }

        pub fn dynamic_royalty_28(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_28()");
        }

        pub fn dynamic_royalty_29(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_29()");
        }

        pub fn dynamic_royalty_30(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_30()");
        }

        pub fn dynamic_royalty_31(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_31()");
        }

        pub fn dynamic_royalty_32(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_32()");
        }

        pub fn dynamic_royalty_33(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_33()");
        }

        pub fn dynamic_royalty_34(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_34()");
        }

        pub fn dynamic_royalty_35(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_35()");
        }

        pub fn dynamic_royalty_36(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_36()");
        }

        pub fn dynamic_royalty_37(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_37()");
        }

        pub fn dynamic_royalty_38(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_38()");
        }

        pub fn dynamic_royalty_39(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_39()");
        }

        pub fn dynamic_royalty_40(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_40()");
        }

        pub fn dynamic_royalty_41(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_41()");
        }

        pub fn dynamic_royalty_42(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_42()");
        }

        pub fn dynamic_royalty_43(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_43()");
        }

        pub fn dynamic_royalty_44(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_44()");
        }

        pub fn dynamic_royalty_45(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_45()");
        }

        pub fn dynamic_royalty_46(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_46()");
        }

        pub fn dynamic_royalty_47(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_47()");
        }

        pub fn dynamic_royalty_48(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_48()");
        }

        pub fn dynamic_royalty_49(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_49()");
        }

        pub fn dynamic_royalty_50(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_50()");
        }

        pub fn dynamic_royalty_51(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_51()");
        }

        pub fn dynamic_royalty_52(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_52()");
        }

        pub fn dynamic_royalty_53(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_53()");
        }

        pub fn dynamic_royalty_54(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_54()");
        }

        pub fn dynamic_royalty_55(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_55()");
        }

        pub fn dynamic_royalty_56(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_56()");
        }

        pub fn dynamic_royalty_57(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_57()");
        }

        pub fn dynamic_royalty_58(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_58()");
        }

        pub fn dynamic_royalty_59(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_59()");
        }

        pub fn dynamic_royalty_60(&self) {
            debug!("EXEC:FeeAdvances::dynamic_royalty_60()");
        }
    }
}