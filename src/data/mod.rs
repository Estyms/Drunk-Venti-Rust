pub mod elements;
pub mod artifacts;
pub mod items;
pub mod weapons;
pub mod builds;
pub mod characters;
pub mod events;
pub mod shared_structs;
pub mod domains;

#[allow(dead_code)]
pub async fn test() {
    elements::test_elem().await;
    artifacts::test_artifacts().await;
    items::test_items().await;
    weapons::test_weapons().await;
    builds::test_builds().await;
    characters::test_character().await;
    events::test_events().await;
}