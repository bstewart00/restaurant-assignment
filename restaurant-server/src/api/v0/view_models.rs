/*
For a given persistence model, return a fixed format for this API version, in addition to allowing sending extra data to clients.
*/

use crate::models::orders::{TableOrder, TableOrderItem};

pub fn to_order_view_model(order: TableOrder) {

}

pub fn to_order_item_summary_view_model(item: TableOrderItem) {

}

pub fn to_order_item_detail_view_model(item: TableOrderItem) {

}
