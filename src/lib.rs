//  - boom: TODO ---------------------------------------------------
// boom                          display high-level overview
// boom all                      show all items in all lists
// boom help                     this help text
// boom <list>                   create a new list
// boom <list>                   show items for a list
// boom delete <list>            deletes a list
// boom <list> <name> <value>    create a new list item
// boom <list> <name>            echo item's value
// boom delete <list> <name>     deletes an item
// -----------------------------------------------------------------
#![feature(match_default_bindings)]
#[macro_use]
extern crate cute;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::default::Default;
use std::ops::Index;
use std::ops::Drop;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoomEntry {
    pub key: String,
    pub value: String,
}
/// A Named collection of `BoomEntry`
#[derive(Serialize, Deserialize, Clone)]
pub struct BoomCollection {
    pub collection: String,
    pub values: Vec<BoomEntry>,
}

impl BoomCollection {
    /// Returns all the Collection entry keys
    pub fn keys(&self) -> Vec<String> {
        c![c.key, for c in self.values.clone()]
    }
    // Helper method to find pos
    fn get_pos(&self, key: &str) -> Option<usize> {
        self.values.iter().position(|r| r.key == key)
    }
    /// Checks collection for a entry with a matching `key`
    pub fn contains_key(&self, key: &str) -> bool {
        self.get_pos(key).is_some()
    }
    /// Removes a entry from the collection, returns the entry if it exists.
    pub fn remove(&mut self, key: &str) -> Option<BoomEntry> {
        match self.get_pos(key) {
            None => None,
            Some(index) => Some(self.values.remove(index)),
        }
    }
    /// Returns a entry with a matching key if it exists.
    pub fn get(&self, key: &str) -> Option<&BoomEntry> {
        match self.get_pos(key) {
            Some(pos) => self.values.get(pos),
            None => None,
        }
    }
    /// Returns entry with a matching key if it exists.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut BoomEntry> {
        match self.get_pos(key) {
            Some(pos) => self.values.get_mut(pos),
            None => None,
        }
    }
    pub fn insert(&mut self, key: String, value: String) -> Option<BoomEntry> {
        match self.get_pos(&key) {
            Some(pos) => {
                let old = self.values.remove(pos);
                self.values.push(BoomEntry { key, value });
                Some(old)
            }
            None => {
                self.values.push(BoomEntry { key, value });
                None
            }
        }
    }
    pub fn insert_entry(&mut self, entry: BoomEntry) -> Option<BoomEntry> {
        match self.get_pos(&entry.key) {
            Some(pos) => {
                let old = self.values.remove(pos);
                self.values.push(entry);
                Some(old)
            }
            None => {
                self.values.push(entry);
                None
            }
        }
    }
    pub fn insert_many(&mut self, entries: Vec<(String, String)>) -> Option<Vec<BoomEntry>> {
        self.insert_many_entries(c![BoomEntry{key: entry.0, value: entry.1}, for entry in entries])
    }
    pub fn insert_many_entries(&mut self, entries: Vec<BoomEntry>) -> Option<Vec<BoomEntry>> {
        #[cfg_attr(feature = "cargo-clippy", allow(for_loop_over_option))]
        let results = c![e, for e in self.insert_entry(entry), for entry in entries];
        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<'a> Index<&'a str> for BoomCollection {
    type Output = String;
    fn index(&self, index: &str) -> &Self::Output {
        &self.values[self.get_pos(index).expect("No entry was found for key")].value
    }
}

#[derive(Serialize, Deserialize, Default)]
struct BoomData {
    pub data: Vec<BoomCollection>,
}

impl BoomData {
    /// Load `BoomData` from a file (toml)
    pub fn load(path: PathBuf) -> Self {
        match File::open(path) {
            Ok(f) => {
                let mut buf = BufReader::new(f);
                let mut content = String::new();
                match buf.read_to_string(&mut content) {
                    Ok(_) => match toml::from_str::<Self>(&content) {
                        Ok(boom) => boom,
                        Err(e) => panic!("Failed to read data {:?}", e),
                    },
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
            Err(_) => Default::default(),
        }
    }
    /// Save `BoomData` to a file (toml)
    pub fn save(&self, path: PathBuf) {
        let mut file = File::create(path).expect("Failed to open data");
        file.write_all(
            toml::to_string_pretty(self)
                .expect("Failed to Serialize data")
                .as_bytes(),
        ).expect("Failed to write data to file");
        file.sync_data().expect("Failed to sync to disk");
    }
    /// Returns all of the collection names
    #[allow(dead_code)]
    pub fn keys(&self) -> Vec<String> {
        c![e.collection, for e in self.data.clone()]
    }
    /// Check if a collection key exists
    pub fn contains_key(&self, collection: &str) -> bool {
        self.get_pos(collection).is_some()
    }
    /// Returns a collection with a matching name
    #[allow(dead_code)]
    pub fn get(&self, collection: &str) -> Option<&BoomCollection> {
        if !self.contains_key(collection) {
            None
        } else {
            Some(&self[collection])
        }
    }
    // Helper function to find collection position
    fn get_pos(&self, collection: &str) -> Option<usize> {
        self.data.iter().position(|r| r.collection == collection)
    }
    // Remove a collection, returns the `BoomCollection` if it exists
    pub fn remove(&mut self, collection: &str) -> Option<BoomCollection> {
        match self.get_pos(collection) {
            None => None,
            Some(idx) => Some(self.data.remove(idx)),
        }
    }
    //
    pub fn get_mut(&mut self, collection: &str) -> Option<&mut BoomCollection> {
        match self.get_pos(collection) {
            None => None,
            Some(idx) => self.data.get_mut(idx),
        }
    }
    pub fn insert(&mut self, key: String, value: Vec<BoomEntry>) -> Option<BoomCollection> {
        match self.get_pos(&key) {
            None => {
                self.data.push(BoomCollection {
                    collection: key,
                    values: value,
                });
                None
            }
            Some(idx) => {
                let old = self.data.remove(idx);
                self.data.push(BoomCollection {
                    collection: key,
                    values: value,
                });
                Some(old)
            }
        }
    }
    pub fn insert_collection(&mut self, collection: BoomCollection) -> Option<BoomCollection> {
        match self.get_pos(&collection.collection) {
            None => {
                self.data.push(collection);
                None
            }
            Some(index) => {
                let old = self.data.remove(index);
                self.data.push(collection);
                Some(old)
            }
        }
    }
    #[allow(dead_code)]
    pub fn insert_many(
        &mut self,
        collections: Vec<(String, Vec<BoomEntry>)>,
    ) -> Option<Vec<BoomCollection>> {
        self.insert_many_collections(
            c![BoomCollection{collection: c.0, values: c.1}, for c in collections],
        )
    }
    pub fn insert_many_collections(
        &mut self,
        collections: Vec<BoomCollection>,
    ) -> Option<Vec<BoomCollection>> {
        #[cfg_attr(feature = "cargo-clippy", allow(for_loop_over_option))]
        let results =
            c![c, for c in self.insert_collection(collection), for collection in collections];
        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}

impl<'a> Index<&'a str> for BoomData {
    type Output = BoomCollection;
    fn index(&self, index: &str) -> &Self::Output {
        &self.data[self.get_pos(index).expect("No entry was found for key")]
    }
}

pub struct Boom {
    /// Hashmap of of ListName: {Key: Value}
    data: BoomData,
    file: Option<PathBuf>,
    auto_save: bool,
}

impl Boom {
    pub fn new(file: PathBuf, auto_save: bool) -> Self {
        Self {
            data: BoomData::load(file.to_owned()),
            file: Some(file),
            auto_save,
        }
    }
    pub fn mem() -> Self {
        Self {
            file: None,
            auto_save: false,
            data: BoomData { data: Vec::new() },
        }
    }
    pub fn all(&self) -> Vec<BoomCollection> {
        self.data.data.clone()
    }
    pub fn save(&mut self) {
        match &self.file {
            Some(path) => self.data.save(path.to_owned()),
            None => (),
        }
    }
    pub fn get(&self, collection: &str) -> Option<&BoomCollection> {
        if self.data.contains_key(collection) {
            Some(&self.data[collection])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, collection: &str) -> Option<&mut BoomCollection> {
        self.data.get_mut(collection)
    }
    pub fn get_collection_entry(&self, collection: &str, entry: &str) -> Option<&BoomEntry> {
        match self.data.get(collection) {
            None => None,
            Some(collect) => collect.get(entry),
        }
    }
    pub fn delete_collection(&mut self, collection: &str) -> Option<BoomCollection> {
        self.data.remove(collection)
    }
    pub fn delete_collection_entry(&mut self, collection: &str, entry: &str) -> Option<BoomEntry> {
        match self.data.get_mut(collection) {
            None => None,
            Some(collect) => collect.remove(entry),
        }
    }
    pub fn create_collection(&mut self, collection: String) -> Option<BoomCollection> {
        self.data.insert(collection, Vec::new())
    }
    pub fn insert_collection(&mut self, collection: BoomCollection) -> Option<BoomCollection> {
        self.data.insert_collection(collection)
    }
}
impl<'a> Index<&'a str> for Boom {
    type Output = BoomCollection;
    fn index(&self, index: &str) -> &Self::Output {
        &self.data[index]
    }
}
impl Drop for Boom {
    fn drop(&mut self) {
        if self.auto_save {
            self.save()
        }
    }
}

impl fmt::Debug for Boom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let entry_count: usize = c![c.len(), for c in &self.data.data].iter().sum();
        write!(
            f,
            "{} collections with {} entries.",
            &self.data.data.len(),
            entry_count
        )
    }
}

impl fmt::Display for Boom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            c![format!("{} ({})", c.collection, c.len()), for c in &self.data.data].join("\n")
        )
    }
}

impl fmt::Debug for BoomCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Collection: {} ({})", self.collection, self.len())
    }
}

impl fmt::Display for BoomCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            c![format!("{}", e), for e in &self.values].join("\n")
        )
    }
}

impl fmt::Display for BoomEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}
