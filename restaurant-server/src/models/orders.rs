use std::collections::HashMap;

use super::menu::MenuItemId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub struct TableId(pub i32);
impl std::fmt::Display for TableId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableOrder {
    pub table_id: TableId,
    pub items: HashMap<MenuItemId, TableOrderItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableOrderItem {
    pub item_id: MenuItemId, // could make item id distinct from menu item id, but will assume a table order can only contain one of each menu item
    pub quantity: i32,
    pub total_preparation_time_mins: i32,
}
