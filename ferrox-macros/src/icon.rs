//! `icon!`: parses the call, checks the drawing, emits the const expression.

use proc_macro2::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

/// `icon!("icons/pin.svg")` or `icon!(name = "spark", "<svg …>")`.
struct IconArgs {
    name: Option<LitStr>,
    source: LitStr,
}

impl Parse for IconArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = if input.peek(syn::Ident) && input.peek2(Token![=]) {
            let key: syn::Ident = input.parse()?;
            if key != "name" {
                return Err(syn::Error::new(key.span(), "expected `name = \"…\"` or a string"));
            }
            input.parse::<Token![=]>()?;
            let name: LitStr = input.parse()?;
            input.parse::<Token![,]>()?;
            Some(name)
        } else {
            None
        };
        let source: LitStr = input.parse()?;
        input.parse::<Option<Token![,]>>()?;
        Ok(IconArgs { name, source })
    }
}

/// Where the drawing comes from: inline markup, or a file to read.
enum Source {
    Inline { name: String, svg: String },
    File(PathBuf),
}

pub fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let args: IconArgs = syn::parse2(input)?;
    match source_of(&args)? {
        Source::Inline { name, svg } => {
            let icon = ferrox_icons_build::check_markup(&name, &svg).map_err(|e| error(&args.source, e))?;
            Ok(icon_data(&icon, None))
        }
        Source::File(path) => {
            let icon = ferrox_icons_build::check_file(&path)
                .map_err(|e| error(&args.source, format!("{}: {e}", args.source.value())))?;
            Ok(icon_data(&icon, Some(&path)))
        }
    }
}

/// Markup starts with `<`; anything else is a file path.
fn source_of(args: &IconArgs) -> syn::Result<Source> {
    let text = args.source.value();
    if text.trim_start().starts_with('<') {
        let name = args.name.as_ref()
            .ok_or_else(|| error(&args.source, "inline markup needs a name: icon!(name = \"…\", \"<svg …>\")"))?;
        return Ok(Source::Inline { name: name.value(), svg: text });
    }
    if let Some(name) = &args.name {
        return Err(error(name, "a file icon is named by its file stem; drop `name = …`"));
    }
    Ok(Source::File(resolve(&text)?))
}

/// A relative path is from the calling crate's root (its Cargo.toml folder).
fn resolve(path: &str) -> syn::Result<PathBuf> {
    let path = Path::new(path);
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let root = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| syn::Error::new(proc_macro2::Span::call_site(), "CARGO_MANIFEST_DIR is not set"))?;
    Ok(Path::new(&root).join(path))
}

/// The const expression. A file is also `include_bytes!`'d, so editing it
/// rebuilds the crate (proc macros can't declare the dependency otherwise).
fn icon_data(icon: &ferrox_icons_build::CheckedIcon, file: Option<&Path>) -> TokenStream {
    let (name, body) = (&icon.name, &icon.body);
    let track = file.map(|f| {
        let f = f.to_string_lossy();
        quote! { const _: &[u8] = include_bytes!(#f); }
    });
    quote! {
        {
            #track
            ::ferrox_icons::IconData::__checked(#name, #body)
        }
    }
}

fn error(at: &LitStr, message: impl std::fmt::Display) -> syn::Error {
    syn::Error::new(at.span(), message)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/></svg>"#;

    fn expand_str(tokens: &str) -> syn::Result<String> {
        expand(tokens.parse().unwrap()).map(|t| t.to_string())
    }

    #[test]
    fn inline_markup_becomes_checked_icon_data() {
        let out = expand_str(&format!("name = \"dash\", r#\"{SVG}\"#")).unwrap();
        assert!(out.contains(":: ferrox_icons :: IconData :: __checked (\"dash\" , \"<path d=\\\"M5 12h14\\\"/>\")"), "{out}");
        assert!(!out.contains("include_bytes"));
    }

    #[test]
    fn inline_markup_breaking_a_rule_is_an_error() {
        let red = SVG.replace("<path ", "<path fill=\"red\" ");
        let err = expand_str(&format!("name = \"dash\", r#\"{red}\"#")).unwrap_err();
        assert!(err.to_string().contains("currentColor"), "{err}");
    }

    #[test]
    fn inline_markup_needs_a_name() {
        let err = expand_str(&format!("r#\"{SVG}\"#")).unwrap_err();
        assert!(err.to_string().contains("needs a name"), "{err}");
    }

    #[test]
    fn a_file_is_read_checked_and_tracked() {
        let dir = std::env::temp_dir().join(format!("ferrox-macros-icon-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("dash.svg");
        std::fs::write(&file, SVG).unwrap();
        let out = expand_str(&format!("{:?}", file.to_string_lossy())).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(out.contains("__checked (\"dash\""), "{out}");
        assert!(out.contains("include_bytes"), "{out}");
    }

    #[test]
    fn a_missing_file_is_an_error_naming_it() {
        let err = expand_str("\"/nonexistent/pin.svg\"").unwrap_err();
        assert!(err.to_string().contains("/nonexistent/pin.svg: cannot read"), "{err}");
    }

    #[test]
    fn a_file_icon_takes_no_name() {
        let err = expand_str("name = \"x\", \"icons/pin.svg\"").unwrap_err();
        assert!(err.to_string().contains("file stem"), "{err}");
    }
}
