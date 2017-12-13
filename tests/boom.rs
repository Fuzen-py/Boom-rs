extern crate boom;
extern crate tempdir;
use boom::*;
use tempdir::TempDir;

#[test]
fn boom() {
    let tempdir = TempDir::new("boom-test").expect("Failed to create tempdir");
    let mut temp_file = tempdir.path().to_path_buf();
    temp_file.push("test.toml");
    {
        let mut boom = Boom::new((*temp_file).to_path_buf(), true);
        assert_eq!(boom.all().len(), 0);
        assert!(boom.create_collection(String::from("Desert")).is_none());
        assert!(boom.get("Desert").is_some());
        assert!(
            boom.get_mut("Desert")
                .expect("Where is my desert???")
                .insert_many(vec![
                    (String::from("cake"), String::from("vanilla")),
                    (
                        String::from("chocolate"),
                        String::from("chocolate covered raisons"),
                    ),
                    (String::from("icecream"), String::from("vanilla")),
                ])
                .is_none()
        );
        assert_eq!(boom["Desert"]["cake"], "vanilla");
        assert!(boom.insert_collection(BoomCollection {
            collection: String::from("Test"),
            values: Vec::new(),
        }).is_none());
    }
    assert_eq!(Boom::new(temp_file, false)["Desert"]["cake"], "vanilla")
}
