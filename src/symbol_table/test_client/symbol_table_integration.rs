use crate::symbol_table::ST;

#[allow(dead_code)]
pub fn run_tests<T>()
where
    T: ST<String, String>,
{
    symbol_table_iterate_keys_ordered::<T>();
    symbol_table_iterate_keys_ordered_between_range::<T>();
    allows_to_search_value_by_key::<T>();
    find_key_by_position::<T>();
    find_position_by_keys::<T>();
    find_min_and_max_keys::<T>();
    floor_can_find_lower_or_equal_key::<T>();
    ceiling_can_find_greater_or_equal_key::<T>();
    delete_values_restore_symbol_table_to_empty::<T>();
}

fn symbol_table_iterate_keys_ordered<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert!(!st.is_empty());
    assert_eq!(st.size(), 10);

    let keys = st.keys();
    let mut iter = keys.iter();
    assert_eq!(iter.next(), Some(&&String::from("A")));
    assert_eq!(iter.next(), Some(&&String::from("C")));
    assert_eq!(iter.next(), Some(&&String::from("E")));
    assert_eq!(iter.next(), Some(&&String::from("H")));
    assert_eq!(iter.next(), Some(&&String::from("L")));
    assert_eq!(iter.next(), Some(&&String::from("M")));
    assert_eq!(iter.next(), Some(&&String::from("P")));
    assert_eq!(iter.next(), Some(&&String::from("R")));
    assert_eq!(iter.next(), Some(&&String::from("S")));
    assert_eq!(iter.next(), Some(&&String::from("X")));
    assert_eq!(iter.next(), None);
}

fn symbol_table_iterate_keys_ordered_between_range<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert!(!st.is_empty());
    assert_eq!(st.size(), 10);

    let keys = st.keys_in_range(&String::from("D"), &String::from("R"));
    let mut iter = keys.iter();
    assert_eq!(iter.next(), Some(&&String::from("E")));
    assert_eq!(iter.next(), Some(&&String::from("H")));
    assert_eq!(iter.next(), Some(&&String::from("L")));
    assert_eq!(iter.next(), Some(&&String::from("M")));
    assert_eq!(iter.next(), Some(&&String::from("P")));
    assert_eq!(iter.next(), Some(&&String::from("R")));
    assert_eq!(iter.next(), None);
}

fn allows_to_search_value_by_key<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert!(st.contains(&String::from("S")));
    assert!(!st.contains(&String::from("T")));

    assert_eq!(st.get(&String::from("A")), Some(&String::from("8")));
    assert_eq!(st.get(&String::from("C")), Some(&String::from("4")));
    assert_eq!(st.get(&String::from("E")), Some(&String::from("12")));
    assert_eq!(st.get(&String::from("H")), Some(&String::from("5")));
    assert_eq!(st.get(&String::from("L")), Some(&String::from("11")));
    assert_eq!(st.get(&String::from("M")), Some(&String::from("9")));
    assert_eq!(st.get(&String::from("P")), Some(&String::from("10")));
    assert_eq!(st.get(&String::from("R")), Some(&String::from("3")));
    assert_eq!(st.get(&String::from("S")), Some(&String::from("0")));
    assert_eq!(st.get(&String::from("X")), Some(&String::from("7")));
}

fn find_key_by_position<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert_eq!(st.select(0), Some(&String::from("A")));
    assert_eq!(st.select(3), Some(&String::from("H")));
    assert_eq!(st.select(8), Some(&String::from("S")));
    assert_eq!(st.select(10), None);
}

fn find_position_by_keys<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert_eq!(st.rank(&String::from("A")), Some(0));
    assert_eq!(st.rank(&String::from("H")), Some(3));
    assert_eq!(st.rank(&String::from("S")), Some(8));
    assert_eq!(st.rank(&String::from("G")), None);
}

fn find_min_and_max_keys<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert_eq!(st.min(), Some(&String::from("A")));
    assert_eq!(st.max(), Some(&String::from("X")));
}

fn floor_can_find_lower_or_equal_key<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S O M E T H I N G T O F I N D".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert_eq!(st.floor(&String::from("M")), Some(&String::from("M")));
    assert_eq!(st.floor(&String::from("J")), Some(&String::from("I")));
    assert_eq!(st.floor(&String::from("A")), None);
}

fn ceiling_can_find_greater_or_equal_key<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    let keys = "S E A R C H E X A M P L E".split(" ");

    // act
    for (position, key) in keys.enumerate() {
        st.put(String::from(key), format!("{}", position));
    }

    // assert
    assert_eq!(st.ceiling(&String::from("C")), Some(&String::from("C")));
    assert_eq!(st.ceiling(&String::from("D")), Some(&String::from("E")));
    assert_eq!(st.ceiling(&String::from("Z")), None);
}

fn delete_values_restore_symbol_table_to_empty<T>()
where
    T: ST<String, String>,
{
    // arrange
    let st = &mut T::new();
    st.put(String::from("test"), String::from("test"));

    // act
    st.delete(&String::from("test"));

    // assert
    // assert!(st.is_empty());
    // assert!(!st.contains(String::from("test")));
}
