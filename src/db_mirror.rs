use syn::{Field, Ident};

fn write(name: &Ident, v: &Vec<Field>, v_is_full_pk: bool) -> quote::Tokens {
    let names = v.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>();
    let types = v.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();
    let parameterized = parameterized(&names);
    let fn_name = create_fn_name(&v, v_is_full_pk);

    quote! {
        impl #name {
            pub fn #fn_name(#(#names: #types),*) -> &'static str {
                concat!("select * from ", stringify!(#name), " where ", #parameterized)
            }
         }
    }
}

fn create_fn_name(v: &Vec<Field>, is_unique: bool) -> Ident {
    if is_unique {
        return Ident::new("select_unique")
    }

    Ident::new("select_by_".to_string() + &v
        .iter()
        .map(|p| p.ident.clone().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("_"))
}

fn parameterized(v: &Vec<Ident>) -> String {
    v
        .iter()
        .map(|f| f.clone().to_string() + " = ?")
        .collect::<Vec<_>>()
        .join(" and ")
}

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

    let fields = struct_fields(ast).clone();
    let partition_key_fields = filter_attributes(&fields, "partition_key");
    let cluster_key_fields = filter_attributes(&fields, "clustering_key");

    if partition_key_fields.is_empty() {
        assert!(cluster_key_fields.is_empty());

        return select_all
    }

    select_all.append(write(name, &partition_key_fields, cluster_key_fields.is_empty()));

    let mut processed_clustering_key_fields = partition_key_fields.clone();
    let key_size = partition_key_fields.len() + cluster_key_fields.len();

    for clustering_key in cluster_key_fields.iter() {
        processed_clustering_key_fields.push(clustering_key.clone());

        select_all.append(write(name, &processed_clustering_key_fields, processed_clustering_key_fields.len() == key_size))
    }

    select_all
}

pub fn struct_fields(ast: &syn::DeriveInput) -> &Vec<Field> {
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        fields
    } else {
        panic!("The derive macro is defined for structs with named fields, not for enums or unit structs");
    }
}

pub fn filter_attributes(fields: &Vec<Field>, att_to_find: &str) -> Vec<Field> {
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
        .map(|f| f.clone())
        .collect()
}