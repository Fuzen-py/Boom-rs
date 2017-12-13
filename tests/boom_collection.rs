
extern crate boom;
use boom::{BoomCollection, BoomEntry};

#[test]
fn collection() {
	let mut collection = BoomCollection{
		collection: String::from("Test"),
		values: vec![
				BoomEntry{key: String::from("this"), value: String::from("is")},
				BoomEntry{key:String::from("a"), value: String::from("test")}
			]
	};

	// Test Keys
	let keys = collection.keys();
	assert!(keys.contains(&String::from("this")));
	assert!(!keys.contains(&String::from("is")));
	assert!(keys.contains(&String::from("a")));
	assert!(!keys.contains(&String::from("test")));
	assert_eq!(keys.len(), 2);

	// Test contains
	assert!(collection.contains_key("this"));
	assert!(!collection.contains_key("is"));
	assert!(collection.contains_key("a"));
	assert!(!collection.contains_key("test"));

	// Test Remove
	let this = collection.remove("this").expect("Failed to find this");
	assert!(!collection.contains_key("this"));
	assert!(collection.remove("this").is_none());

	// Test Insert
	assert!(collection.insert(String::from("i"), String::from("swear")).is_none());
	assert!(collection.contains_key("i"));

	// Test Insert Entry
	assert!(collection.insert_entry(this).is_none());
	assert!(collection.contains_key("this"));

	// Test Insert many
	assert!(collection.insert_many(vec![
		(String::from("please"),String::from("remain")),
		(String::from("calm"),String::from("under")),
		(String::from("all"),String::from("circumstances"))
	]).is_none());
	// Make sure insert is returning all results
	assert_eq!(collection.insert_many_entries(vec![
		BoomEntry{key: String::from("please"), value: String::from("remain")},
		BoomEntry{key: String::from("calm"), value: String::from("under")},
		BoomEntry{key: String::from("all"), value: String::from("circumstances")}
	]).expect("There is duplicates").len(), 3);

	// Test len
	assert_eq!(collection.len(), 6);

	// Test get
	assert_eq!(collection.get("please").expect("Failed to find please (error)").value, "remain");
	// Test get_mut
	collection.get_mut("please").expect("Failed to find please (error)").value = String::from("Remain");
	assert_eq!(collection.get("please").expect("Failed to find please (error)").value, "Remain");

	// Test Index
	assert_eq!(collection["please"], "Remain");
}
