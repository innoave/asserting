use super::*;
use crate::recursive_comparison::value::proptest_support::*;
use proptest::prelude::*;

#[test]
fn type_name_of_empty_map() {
    let map = Map::new();

    assert_eq!(map.type_name(), "Map<Value, Value>");
}

proptest! {
    #[test]
    fn type_name_of_map_with_one_entry(
        key in any_value(),
        value in any_value(),
    ) {
        let key_type = key.type_name();
        let value_type = value.type_name();
        let map = Map::from_iter([(key, value)]);

        assert_eq!(map.type_name(), format!("Map<{key_type}, {value_type}>"));
    }
}
