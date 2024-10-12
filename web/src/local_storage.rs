use std::collections::HashSet;

use shared::types::installation::InstallationId;
use web_sys::Storage;

use leptos::prelude::*;

/// `use_local_storage*` from `leptos-use` doesn't work.
pub fn local_storage() -> Storage {
    window()
        .local_storage()
        .expect("shouldn't happen")
        .expect("shouldn't happen")
}

const INSTALLATION_IDS_KEY: &str = "installation_ids";

pub fn add_installation_id_to_local_storage(id: InstallationId) {
    let mut ids = get_installation_ids_from_local_storage();
    ids.insert(id);

    local_storage()
        .set_item(
            INSTALLATION_IDS_KEY,
            &serde_json::to_string(&ids).expect(""),
        )
        .expect("");
}

pub fn get_installation_ids_from_local_storage() -> HashSet<InstallationId> {
    local_storage()
        .get_item(INSTALLATION_IDS_KEY)
        .expect("")
        .map(|ids| serde_json::from_str(&ids).ok())
        .flatten()
        .unwrap_or(HashSet::new())
}
