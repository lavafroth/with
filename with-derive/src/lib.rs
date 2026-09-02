use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    DataStruct, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input,
};

#[proc_macro_derive(withopt)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let fields = match &input.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => &fields_named.named,
        _ => {
            return syn::Error::new_spanned(input, "Only named structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let mut opt_idents = Vec::new();
    let mut opt_types = Vec::new();

    let mut required_idents = Vec::new();
    let mut required_types = Vec::new();

    for f in fields {
        let ident = &f.ident;
        let (is_option, inner) = extract_option_type(&f.ty);
        if is_option {
            opt_idents.push(ident);
            opt_types.push(inner);
        } else {
            required_idents.push(ident);
            required_types.push(&f.ty);
        }
    }

    let field_idents: Vec<&syn::Ident> = fields
        .iter()
        .map(|f| {
            f.ident
                .as_ref()
                .expect("already asserted that the struct has named fields")
        })
        .collect();

    let struct_name = &input.ident;

    if field_idents.is_empty() {
        return quote! {
            impl ::with::WithOpt for #struct_name {
                fn parse(args: &[&str]) -> Result<Self, String> {
                    Ok(Self {})
                }
            }
        }
        .into();
    }

    let output = quote! {
        impl ::with::WithOpt for #struct_name {
            fn parse(args: &[&str]) -> Result<Self, String> {
                use std::collections::BTreeMap;
                let mut with_ixs = vec![];
                let mut as_ixs = vec![];
                for (i, &word) in args.iter().enumerate() {
                    if word.eq("with") {
                        with_ixs.push(i);
                    }

                    if word.eq("as") && with_ixs.len() - as_ixs.len() == 1 {
                        as_ixs.push(i);
                    }
                }

                let equal_delimiters = with_ixs.len() == as_ixs.len();
                if !equal_delimiters {
                    return Err(format!("a `with flag as value` pattern is missing the `as`"));
                }

                if with_ixs.is_empty() {
                    return Err(format!("at least one `with flag as value` pattern must be provided"));
                }

                let mut end_ixs = with_ixs[1..].to_vec();
                end_ixs.push(args.len());

                let equal_delimiter_terminators = with_ixs.len() == end_ixs.len();
                if !equal_delimiter_terminators {
                    return Err(format!("a `with flag as value` pattern is missing the `as`"));
                }

                let mut collection = BTreeMap::new();

                for ((start, sep), end) in with_ixs.iter().zip(as_ixs).zip(end_ixs) {
                    collection.insert(args[start + 1..sep].join("_"), args[sep + 1..end].join(" "));
                }
                // println!("{:?}", collection);
                Ok(Self {
                    #(
                      #opt_idents: match collection.get(stringify!(#opt_idents)) {
                          None => None,
                          Some(v) => Some(v.parse::<#opt_types>().map_err(|_| format!("failed to parse optional argument {}",
                               stringify!(#opt_idents)
                          ))?),
                      },
                    )*

                    #(
                      #required_idents: collection.get(stringify!(#required_idents)).ok_or_else(|| {
                          format!("missing required argument {}", stringify!(#required_idents))
                      })?.parse::<#required_types>()
                      .map_err(|_| format!("failed to parse required argument {}",
                           stringify!(#required_idents)
                      ))?,
                    )*
                })
            }
        }
    };
    output.into()
}

fn extract_option_type(ty: &Type) -> (bool, &Type) {
    let default = (false, ty);
    let Type::Path(type_path) = ty else {
        return default;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return default;
    };
    if segment.ident != "Option" {
        return default;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return default;
    };
    let Some(GenericArgument::Type(inner_type)) = args.args.first() else {
        return default;
    };
    (true, inner_type)
}
