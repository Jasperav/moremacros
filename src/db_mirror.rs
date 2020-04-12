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
    // It should expand to this:

    // impl SomeStruct {
    //     pub fn select_by_id_another_id(id: i32, another_id: i32) -> &'static str {
    //         "select * from SomeStruct where id = ? and another_id = ?"
    //     }
    //
    //     pub fn select_by_id_another_id_cluster_key(id: i32, another_id: i32, cluster_key: i32) -> &'static str {
    //         // Can also call the other macro
    //         "select * from SomeStruct where id = ? and another_id = ? and cluster_key = ?"
    //     }
    //
    //     pub fn select_unique(id: i32, another_id: i32, cluster_key: i32, another_cluster_key: i32) -> &'static str {
    //         // Can also call the other macro
    //         "select * from SomeStruct where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?"
    //     }
    // }
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