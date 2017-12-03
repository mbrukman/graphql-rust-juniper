#[cfg(test)]
use fnv::FnvHashMap;

#[cfg(test)]
use juniper::{self, execute, EmptyMutation, GraphQLType, RootNode, Value, Variables};

#[derive(GraphQLObject, Debug, PartialEq)]
#[graphql(name = "MyObj", description = "obj descr")]
struct Obj {
    regular_field: bool,
    #[graphql(name = "renamedField", description = "descr", deprecation = "field descr")]
    c: i32,
}

#[derive(GraphQLObject, Debug, PartialEq)]
struct Nested {
    obj: Obj,
}

struct Query;

graphql_object!(Query: () |&self| {
    field obj() -> Obj {
      Obj{
        regular_field: true,
        c: 22,
      }
    }

    field nested() -> Nested {
        Nested{
            obj: Obj{
                regular_field: false,
                c: 333,
            }
        }
    }
});

#[test]
fn test_derived_object() {
    assert_eq!(Obj::name(&()), Some("MyObj"));

    // Verify meta info.
    let mut registry = juniper::Registry::new(FnvHashMap::default());
    let meta = Obj::meta(&(), &mut registry);

    assert_eq!(meta.name(), Some("MyObj"));
    assert_eq!(meta.description(), Some(&"obj descr".to_string()));

    let doc = r#"
        {
            obj {
                regularField
                renamedField
            }
        }"#;

    let schema = RootNode::new(Query, EmptyMutation::<()>::new());

    assert_eq!(
    execute(doc, None, &schema, &Variables::new(), &()),
    Ok((Value::object(vec![
      ("obj", Value::object(vec![
        ("regularField", Value::boolean(true)),
        ("renamedField", Value::int(22)),
      ].into_iter().collect())),
    ].into_iter().collect()),
        vec![])));
}

#[test]
fn test_derived_object_nested() {
    let doc = r#"
        {
            nested {
                obj {
                    regularField
                    renamedField
                }
            }
        }"#;

    let schema = RootNode::new(Query, EmptyMutation::<()>::new());

    assert_eq!(
        execute(doc, None, &schema, &Variables::new(), &()),
        Ok((Value::object(vec![
            ("nested", Value::object(vec![
                ("obj", Value::object(vec![
                    ("regularField", Value::boolean(false)),
                    ("renamedField", Value::int(333)),
                ].into_iter().collect())
                )].into_iter().collect())),
        ].into_iter().collect()),
            vec![])));
}
