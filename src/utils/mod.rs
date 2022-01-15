use linked_hash_map::LinkedHashMap;
use serenity::builder::{CreateActionRow, CreateButton};
use serenity::model::interactions::message_component::ButtonStyle;

pub mod mongo;

pub fn create_action_row_basic (dict: LinkedHashMap<String, String>, command: &str) -> CreateActionRow {
    let mut buttons: Vec<CreateButton> = vec!();
    for d in dict {
        let mut b = CreateButton::default();
        b.custom_id(format!("{}_{}", command, &d.0));
        b.label(d.1);
        b.style(ButtonStyle::Primary);
        buttons.push(b)
    }

    let mut ar = CreateActionRow::default();
    for btn in buttons {
        ar.add_button(btn);
    }
    ar
}
