// For a given persistence model, return a fixed format for this API version
// in addition to allowing sending extra data to clients that may be more convenient, reducing requests

use crate::models::{menu::get_menu_item, orders::{TableOrder, TableOrderItem}};

#[derive(serde::Serialize)]

pub struct TableOrderViewModel {
    table_id: String,
    items: Vec<TableOrderItemSummaryViewModel>
}

#[derive(serde::Serialize)]

pub struct TableOrderItemSummaryViewModel {
    pub item_id: String,
    pub name: String,
    pub quantity: i32,
    pub total_preparation_time_mins: i32
}

#[derive(serde::Serialize)]
pub struct TableOrderItemDetailViewModel {
    pub item_id: String,
    pub name: String,
    pub quantity: i32,
    pub total_preparation_time_mins: i32,
    pub description: String
}

pub fn to_order_view_model(order: &TableOrder) -> TableOrderViewModel {
    return TableOrderViewModel { 
        table_id: order.table_id.to_string(),
        items: order.items.values().map(|i| to_order_item_summary_view_model(&i)).collect()
    };
}

pub fn to_order_item_summary_view_model(item: &TableOrderItem) -> TableOrderItemSummaryViewModel {
    let menu_item = get_menu_item(&item.item_id);

    return TableOrderItemSummaryViewModel { 
        item_id: item.item_id.to_string(),
        name: menu_item.name,
        quantity: item.quantity, 
        total_preparation_time_mins: item.total_preparation_time_mins
    };
}

pub fn to_order_item_detail_view_model(item: &TableOrderItem) -> TableOrderItemDetailViewModel {
    let menu_item = get_menu_item(&item.item_id);

    return TableOrderItemDetailViewModel { 
        item_id: item.item_id.to_string(),
        name: menu_item.name,
        quantity: item.quantity, 
        total_preparation_time_mins: item.total_preparation_time_mins,
        description: menu_item.description
    };
}
