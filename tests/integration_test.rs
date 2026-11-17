//! Integration tests for SigmaOS-distribution-testing-infrastructure.

use sigmaos_distribution_testing_infrastructure_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
