use super::*;

#[test]
fn generate_index_for_nothing_writes_nothing() {
    let mut inp = String::from("");
    let mut out = Vec::<u8>::new();
    // Generate the index for empty content, make sure it returns `Ok`
    assert!(generate_index(&mut inp.as_bytes(), &mut out).is_ok());
    // MAke sure the output did not get written onto:
    assert!(out.is_empty());
}
