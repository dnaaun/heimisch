use super::*;
use crate::raw_traits::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestUser {
    id: u32,
    name: String,
    email: String,
}

impl Table for TestUser {
    const NAME: &'static str = "users";
    type Id = u32;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn index_names() -> &'static [&'static str] {
        &["email"]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestPost {
    id: String,
    title: String,
    content: String,
    user_id: u32,
}

impl Table for TestPost {
    const NAME: &'static str = "posts";
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn index_names() -> &'static [&'static str] {
        &["user_id", "title"]
    }
}

async fn create_test_db() -> SqliteDatabase {
    SqliteDatabase::builder(":memory:")
        .add_table(SqliteDatabase::table_builder::<TestUser>())
        .add_table(SqliteDatabase::table_builder::<TestPost>())
        .build()
        .await
        .expect("Failed to create test database")
}

#[tokio::test]
async fn test_error_conversions() {
    let sqlite_error = rusqlite::Error::InvalidColumnIndex(42);
    let converted: Error = sqlite_error.into();
    match converted {
        Error::Sqlite(_) => (),
        _ => panic!("Expected Sqlite error variant"),
    }

    let json_error = serde_json::from_str::<u32>("not a number").unwrap_err();
    let converted: Error = json_error.into();
    match converted {
        Error::Serde(_) => (),
        _ => panic!("Expected Serde error variant"),
    }
}

#[tokio::test]
async fn test_error_display() {
    let error = Error::Sqlite(rusqlite::Error::InvalidColumnIndex(42));
    let display_str = format!("{}", error);
    assert!(display_str.contains("Sqlite"));
}

#[tokio::test]
async fn test_table_access_put_and_get() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], true).await;
    let table = txn.get_table("users");

    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let id = SerializedId::new_from_row(&user);
    let serialized_user = SerializedObject::from_row(&user).unwrap();

    table.put(&id, &serialized_user).await.unwrap();

    let retrieved = table.get(&id).await.unwrap();
    assert!(retrieved.is_some());

    let retrieved_user: TestUser = serde_json::from_str(&retrieved.unwrap().0).unwrap();
    assert_eq!(retrieved_user, user);
}

#[tokio::test]
async fn test_table_access_get_nonexistent() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], false).await;
    let table = txn.get_table("users");

    let id = SerializedId::new_from_id::<TestUser>(&999);
    let result = table.get(&id).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_table_access_get_all() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], true).await;
    let table = txn.get_table("users");

    let users = vec![
        TestUser {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        TestUser {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    for user in &users {
        let id = SerializedId::new_from_row(user);
        let serialized_user = SerializedObject::from_row(user).unwrap();
        table.put(&id, &serialized_user).await.unwrap();
    }

    let all_users = table.get_all().await.unwrap();
    assert_eq!(all_users.len(), 2);

    let retrieved_users: Vec<TestUser> = all_users
        .into_iter()
        .map(|obj| serde_json::from_str(&obj.0).unwrap())
        .collect();

    assert!(retrieved_users.contains(&users[0]));
    assert!(retrieved_users.contains(&users[1]));
}

#[tokio::test]
async fn test_table_access_delete() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], true).await;
    let table = txn.get_table("users");

    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let id = SerializedId::new_from_row(&user);
    let serialized_user = SerializedObject::from_row(&user).unwrap();

    table.put(&id, &serialized_user).await.unwrap();

    let retrieved = table.get(&id).await.unwrap();
    assert!(retrieved.is_some());

    table.delete(&id).await.unwrap();

    let retrieved_after_delete = table.get(&id).await.unwrap();
    assert!(retrieved_after_delete.is_none());
}

#[tokio::test]
async fn test_table_access_put_replace() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], true).await;
    let table = txn.get_table("users");

    let user1 = TestUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let user2 = TestUser {
        id: 1,
        name: "Alice Updated".to_string(),
        email: "alice.updated@example.com".to_string(),
    };

    let id = SerializedId::new_from_row(&user1);

    table
        .put(&id, &SerializedObject::from_row(&user1).unwrap())
        .await
        .unwrap();
    table
        .put(&id, &SerializedObject::from_row(&user2).unwrap())
        .await
        .unwrap();

    let retrieved = table.get(&id).await.unwrap().unwrap();
    let retrieved_user: TestUser = serde_json::from_str(&retrieved.0).unwrap();
    assert_eq!(retrieved_user, user2);
}

#[tokio::test]
async fn test_transaction_commit() {
    let db = create_test_db().await;
    let txn = SqliteTransaction::new(db.connection.clone()).await;

    let table = txn.get_table("users");
    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let id = SerializedId::new_from_row(&user);
    let serialized_user = SerializedObject::from_row(&user).unwrap();

    table.put(&id, &serialized_user).await.unwrap();
    txn.commit().await.unwrap();

    let new_txn = db.txn(&["users"], false).await;
    let new_table = new_txn.get_table("users");
    let retrieved = new_table.get(&id).await.unwrap();
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_transaction_abort() {
    let db = create_test_db().await;
    let txn = SqliteTransaction::new(db.connection.clone()).await;

    let table = txn.get_table("users");
    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let id = SerializedId::new_from_row(&user);
    let serialized_user = SerializedObject::from_row(&user).unwrap();

    table.put(&id, &serialized_user).await.unwrap();
    txn.abort().await.unwrap();

    let new_txn = db.txn(&["users"], false).await;
    let new_table = new_txn.get_table("users");
    let retrieved = new_table.get(&id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_transaction_sequential_operations() {
    let db = create_test_db().await;
    let txn1 = SqliteTransaction::new(db.connection.clone()).await;
    txn1.commit().await.unwrap();

    // After first transaction is committed, we can start a new one
    let txn2 = SqliteTransaction::new(db.connection.clone()).await;
    let result = txn2.commit().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_transaction_abort_then_new() {
    let db = create_test_db().await;
    let txn1 = SqliteTransaction::new(db.connection.clone()).await;
    txn1.abort().await.unwrap();

    // After first transaction is aborted, we can start a new one
    let txn2 = SqliteTransaction::new(db.connection.clone()).await;
    let result = txn2.abort().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_index_get() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], true).await;
    let table = txn.get_table("users");

    let user = TestUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let id = SerializedId::new_from_row(&user);
    let serialized_user = SerializedObject::from_row(&user).unwrap();
    table.put(&id, &serialized_user).await.unwrap();

    let index = table.index("email");
    let email_value = SerializedValue::from_value(&user.email).unwrap();
    let result = index.get(&email_value).await.unwrap();

    assert!(result.is_some());
    let retrieved_user: TestUser = serde_json::from_str(&result.unwrap().0).unwrap();
    assert_eq!(retrieved_user, user);
}

#[tokio::test]
async fn test_index_get_nonexistent() {
    let db = create_test_db().await;
    let txn = db.txn(&["users"], false).await;
    let table = txn.get_table("users");

    let index = table.index("email");
    let email_value = SerializedValue::from_value(&"nonexistent@example.com").unwrap();
    let result = index.get(&email_value).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_index_get_all_with_value() {
    let db = create_test_db().await;
    let txn = db.txn(&["posts"], true).await;
    let table = txn.get_table("posts");

    let posts = vec![
        TestPost {
            id: "1".to_string(),
            title: "First Post".to_string(),
            content: "Content 1".to_string(),
            user_id: 1,
        },
        TestPost {
            id: "2".to_string(),
            title: "Second Post".to_string(),
            content: "Content 2".to_string(),
            user_id: 1,
        },
        TestPost {
            id: "3".to_string(),
            title: "Third Post".to_string(),
            content: "Content 3".to_string(),
            user_id: 2,
        },
    ];

    for post in &posts {
        let id = SerializedId::new_from_row(post);
        let serialized_post = SerializedObject::from_row(post).unwrap();
        table.put(&id, &serialized_post).await.unwrap();
    }

    txn.commit().await.unwrap();

    let new_txn = db.txn(&["posts"], false).await;
    let new_table = new_txn.get_table("posts");
    let index = new_table.index("user_id");
    let user_id_value = SerializedValue::from_value(&1u32).unwrap();
    let results = index.get_all(Some(&user_id_value)).await.unwrap();

    assert_eq!(results.len(), 2);
    let retrieved_posts: Vec<TestPost> = results
        .into_iter()
        .map(|obj| serde_json::from_str(&obj.0).unwrap())
        .collect();

    assert!(retrieved_posts.iter().all(|p| p.user_id == 1));
}

#[tokio::test]
async fn test_index_get_all_without_value() {
    let db = create_test_db().await;
    let txn = db.txn(&["posts"], true).await;
    let table = txn.get_table("posts");

    let posts = vec![
        TestPost {
            id: "1".to_string(),
            title: "First Post".to_string(),
            content: "Content 1".to_string(),
            user_id: 1,
        },
        TestPost {
            id: "2".to_string(),
            title: "Second Post".to_string(),
            content: "Content 2".to_string(),
            user_id: 2,
        },
    ];

    for post in &posts {
        let id = SerializedId::new_from_row(post);
        let serialized_post = SerializedObject::from_row(post).unwrap();
        table.put(&id, &serialized_post).await.unwrap();
    }

    let index = table.index("user_id");
    let results = index.get_all(None).await.unwrap();

    assert_eq!(results.len(), 2);
}
