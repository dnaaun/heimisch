use super::*;
use crate::zwang_url;

#[test]
fn basic_url_construction() {
    let expr: syn::LitStr = parse_quote! { "/owner_name=hi/repo_name=hello/issues/new" };
    let url = zwang_url(expr);
    println!("{}", url);
}
