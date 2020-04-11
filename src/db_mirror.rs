use syn::Field;

pub fn generate_select_queries(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let mut select_all = quote! {
        impl #name {
            pub fn select_all() -> &'static str {
                concat!("select * from ", stringify!(#name))
            }

            pub fn select_all_count() -> &'static str {
                concat!("select count(*) from ", stringify!(#name))
            }
        }
    };


    let fields = struct_fields(ast);
    let partition_key_fields = filter_attributes(&fields, "partition_key");
    let cluster_key_fields = filter_attributes(&fields, "clustering_key");
    // How to implement this macro?
    // The test inside example/src/main.rs/test_db_mirror/test_select_queries should be compiled
    let select_all_primary_key = quote! {
        impl #name {

         }
    };


    select_all.append(select_all_primary_key);

    select_all
}

pub fn struct_fields(ast: &syn::DeriveInput) -> &Vec<Field> {
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        fields
    } else {
        panic!("The derive macro is defined for structs with named fields, not for enums or unit structs");
    }
}

pub fn filter_attributes<'a>(fields: &'a Vec<Field>, att_to_find: &'static str) -> Vec<&'a Field> {
    fields
        .iter()
        .filter(|f|
            f.attrs
                .iter()
                .any(|att| {
                    if let syn::MetaItem::Word(ref w) = att.value {
                        w.as_ref() == att_to_find
                    } else {
                        false
                    }
                }))
        .collect()
}