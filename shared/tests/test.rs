use shared::{
    sync_engine::optimistic_change_map::{OptimisticChangeMap, Status},
    types::repository::{Repository, RepositoryId},
};

use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// Assume supporting structures are defined somewhere in your crate
#[derive(Clone)]
struct MockValue(i32);

fn create_mock_repo_id() -> RepositoryId {
    124123.into()
}

#[wasm_bindgen_test]
fn test_status_mark_successful() {
    let mut status: Status<MockValue> = Status::Pending(Some(MockValue(42)));
    status.mark_successful();
    if let Status::Successful(Some(MockValue(value))) = status {
        assert_eq!(value, 42);
    } else {
        panic!("Status was not marked successful correctly");
    }
}

#[wasm_bindgen_test]
fn test_status_read() {
    let status: Status<MockValue> = Status::Pending(Some(MockValue(42)));
    let value = status.read();
    assert_eq!(value.0, 42);
}

#[wasm_bindgen_test]
fn test_status_get() {
    let status: Status<MockValue> = Status::Pending(Some(MockValue(42)));
    let value = status.get();
    assert_eq!(value.0, 42);
}

#[wasm_bindgen_test]
fn test_optimistic_change_map_insert() {
    let change_map = OptimisticChangeMap::<MockValue>::default();
    let repo_id = create_mock_repo_id();
    let value = MockValue(100);

    let time = change_map.insert::<Repository>(&repo_id, value);

    let latest_status = change_map.latest::<Repository>(&repo_id);
    assert!(latest_status.is_some());
    assert_eq!(latest_status.unwrap().read().0, 100);
}

#[wasm_bindgen_test]
fn test_optimistic_change_map_mark_successful() {
    let change_map = OptimisticChangeMap::<MockValue>::default();
    let repo_id = create_mock_repo_id();
    let value = MockValue(200);

    let time = change_map.insert::<Repository>(&repo_id, value);
    change_map.mark_successful::<Repository>(&repo_id, &time);

    let status_after = change_map.latest::<Repository>(&repo_id).unwrap();
    if let Status::Successful(_) = status_after {
        assert!(true);
    } else {
        panic!("Failed to mark as Successful");
    }
}

#[wasm_bindgen_test]
fn test_optimistic_change_map_remove() {
    let change_map = OptimisticChangeMap::<MockValue>::default();
    let repo_id = create_mock_repo_id();
    let value = MockValue(300);

    let time = change_map.insert::<Repository>(&repo_id, value);
    change_map.remove::<Repository>(&repo_id, &time);

    let latest_status = change_map.latest::<Repository>(&repo_id);
    assert!(latest_status.is_none());
}
