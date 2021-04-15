use crate::datapoint;
use std::error::Error;

#[test]
fn test_from_string() -> Result<(), Box<dyn Error>> {
    let dp = datapoint::Datapoint::from_string("my.metric.test 1 1618430311");
    let expected_dp = datapoint::Datapoint::new("my.metric.test".to_string(), 1, 1618430311);
    assert_eq!(dp?, expected_dp);
    return Ok(());
}

#[test]
#[should_panic]
fn test_invalid_from_string() {
    let _ = datapoint::Datapoint::from_string("my.metric.test 1 aaa").unwrap();
}
