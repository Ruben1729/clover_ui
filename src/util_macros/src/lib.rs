extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DeriveInput, Fields, Ident, Variant, FieldsUnnamed
};

#[proc_macro_derive(DeriveStyleKeys)]
pub fn derive_style_keys(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let new_enum_name = Ident::new(
        &format!("{}Key", name),
        name.span(),
    );
    let new_struct_name = Ident::new(
        &format!("{}Map", name),
        name.span(),
    );

    let data = match ast.data {
        Data::Enum(data) => data,
        _ => panic!("This macro only works with enums"),
    };

    let new_variants = generate_variants(&data);
    let insert_cases = generate_insert_cases(&name, &data);
    let get_default_cases = generate_get_default_cases(&name, &data);
    let getter_functions = generate_getter_functions(&name, &data);
    let setter_functions = generate_setter_functions(&name, &data);

    let expanded = quote! {
        #[derive(Debug, Clone, Eq, Hash, PartialEq)]
        pub enum #new_enum_name {
            #(#new_variants),*
        }

        pub struct #new_struct_name {
            pub values: std::collections::HashMap<#new_enum_name, #name>,
        }

        impl #new_struct_name {
            pub fn new() -> Self {
                Self {
                    values: std::collections::HashMap::new(),
                }
            }

            pub fn insert(&mut self, value: #name) {
                match value {
                    #(#insert_cases),*
                }
            }

            pub fn get(&self, key: #new_enum_name) -> #name {
                match self.values.get(&key) {
                    Some(value) => value.clone(),
                    None => match key {
                        #(#get_default_cases),*
                    },
                }
            }

            pub fn set(&mut self, value: #name) {
                match value {
                    #(#insert_cases),*
                }
            }

            #(#getter_functions)*
            #(#setter_functions)*
        }
    };

    TokenStream::from(expanded)
}

fn generate_variants(data: &DataEnum) -> Vec<Ident> {
    data.variants
        .iter()
        .map(|variant| {
            let Variant { ident, fields, .. } = variant;
            if let Fields::Unnamed(_) = fields {
                ident.clone()
            } else {
                panic!("This macro only works with enums where each variant has exactly one unnamed field");
            }
        })
        .collect()
}

fn generate_insert_cases(name: &Ident, data: &DataEnum) -> Vec<proc_macro2::TokenStream> {
    let property_enum_name = Ident::new(&format!("{}Key", name), name.span());
    data.variants
        .iter()
        .map(|variant| {
            let Variant { ident, .. } = variant;
            quote! {
                #name::#ident(val) => {
                    self.values.insert(#property_enum_name::#ident, #name::#ident(val));
                }
            }
        })
        .collect()
}

fn generate_get_default_cases(name: &Ident, data: &DataEnum) -> Vec<proc_macro2::TokenStream> {
    let property_enum_name = Ident::new(&format!("{}Key", name), name.span());
    data.variants
        .iter()
        .map(|variant| {
            let Variant { ident, .. } = variant;
            quote! {
                #property_enum_name::#ident => #name::#ident(Default::default())
            }
        })
        .collect()
}

fn generate_getter_functions(name: &Ident, data: &DataEnum) -> Vec<proc_macro2::TokenStream> {
    let property_enum_name = Ident::new(&format!("{}Key", name), name.span());
    data.variants
        .iter()
        .map(|variant| {
            let Variant { ident, fields, .. } = variant;
            let getter_name = Ident::new(&format!("get_{}", ident.to_string().to_lowercase()), ident.span());
            let field_type = match fields {
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => &unnamed.first().unwrap().ty,
                _ => panic!("Expected unnamed fields"),
            };

            quote! {
                pub fn #getter_name(&self) -> #field_type {
                    match self.values.get(&#property_enum_name::#ident) {
                        Some(#name::#ident(value)) => value.clone(),
                        None => Default::default(),
                        _ => panic!("At key the property is not of valid type")
                    }
                }
            }
        })
        .collect()
}

fn generate_setter_functions(name: &Ident, data: &DataEnum) -> Vec<proc_macro2::TokenStream> {
    let property_enum_name = Ident::new(&format!("{}Key", name), name.span());
    data.variants
        .iter()
        .map(|variant| {
            let Variant { ident, fields, .. } = variant;
            let setter_name = Ident::new(&format!("set_{}", ident.to_string().to_lowercase()), ident.span());
            let field_type = match fields {
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => &unnamed.first().unwrap().ty,
                _ => panic!("Expected unnamed fields"),
            };

            quote! {
                pub fn #setter_name(&mut self, val: #field_type) {
                    self.values.insert(#property_enum_name::#ident, #name::#ident(val));
                }
            }
        })
        .collect()
}
