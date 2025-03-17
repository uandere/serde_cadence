extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ToCadenceValue)]
pub fn derive_to_cadence_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get field information
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("ToCadenceValue can only be derived for structs with named fields"),
        },
        _ => panic!("ToCadenceValue can only be derived for structs"),
    };

    // Generate code for each field
    let field_conversions = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            let #field_name = serde_cadence::CompositeField {
                name: #field_name_str.to_string(),
                value: self.#field_name.to_cadence_value()?,
            };
            fields.push(#field_name);
        }
    });

    // Generate the impl
    let expanded = quote! {
        impl serde_cadence::ToCadenceValue for #name {
            fn to_cadence_value(&self) -> serde_cadence::Result<serde_cadence::CadenceValue> {
                let mut fields = Vec::new();

                #(#field_conversions)*

                Ok(serde_cadence::CadenceValue::Struct {
                    value: serde_cadence::CompositeValue {
                        id: stringify!(#name).to_string(),
                        fields,
                    },
                })
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromCadenceValue)]
pub fn derive_from_cadence_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get field information
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FromCadenceValue can only be derived for structs with named fields"),
        },
        _ => panic!("FromCadenceValue can only be derived for structs"),
    };

    // Generate field extraction code
    let field_extractions = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            let #field_name = {
                let field = fields.iter()
                    .find(|f| f.name == #field_name_str)
                    .ok_or_else(||
                        serde_cadence::Error::Custom(
                            format!("Field {} not found in Cadence value", #field_name_str)
                        )
                    )?;
                serde_cadence::FromCadenceValue::from_cadence_value(&field.value)?
            };
        }
    });

    // Generate struct construction
    let field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name }
    });

    // Generate the impl
    let expanded = quote! {
        impl serde_cadence::FromCadenceValue for #name {
            fn from_cadence_value(value: &serde_cadence::CadenceValue) -> serde_cadence::Result<Self> {
                match value {
                    serde_cadence::CadenceValue::Struct { value: composite } => {
                        let fields = &composite.fields;

                        #(#field_extractions)*

                        Ok(Self {
                            #(#field_names),*
                        })
                    },
                    _ => Err(serde_cadence::Error::TypeMismatch {
                        expected: "Struct".to_string(),
                        got: format!("{:?}", value),
                    }),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
