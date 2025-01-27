use std::collections::{BTreeMap, HashMap};

use uuid::Uuid;

use crate::note::Note;
pub fn generate_lookup(notes: &HashMap<Uuid, Note>) -> BTreeMap<&str, &Uuid> {
    notes
        .iter()
        .map(|(uuid, note)| {let a:&str =&note.title; (a, uuid)})
        .collect()
}
