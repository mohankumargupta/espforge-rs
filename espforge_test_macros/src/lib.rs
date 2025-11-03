// /espforge_test_macros/src/lib.rs

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Error, FnArg, ItemFn, LitStr, Pat, Type};

/// Attribute macro for CLI integration tests
///
/// # Example
/// ```ignore
/// #[cli_test("examples/blink.toml")]
/// fn test_blink_compilation(output: Output) {
///     use predicates::prelude::*;
///     output
///         .assert_file("output.txt")
///         .assert(predicate::str::contains("expected content"));
/// }
/// ```
#[proc_macro_attribute]
pub fn cli_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let config_path = parse_macro_input!(attr as LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);

    expand_cli_test(&config_path, &input_fn)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn expand_cli_test(config_path: &LitStr, input_fn: &ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    // Validate and extract the function parameter
    let output_param = validate_and_extract_param(input_fn)?;

    let fn_name = &input_fn.sig.ident;
    let body = &input_fn.block;
    let config_path_str = config_path.value();

    // Generate a unique struct name to avoid conflicts
    // Convert snake_case function name to UpperCamelCase
    let fn_name_str = fn_name.to_string();
    let camel_case_name = fn_name_str
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>();
    
    let output_struct_name = syn::Ident::new(
        &format!("Output{}", camel_case_name),
        fn_name.span()
    );

    // The entire test logic is generated here.
    // All necessary `use` statements are included inside the `quote!` block
    // to solve macro hygiene issues.
    Ok(quote! {
        /// Helper struct providing assertion utilities for CLI test output
        struct #output_struct_name<'a> {
            temp: &'a assert_fs::TempDir,
        }

        impl<'a> #output_struct_name<'a> {
            /// Returns a `ChildPath` for a file within the temporary directory.
            /// This type from `assert_fs` can be used directly for assertions.
            fn assert_file<P: AsRef<::std::path::Path>>(&self, path: P) -> assert_fs::fixture::ChildPath {
                use assert_fs::{fixture::PathChild, assert::PathAssert};
                use predicates::prelude::*;

                // let child = self.temp.child(path);
                // child.assert(predicate::path::exists());
                let child = self.temp.child(&path);
                let path_str = path.as_ref().display().to_string();
                if !predicate::path::exists().eval(child.path()) {
                    panic!("Expected path to exist, but was missing: {}", child.path().display());
                }          
                child
            }

            /// Get the path to the temporary directory
            fn path(&self) -> &std::path::Path {
                self.temp.path()
            }
        }

        #[test]
        fn #fn_name() -> Result<(), anyhow::Error> {
            use assert_cmd::{Command, prelude::*, pkg_name};
            use assert_fs::{assert::PathAssert, fixture::{PathChild, ChildPath}};
            use predicates::prelude::*;
            use std::{fs, path::PathBuf};

            // Arrange: Set up temporary directory and configuration
            let temp = assert_fs::TempDir::new()?;
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let configuration_path = manifest_dir.join(#config_path_str);
            
            if !configuration_path.exists() {
                anyhow::bail!(
                    "Configuration file not found: {}",
                    configuration_path.display()
                );
            }

            let temp_config = temp.child("config.toml");
            fs::copy(&configuration_path, temp_config.path())?;

            // Act: Execute the CLI command
            let mut cmd = Command::new(pkg_name!());
            cmd.current_dir(temp.path())
               .arg("compile")
               .arg(temp_config.path());
            
            cmd.assert().success();

            // Create the output helper struct for the user's test
            let #output_param = #output_struct_name { temp: &temp };

            // Execute the user's test body
            #body

            Ok(())
        }
    })
}

fn validate_and_extract_param(input_fn: &ItemFn) -> syn::Result<&Pat> {
    // Ensure function takes exactly one parameter
    if input_fn.sig.inputs.len() != 1 {
        return Err(Error::new_spanned(
            &input_fn.sig,
            "cli_test functions must take exactly one parameter of type Output"
        ));
    }

    // Extract the parameter pattern
    let param = input_fn.sig.inputs.first().unwrap();
    
    match param {
        FnArg::Typed(pat_type) => {
            // Optionally validate the type is "Output"
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident != "Output" {
                        return Err(Error::new_spanned(
                            &pat_type.ty,
                            "parameter must be of type Output"
                        ));
                    }
                }
            }
            Ok(&pat_type.pat)
        }
        FnArg::Receiver(_) => {
            Err(Error::new_spanned(
                param,
                "cli_test functions cannot have self parameter"
            ))
        }
    }
}

