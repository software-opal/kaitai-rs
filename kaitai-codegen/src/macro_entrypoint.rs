use proc_macro2::TokenStream;
use std::fs::File;
use std::path::PathBuf;

use syn::parse::{Parse, ParseStream, Result};
use syn::LitStr;

pub struct IncludeKsyConfig {
    file: String,
}
pub struct CodegenKsyConfig {
    content: String,
}

impl Parse for IncludeKsyConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let input_file: LitStr = input.parse()?;
        Ok(Self {
            file: input_file.value(),
        })
    }
}
impl Parse for CodegenKsyConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let content: LitStr = input.parse()?;
        Ok(Self {
            content: content.value(),
        })
    }
}

pub fn include_ksy(cfg: IncludeKsyConfig) -> TokenStream {
    let input_file = PathBuf::from(cfg.file);
    let crate_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let input_path = if input_file.is_relative() {
        crate_root.join(input_file)
    } else {
        input_file
    };

    let ksy_struct =
        kaitai_loader::loader::from_reader(File::open(&input_path).unwrap_or_else(|err| {
            panic!("Unable to read `{}`: {}", input_path.to_string_lossy(), err)
        }))
        .unwrap_or_else(|err| {
            panic!(
                "Unable to parse `{}`: {}",
                input_path.to_string_lossy(),
                err
            )
        });

    crate::render::render_spec(ksy_struct, crate::render::Config::default())
}

pub fn codegen_ksy(cfg: CodegenKsyConfig) -> TokenStream {
    let ksy_struct = kaitai_loader::loader::from_string(cfg.content)
        .unwrap_or_else(|err| panic!("Unable to parse: {}", err));

    crate::render::render_spec(ksy_struct, crate::render::Config::default())
}
