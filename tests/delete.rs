use serde_json::{json};
use json_pointer::{IndexError, JsonPointer};

#[test]
fn delete() {
	let mut json = json!({
		"foo": "bar",
		"baz": {
			"boo": "x",
			"bam": "y"
		},
		"list": [
			"a",
			"b",
			"c"
		]
	});

	// Deleting keys from objects
	let ptr: JsonPointer<_, _> = "/baz/boo".parse().unwrap();
	ptr.delete(&mut json).unwrap();
	assert!(json["baz"].as_object().unwrap().contains_key("bam"));
	assert!(!json["baz"].as_object().unwrap().contains_key("boo"));
	let ptr: JsonPointer<_, _> = "/baz".parse().unwrap();
	ptr.delete(&mut json).unwrap();
	assert!(json.as_object().unwrap().contains_key("foo"));
	assert!(!json.as_object().unwrap().contains_key("baz"));

	// Deleting from arrays
	let ptr: JsonPointer<_, _> = "/list/1".parse().unwrap();
	ptr.delete(&mut json).unwrap();
	assert_eq!(json["list"].as_array().unwrap().len(), 2);
	assert_eq!(json["list"].as_array().unwrap()[0], "a");
	assert_eq!(json["list"].as_array().unwrap()[1], "c");

	// Deleting past the end of an array
	let ptr: JsonPointer<_, _> = "/list/2".parse().unwrap();
	assert_eq!(ptr.delete(&mut json), Err(IndexError::OutOfBounds(2)));

	// Deleting the 'next new item' of an array fails
	let ptr: JsonPointer<_, _> = "/list/-".parse().unwrap();
	assert_eq!(ptr.delete(&mut json), Err(IndexError::OutOfBounds(2)));

	// Can't delete root
	let ptr: JsonPointer<_, _> = "".parse().unwrap();
	assert_eq!(ptr.delete(&mut json), Err(IndexError::NotIndexable));
}