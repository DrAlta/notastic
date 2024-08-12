use std::collections::{BTreeMap, HashMap};

use uuid::Uuid;

use crate::{note::Note, ReverseStr};
pub fn generate_lookup(notes: &HashMap<Uuid, Note>) -> BTreeMap<ReverseStr, &Uuid> {
    notes
        .iter()
        .map(|(uuid, note)| (Into::<ReverseStr>::into(&note.title), uuid))
        .collect()
}
