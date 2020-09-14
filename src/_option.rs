pub fn test_option() {
  let some_number = Some(5);
  let some_string = Some("a string");

  let option: Option<String> = None;

  assert_eq!(some_number.is_some(), true);
  assert_eq!(some_string.is_some(), true);
  assert_eq!(option.is_none(), true);
}