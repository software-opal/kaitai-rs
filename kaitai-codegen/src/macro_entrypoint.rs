extern crate proc_macro;
use self::proc_macro::TokenStream;
use std::fs::File;
use std::path::PathBuf;

use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitStr};

struct IncludeKsyConfig {
    file: String,
}

impl Parse for IncludeKsyConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let input_file: LitStr = input.parse()?;
        Ok(IncludeKsyConfig {
            file: input_file.value(),
        })
    }
}

pub fn include_ksy(input: TokenStream) -> TokenStream {
    let IncludeKsyConfig { file } = parse_macro_input!(input as IncludeKsyConfig);

    let input_file = PathBuf::from(file);
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

    // let enums =
    // crate::render::enums::render_enums(ksy_struct.enums)
    TokenStream::new()

}
