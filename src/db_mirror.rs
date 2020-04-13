use syn::{Field, Ident, AngleBracketedParameterData};

pub fn generate(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let field = struct_fields(ast)[0].clone();
    let param_name = field.ident.unwrap();
    let fn_name = Ident::new("call_".to_string() + param_name.clone().as_ref());

    let param_ty = syn::Ty::Path(None, syn::Path::from(
        syn::PathSegment {
            ident: Ident::new("std::vec::Vec"),
            parameters: syn::PathParameters::AngleBracketed(AngleBracketedParameterData {
                lifetimes: vec![],
                types: vec![syn::Ty::Path(None,
                                          syn::Path {
                                              global: false,
                                              segments: vec![syn::PathSegment {
                                                  ident: match field.ty {
                                                      syn::Ty::Path(_, p) => {
                                                          p.segments[0].ident.clone()
                                                      }
                                                      _ => panic!()
                                                  },
                                                  parameters: syn::PathParameters::AngleBracketed(AngleBracketedParameterData {
                                                      lifetimes: vec![],
                                                      types: vec![],
                                                      bindings: vec![],
                                                  }),
                                              }],
                                          },
                )],
                bindings: vec![],
            }),
        }
    ));

    let q =

    quote! {
        impl #name {
            pub fn #fn_name(#param_name: #param_ty) {

            }
         }
    };

    println!("{:#?}", q);

    q
}

pub fn struct_fields(ast: &syn::DeriveInput) -> &Vec<Field> {
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        fields
    } else {
        panic!("The derive macro is defined for structs with named fields, not for enums or unit structs");
    }
}