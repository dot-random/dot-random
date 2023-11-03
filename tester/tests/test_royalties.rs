use scrypto_unit::*;
use transaction::prelude::*;

mod common;


#[test]
fn test_royalties() {
    // Arrange
    let mut test_runner = TestRunnerBuilder::new().build();

    let (_random_env, example_component) = common::deploy_component_and_caller(&mut test_runner);

    // Act
    let receipt = test_runner.execute_manifest_ignoring_fee(
        ManifestBuilder::new()
            .call_method(
                example_component,
                "request_mint",
                manifest_args!(),
            )
            .build(), vec![]);
    receipt.expect_commit_success();

    let royalties = receipt.fee_summary.total_royalty_cost_in_xrd;
    common::assert_equal(dec!(1), royalties, "When unset, total royalties should be 1 XRD");
}
