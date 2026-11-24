use super::*;

#[test]
fn test_parse_json() {
    let raw = br#"{"name": "Alice", "age": 30}"#;
    let val = parse_input(raw, InputFormat::Json).unwrap();
    assert_eq!(val["name"], "Alice");
    assert_eq!(val["age"], 30);
}

#[test]
fn test_parse_csv() {
    let raw = b"name,age,active\nAlice,30,true\nBob,25,false\n";
    let val = parse_input(raw, InputFormat::Csv).unwrap();
    assert!(val.is_array());
    let arr = val.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["name"], "Alice");
    assert_eq!(arr[0]["age"], 30);
    assert_eq!(arr[0]["active"], true);
    assert_eq!(arr[1]["name"], "Bob");
    assert_eq!(arr[1]["age"], 25);
    assert_eq!(arr[1]["active"], false);
}

#[test]
fn test_detect_format() {
    assert_eq!(InputFormat::detect_from_filename("data.json"), Some(InputFormat::Json));
    assert_eq!(InputFormat::detect_from_filename("records.csv"), Some(InputFormat::Csv));
    assert_eq!(InputFormat::detect_from_filename("sheet.xlsx"), Some(InputFormat::Xlsx));
    assert_eq!(InputFormat::detect_from_filename("unknown.txt"), None);
}
