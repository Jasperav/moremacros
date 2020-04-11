#[macro_use]
extern crate moremacros_derive;

#[cfg(test)]
mod test_db_mirror {
    #[derive(DBMirror)]
    #[allow(dead_code)]
    struct SomeStruct {
        #[partition_key]
        id: i32,
        #[partition_key]
        another_id: i32,
        #[clustering_key]
        cluster_key: i32,
        #[clustering_key]
        another_cluster_key: i32,
        // Just some column that is not part of the primary key
        name: String,
    }

    #[test]
    fn test_select_queries() {
        assert_eq!("select * from SomeStruct", SomeStruct::select_all());
        assert_eq!("select count(*) from SomeStruct", SomeStruct::select_all_count());

        // The line below should NOT be compiled, since only queries that have a where clause can be queried at least by there full partition key (and optional clustering keys)
        //assert_eq!("select * from SomeStruct where id = ?", SomeStruct::select_by_id());
        assert_eq!("select * from SomeStruct where id = ? and another_id = ?", SomeStruct::select_by_id_another_id());
        assert_eq!("select * from SomeStruct where id = ? and another_id = ? and cluster_key = ?", SomeStruct::select_by_id_another_id_cluster_key());
        assert_eq!("select * from SomeStruct where id = ? and another_id = ? and cluster_key = ? and another_cluster_key = ?", SomeStruct::select_unique());

    }
}