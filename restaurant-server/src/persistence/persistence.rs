use crate::models::{
    menu::MenuItemId,
    orders::{TableId, TableOrder, TableOrderItem},
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ReadOrderError {
    #[error("Order id {0} not found.")]
    OrderNotFound(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum CreateOrderError {
    #[error("An order already exists for table id {0}.")]
    OrderAlreadyExistsForTable(String),
}

#[derive(Error, Debug, PartialEq)]

pub enum ReadOrderItemError {
    #[error("Order id {0} not found.")]
    OrderNotFound(String),
    #[error("Order item id {0} not found.")]
    OrderItemNotFound(String),
}

pub trait Persistence {
    async fn create_order(&mut self, table_id: &TableId, items: &[TableOrderItem]) -> Result<&TableOrder, CreateOrderError>;

    async fn find_order(&self, table_id: &TableId) -> Result<&TableOrder, ReadOrderError>;

    async fn update_order(&mut self, table_id: &TableId, new_items: &[TableOrderItem]) -> Result<&TableOrder, ReadOrderError>;

    async fn delete_order(&mut self, table_id: &TableId) -> Result<(), ReadOrderError>;
    async fn delete_order_item(&mut self, table_id: &TableId, item_id: &MenuItemId) -> Result<&TableOrder, ReadOrderItemError>;
}
