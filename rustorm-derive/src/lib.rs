extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

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
    let fields:Vec<&syn::Ident> = match ast.body {
        syn::Body::Struct(ref data) => {
            match *data{
                syn::VariantData::Struct(ref fields) => {
                    fields.iter().map(|f| {
                                let ident = f.ident.as_ref().unwrap();
                                let ty = &f.ty;
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
                            #field: dao.get(stringify!(#field)),
                        }
                    }).collect::<Vec<_>>();

    let to_dao:Vec<quote::Tokens> =
            fields.iter().map(|field| {
                        quote!{
                            dao.insert(stringify!(#field), self.#field);
                        }
                    }).collect::<Vec<_>>();
    quote! {
        impl IsDao for  #name {
        
            fn from_dao(dao: &Dao) -> Self{
                Dao{
                    #(#from_fields)*
                }
            }

            fn to_dao(&self) -> Dao {
                let dao = Dao::new();
                #(#to_dao)*
                dao
            }
        }
    }
}
