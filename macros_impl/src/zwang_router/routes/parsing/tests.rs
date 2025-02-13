use crate::zwang_router::TEST_STR;

use super::*;
use syn::parse_str;

#[test]
fn test_parse_routes_with_fallback() {
    let parsed: Part = parse_str(TEST_STR).expect("Unable to parse routes");
    println!("{parsed:#?}");
}
