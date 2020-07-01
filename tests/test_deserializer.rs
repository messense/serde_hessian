use std::fs;
use std::sync::Arc;

use hessian_rs::Error;
use hessian_rs::{de::Deserializer, Value};

fn load_value_from_file(file_name: &str) -> Result<Arc<Value>, Error> {
    let rdr = fs::read(file_name)?;
    let mut de = Deserializer::new(rdr);
    de.read_value()
}

fn test_decode_bin(file_name: &str, expected: Value) {
    let decoded = load_value_from_file(file_name).unwrap();
    assert_eq!(*decoded, expected);
}

#[test]
fn test_decode_long_binary() {
    let value = load_value_from_file("tests/fixtures/bytes/65535.bin").unwrap();
    match &*value {
        Value::Bytes(bytes) => assert_eq!(*bytes, vec![0x41; 65535]),
        _ => panic!("expect bytes"),
    }
}

#[test]
fn test_decode_date() {
    test_decode_bin(
        "tests/fixtures/date/894621060000.bin",
        Value::Date(894621060000),
    );
    test_decode_bin(
        "tests/fixtures/date/894621091000.bin",
        Value::Date(894621091000),
    );
    test_decode_bin(
        "tests/fixtures/date/128849018880000.bin",
        Value::Date(128849018880000),
    );
    test_decode_bin(
        "tests/fixtures/date/-128849018940000.bin",
        Value::Date(-128849018940000),
    );
}

#[test]
fn test_decode_string() {
    test_decode_bin(
        "tests/fixtures/string/empty.bin",
        Value::String("".to_string()),
    );
    test_decode_bin(
        "tests/fixtures/string/foo.bin",
        Value::String("foo".to_string()),
    );
    test_decode_bin(
        "tests/fixtures/string/chinese.bin",
        Value::String("中文 Chinese".to_string()),
    );
}

#[test]
fn test_decode_list() {
    test_decode_bin(
        "tests/fixtures/list/untyped_list.bin",
        Value::List(vec![Value::Int(1), Value::Int(2), "foo".into()].into()),
    );
    test_decode_bin(
        "tests/fixtures/list/untyped_[].bin",
        Value::List(Vec::<Arc<Value>>::new().into()),
    );
    test_decode_bin(
        "tests/fixtures/list/untyped_list_8.bin",
        Value::List(
            vec!["1", "2", "3", "4", "5", "6", "7", "8"]
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<Value>>()
                .into(),
        ),
    );
    test_decode_bin(
        "tests/fixtures/list/untyped_<String>[foo,bar].bin",
        Value::List(vec![Arc::new("foo".into()), Arc::new("bar".into())].into()),
    );
    test_decode_bin(
        "tests/fixtures/list/[int.bin",
        Value::List(("[int", vec![Value::Int(1), Value::Int(2), Value::Int(3)]).into()),
    );
    test_decode_bin(
        "tests/fixtures/list/[string.bin",
        Value::List(("[string", vec![Arc::new("1".into()), Arc::new("@".into()), Arc::new("3".into())]).into()),
    );
    test_decode_bin(
        "tests/fixtures/list/typed_list.bin",
        Value::List(
            (
                "hessian.demo.SomeArrayList",
                vec![Arc::new("ok".into()), Arc::new("some list".into())],
            )
                .into(),
        ),
    );
    test_decode_bin(
        "tests/fixtures/list/typed_list_8.bin",
        Value::List(
            (
                "hessian.demo.SomeArrayList",
                vec!["1", "2", "3", "4", "5", "6", "7", "8"]
                    .into_iter()
                    .map(|x| Arc::new(x.into()))
                    .collect::<Vec<Arc<Value>>>()
            )
                .into(),
        ),
    );
}

#[test]
fn test_decode_map() {
    use maplit::hashmap;

    test_decode_bin(
        "tests/fixtures/map/car.bin",
        Value::Map(
            (
                "hessian.demo.Car",
                hashmap! {
                    "a".into() => "a".into(),
                    "b".into() => "b".into(),
                    "c".into() => "c".into(),
                    "model".into() => "Beetle".into(),
                    "color".into() => "aquamarine".into(),
                    "mileage".into() => Value::Int(65536),
                },
            )
                .into(),
        ),
    );

    /*
    test_decode_bin(
        "tests/fixtures/map/car1.bin",
        Value::Map(
            (
                "hessian.demo.Car",
                hashmap! {
                    "prev".into() => Value::Null,
                    "self".into() => Value::Ref(0),
                    "model".into() => "Beetle".into(),
                    "color".into() => "aquamarine".into(),
                    "mileage".into() => Value::Int(65536),
                },
            )
                .into(),
        ),
    );
    */

    test_decode_bin(
        "tests/fixtures/map/foo_empty.bin",
        Value::Map(
            hashmap! {
                Arc::new("foo".into()) => Arc::new("".into()),
            }
            .into(),
        ),
    );

    test_decode_bin(
        "tests/fixtures/map/foo_bar.bin",
        Value::Map(
            hashmap! {
                "foo".into() => "bar".into(),
                "123".into() => Value::Int(456),
                "zero".into() => Value::Int(0),
                "中文key".into() => "中文哈哈value".into(),
            }
            .into(),
        ),
    );

    test_decode_bin(
        "tests/fixtures/map/hashtable.bin",
        Value::Map(
            (
                "java.util.Hashtable",
                hashmap! {
                    Arc::new("foo".into()) => Arc::new("bar".into()),
                    Arc::new("中文key".into()) => Arc::new("中文哈哈value".into()),
                },
            )
                .into(),
        ),
    );

    test_decode_bin(
        "tests/fixtures/map/generic.bin",
        Value::Map(
            hashmap! {
                Value::Long(123) => Value::Int(123456),
                Value::Long(123456) => Value::Int(123),
            }
            .into(),
        ),
    );

    let val = load_value_from_file("tests/fixtures/map/hashmap.bin").unwrap();
    let map = val.as_map().unwrap();
    let data = &map[&Arc::new("data".into())];
    let data = data.as_map().unwrap();
    assert_eq!(data.len(), 2);

    let val = load_value_from_file("tests/fixtures/map/custom_map_type.bin").unwrap();
    let list = val.as_list().unwrap();
    assert_eq!(list.r#type().unwrap(), "com.alibaba.fastjson.JSONArray");
    let item0 = &list[0];
    let map = item0.as_map().unwrap();
    assert_eq!(map.r#type().unwrap(), "com.alibaba.fastjson.JSONObject");
    assert_eq!(map.len(), 3);
}

#[test]
fn test_decode_object() {
    let val = load_value_from_file("tests/fixtures/object/ConnectionRequest.bin").unwrap();
    let map = val.as_map().unwrap();
    assert_eq!(map.r#type().unwrap(), "hessian.ConnectionRequest");
    let ctx = &map[&Arc::new("ctx".into())].as_map().unwrap();
    assert_eq!(
        ctx.r#type().unwrap(),
        "hessian.ConnectionRequest$RequestContext"
    );
    assert_eq!(*ctx[&Arc::new("id".into())], Value::Int(101));

    let val = load_value_from_file("tests/fixtures/object/AtomicLong0.bin").unwrap();
    let map = val.as_map().unwrap();
    assert_eq!(
        map.r#type().unwrap(),
        "java.util.concurrent.atomic.AtomicLong"
    );
    assert_eq!(*map[&Arc::new("value".into())], Value::Long(0));

    let val = load_value_from_file("tests/fixtures/object/AtomicLong1.bin").unwrap();
    let map = val.as_map().unwrap();
    assert_eq!(
        map.r#type().unwrap(),
        "java.util.concurrent.atomic.AtomicLong"
    );
    assert_eq!(*map[&Arc::new("value".into())], Value::Long(1));
}
