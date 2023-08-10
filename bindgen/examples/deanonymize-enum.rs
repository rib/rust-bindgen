use bindgen::{builder, callbacks::ParseCallbacks};

#[derive(Debug)]
struct Callbacks;

/// Finds a common prefix for underscore separated symbols
fn common_prefix(names: &[(String, String)]) -> Option<String> {
    let (first, _) = names.first()?;

    for (i, part) in first.split('_').enumerate() {
        if names.iter().find(|(name, _)| name.split('_').nth(i) != Some(part)).is_some() {
            return if i > 0 {
                let parts: Vec<_> = first.split('_').take(i).collect();
                Some(parts.join("_"))
            } else {
                None
            };
        }
    }

    Some(first.to_string())
}

impl ParseCallbacks for Callbacks {
    fn enum_variant_name(
            &self,
            _enum_name: Option<&str>,
            _original_variant_name: &str,
            _variant_value: bindgen::callbacks::EnumVariantValue,
        ) -> Option<String>
    {
        _original_variant_name.strip_prefix("MY_ENUM_").map(str::to_string)
    }

    fn enum_deanonymize_name(
            &self,
            _variants: &[(String, String)],
        ) -> Option<String>
    {
        let mapped = _variants.first().into_iter().filter_map(|(variant, _)| {
            if variant.starts_with("MY_ENUM_") {
                Some("MY_ENUM".to_string())
            } else {
                None
            }
        }).next();

        mapped.or_else(|| common_prefix(_variants))
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let header = std::env::args().nth(1).expect("Expected path to header");
    let bindings = builder().header(header)
        //.default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .parse_callbacks(Box::new(Callbacks))
        .generate()?;

    bindings.write_to_file("output.rs")?;
    Ok(())
}