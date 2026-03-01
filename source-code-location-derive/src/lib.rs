extern crate proc_macro;
use proc_macro::TokenStream;

/// Gets the location in the code it was invoked for the purposes of error
/// handling.
#[proc_macro]
pub fn location(input: TokenStream) -> TokenStream {
    do_location(input, "source_code_location::", "location")
}

/// Same as [`location!`], but for unit tests.
#[proc_macro]
pub fn location_test(input: TokenStream) -> TokenStream {
    do_location(input, "crate::", "tests::location_test")
}

fn do_location(input: TokenStream, pfx: &'static str, self_name: &'static str) -> TokenStream {
    if let Some(token_tree) = input.into_iter().next() {//.expect("no tokens");
        let span = token_tree.span();
        let column = span.column();
        let line = span.line();
        let file = span.file();
        
        format!("{pfx}Location::new(const {{ &\"{file}:{line}:{column}\" }})").parse().expect("formatting failed")
    } else {
        format!("{pfx}{self_name}!(_)").parse().expect("formatting failed")
    }
}
