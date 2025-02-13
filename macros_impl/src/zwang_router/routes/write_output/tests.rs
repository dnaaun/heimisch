use main_model::Parts;

use crate::zwang_router::{parsing, TEST_STR};

use super::*;

#[test]
fn test_writing_output() -> Result<()> {
    let parsed: parsing::Part = parse_str(TEST_STR).expect("Unable to parse routes");
    let main_model_parts = Parts::try_from(parsed)?;
    let output = write_output(main_model_parts).unwrap().to_string();
    // println!("\n\n{output}\n\n");
    println!(
        "\n\n{}\n\n",
        prettyplease::unparse(&syn::parse_file(&output).unwrap())
    );
    Ok(())
}
