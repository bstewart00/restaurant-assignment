use crate::models::{
    menu::{get_preparation_time, MenuItemId},
    orders::{TableId, TableOrderItem},
};

#[derive(serde::Deserialize)]
pub struct ClientNewItem {
    pub item_id: String,
    pub qty: i32,
}

#[derive(serde::Deserialize)]
pub struct CreateOrUpdateOrderParams {
    pub items: Vec<ClientNewItem>,
}

pub fn from_client_table_id(table_id: &str) -> TableId {
    return TableId(table_id.parse().unwrap());
}

pub fn from_client_item_id(item_id: &str) -> MenuItemId {
    return MenuItemId(item_id.parse().unwrap());
}

pub fn from_client_item(new_item: &ClientNewItem) -> TableOrderItem {
    let item_id = from_client_item_id(&new_item.item_id);
    let preparation_time = get_preparation_time(&item_id);

    return TableOrderItem { item_id: item_id, quantity: new_item.qty, total_preparation_time_mins: preparation_time };
}
