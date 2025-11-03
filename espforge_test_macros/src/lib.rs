use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Error, FnArg, ItemFn, LitStr, Pat, Type};

/// Attribute macro for CLI integration tests
/// 
/// # Example
/// ```ignore
/// #[cli_test("examples/blink.toml")]
/// fn test_blink_compilation(output: Output) {
///     output.assert_file("output.txt").contains("expected content");
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

    Ok(quote! {
        #[test]
        fn #fn_name() -> Result<(), anyhow::Error> {
            use assert_cmd::{Command, prelude::*, pkg_name};
            use assert_fs::{assert::PathAssert, fixture::PathChild};
            use predicates::prelude::*;
            use std::{fs, path::PathBuf};

            // Arrange: Set up temporary directory and configuration
            let temp = assert_fs::TempDir::new()?;
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let configuration_path = manifest_dir.join(#config_path_str);
            
            // Validate config file exists
            if !configuration_path.exists() {
                anyhow::bail!(
                    "Configuration file not found: {}",
                    configuration_path.display()
                );
            }

            let temp_config = temp.child("config.toml");
            fs::copy(&configuration_path, temp_config.path())
                .map_err(|e| anyhow::anyhow!(
                    "Failed to copy config from {} to {}: {}",
                    configuration_path.display(),
                    temp_config.path().display(),
                    e
                ))?;

            // Act: Execute the CLI command
            let mut cmd = Command::new(pkg_name!());
            cmd.current_dir(temp.path())
               .arg("compile")
               .arg(temp_config.path());
            
            cmd.assert().success();

            // Create output helper
            let #output_param = Output { temp: &temp };

            // Execute user's test body
            #body

            Ok(())
        }

        /// Helper struct providing assertion utilities for CLI test output
        pub struct Output<'a> {
            temp: &'a assert_fs::TempDir,
        }

        impl<'a> Output<'a> {
            /// Assert on a file within the temporary directory
            pub fn assert_file<P: AsRef<std::path::Path>>(&self, path: P) -> FileAssert {
                FileAssert {
                    path: self.temp.child(path),
                }
            }

            /// Get the path to the temporary directory
            pub fn path(&self) -> &std::path::Path {
                self.temp.path()
            }
        }

        /// Assertion builder for file contents
        pub struct FileAssert {
            path: assert_fs::NamedTempFile,
        }

        impl FileAssert {
            /// Assert that the file contains the given text
            pub fn contains(self, text: &str) -> Self {
                let content = std::fs::read_to_string(self.path.path())
                    .unwrap_or_else(|e| panic!(
                        "Failed to read file {:?}: {}",
                        self.path.path(),
                        e
                    ));
                
                assert!(
                    content.contains(text),
                    "Expected file {:?} to contain '{}'\nActual content:\n{}",
                    self.path.path(),
                    text,
                    content
                );
                self
            }

            /// Assert that the file does not contain the given text
            pub fn not_contains(self, text: &str) -> Self {
                let content = std::fs::read_to_string(self.path.path())
                    .unwrap_or_else(|e| panic!(
                        "Failed to read file {:?}: {}",
                        self.path.path(),
                        e
                    ));
                
                assert!(
                    !content.contains(text),
                    "Expected file {:?} to not contain '{}'\nActual content:\n{}",
                    self.path.path(),
                    text,
                    content
                );
                self
            }

            /// Assert that the file exists
            pub fn exists(self) -> Self {
                self.path.assert(predicates::path::exists());
                self
            }

            /// Assert that the file matches a predicate
            pub fn matches<P: predicates::Predicate<[u8]>>(self, predicate: P) -> Self {
                self.path.assert(predicate);
                self
            }

            /// Get the actual file path
            pub fn path(&self) -> &std::path::Path {
                self.path.path()
            }

            /// Get the file content as a string
            pub fn content(&self) -> String {
                std::fs::read_to_string(self.path.path())
                    .unwrap_or_else(|e| panic!(
                        "Failed to read file {:?}: {}",
                        self.path.path(),
                        e
                    ))
            }
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

