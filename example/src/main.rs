#[macro_use]
extern crate moremacros_derive;

#[cfg(test)]
mod test {
    #[derive(DBMirror)]
    struct SomeStruct {
        id: i32
    }

    #[test]
    fn test_some_struct() {
        SomeStruct::call_id(vec![1, 2]);
    }
}