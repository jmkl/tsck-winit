use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ScopeParser)]
pub fn derive_scope_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("ScopeParser only works on enums"),
    };

    let parse_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string().to_lowercase();

        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field_type = &fields.unnamed.first().unwrap().ty;

                quote! {
                    #variant_str => {
                        let func = #field_type::from_func(&parsed)?;
                        Ok(#name::#variant_name(func))
                    },
                }
            }
            _ => quote! {},
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn parse(s: &str) -> Result<Self, String> {
                let parsed = FuncLexer::parse_func(s)
                    .ok_or_else(|| format!("Failed to parse function string: {}", s))?;

                match parsed.entry.to_lowercase().as_str() {
                    #(#parse_arms)*
                    _ => Err(format!("Unknown scope: {}", parsed.entry))
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse(s)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FuncParser)]
pub fn derive_func_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("FuncParser only works on enums"),
    };

    let parse_branches = variants.iter().map(|variant| {
          let variant_name = &variant.ident;
          let variant_str = variant_name.to_string().to_lowercase();

          match &variant.fields {
              Fields::Unit => {
                  quote! {
                      if parsed.func.eq_ignore_ascii_case(#variant_str) {
                          return Ok(#name::#variant_name);
                      }
                  }
              }
              Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                  let field_type = &fields.unnamed.first().unwrap().ty;
                  let type_str = quote!(#field_type).to_string();

                  if type_str == "String" {
                      quote! {
                          if parsed.func.eq_ignore_ascii_case(#variant_str) {
                              if let Some(FuncExpr::String(s)) = &parsed.args {
                                  return Ok(#name::#variant_name(s.to_string()));
                              }
                              return Err(format!("{} expects a string argument", #variant_str));
                          }
                      }
                  } else if type_str == "i32" {
                      quote! {
                          if parsed.func.eq_ignore_ascii_case(#variant_str) {
                              if let Some(FuncExpr::Number(n)) = &parsed.args {
                                  return Ok(#name::#variant_name(*n as i32));
                              }
                              return Err(format!("{} expects a number argument", #variant_str));
                          }
                      }
                  } else {
                      quote! {
                          if parsed.func.eq_ignore_ascii_case(#variant_str) {
                              if let Some(FuncExpr::Ident(ident)) = &parsed.args {
                                  return ident.parse::<#field_type>()
                                      .map(|v| #name::#variant_name(v))
                                      .map_err(|e| format!("Failed to parse {}: {:?}", #variant_str, e));
                              }
                              return Err(format!("{} expects an identifier argument", #variant_str));
                          }
                      }
                  }
              }
              _ => quote! {}
          }
      });

    let expanded = quote! {
        impl #name {
            pub fn from_func(parsed: &Func) -> Result<Self, String> {
                #(#parse_branches)*
                Err(format!("Unknown {} variant: {}", stringify!(#name), parsed.func))
            }
        }
    };

    TokenStream::from(expanded)
}
