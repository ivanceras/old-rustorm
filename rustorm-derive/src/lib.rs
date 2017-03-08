#![deny(warnings)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::MetaItem::*;

#[proc_macro_derive(IsDao)]
pub fn is_dao(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_is_dao(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_is_dao(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields:Vec<(&syn::Ident, &syn::Ty)> = match ast.body {
        syn::Body::Struct(ref data) => {
            match *data{
                syn::VariantData::Struct(ref fields) => {
                    fields.iter().map(|f| {
                                let ident = f.ident.as_ref().unwrap();
                                let ty = &f.ty;
                                (ident,ty)
                            }).collect::<Vec<_>>()
                },
                _ => panic!("tuples and unit are not covered")
            }
        },
        syn::Body::Enum(_) => panic!("#[derive(NumFields)] can only be used with structs"),
    };
    let from_fields:Vec<quote::Tokens> =
            fields.iter().map(|&(field,_ty)| {
                        quote!{
                            #field: {
                                    let v = dao.get(stringify!(#field)).unwrap();
                                    FromValue::from_type(v.to_owned())
                                },
                        }
                    }).collect::<Vec<_>>();

    let to_dao:Vec<quote::Tokens> =
            fields.iter().map(|&(field,_ty)| {
                        quote!{
                            dao.insert(stringify!(#field).to_string(), self.#field.to_db_type());
                        }
                    }).collect::<Vec<_>>();
    quote! {
        impl IsDao for  #name {
        
            fn from_dao(dao: &Dao) -> Self{
                #name{
                    #(#from_fields)*
                }
            }

            fn to_dao(&self) -> Dao {
                let mut dao = Dao::new();
                #(#to_dao)*
                dao
            }
        }
    }
}

#[proc_macro_derive(IsTable)]
pub fn to_table_name(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_to_table_name(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn get_table_attr(attrs: &Vec<syn::Attribute>)->Option<String>{
    for att in attrs{
        println!("{:?}", att);
        match att.value{
            Word(_) => continue,
            List(_,_) => continue,
            NameValue(ref name, ref value) => {
                if name == "table"{
                    match *value{
                        syn::Lit::Str(ref s,ref _style) => {
                            return Some(s.to_owned())
                        }
                        _ => continue
                    }
                }else{continue}
            }
        };
    }
    None
}

fn impl_to_table_name(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let attrs = &ast.attrs;
    let tbl = get_table_attr(attrs);
    let table_name = match tbl{
        Some(tbl) => tbl,
        None => format!("{}",name).to_lowercase()
    };
    let fields:Vec<&syn::Ident> = match ast.body {
        syn::Body::Struct(ref data) => {
            match *data{
                syn::VariantData::Struct(ref fields) => {
                    fields.iter().map(|f| {
                                let ident = f.ident.as_ref().unwrap();
                                let _ty = &f.ty;
                                ident
                            }).collect::<Vec<_>>()
                },
                _ => panic!("tuples and unit are not covered")
            }
        },
        syn::Body::Enum(_) => panic!("#[derive(NumFields)] can only be used with structs"),
    };
    let from_fields:Vec<quote::Tokens> =
            fields.iter().map(|field| {
                        quote!{
                            ColumnName{
                                column: stringify!(#field).to_string(),
                                table: Some(#table_name.to_string()),
                                schema: None
                            }
                        }
                    }).collect::<Vec<_>>();

    quote! {
        impl IsTable for  #name {
        
            fn table_name() -> TableName{
                TableName{
                    schema: None,
                    name: #table_name.to_string(),
                    columns: vec![#(#from_fields),*],
                }
            }
        }
    }
}
