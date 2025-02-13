use crate::zwang_router::TEST_STR;

use super::*;

#[test]
fn test_converting_to_main_model_parts() -> Result<()> {
    let parsed: parsing::Part = parse_str(TEST_STR).expect("Unable to parse routes");
    let main_model_parts = Parts::try_from(parsed)?;
    println!("{:#?}", main_model_parts);
    Ok(())
}
