use std::collections::HashMap;

use crate::models::{
    menu::MenuItemId,
    orders::{TableId, TableOrder, TableOrderItem},
};

use super::persistence::{CreateOrderError, Persistence, ReadOrderError, ReadOrderItemError};

#[derive(Default, Debug)]
pub struct MemoryPersistence {
    data: HashMap<TableId, TableOrder>,
}

impl MemoryPersistence {
    pub fn new(data: HashMap<TableId, TableOrder>) -> Self {
        return Self { data: data };
    }
}

impl Persistence for MemoryPersistence {
    async fn create_order(&mut self, table_id: &TableId, items: &[TableOrderItem]) -> Result<&TableOrder, CreateOrderError> {
        if self.data.contains_key(&table_id) {
            return Err(CreateOrderError::OrderAlreadyExistsForTable(table_id.to_string()));
        }

        let new_record: TableOrder = TableOrder { table_id: table_id.clone(), items: item_slice_to_hashmap(items) };

        self.data.insert(table_id.clone(), new_record);

        return Ok(self.data.get(&table_id).unwrap());
    }

    async fn find_order(&self, table_id: &TableId) -> Result<&TableOrder, ReadOrderError> {
        return self.data.get(&table_id).ok_or_else(|| ReadOrderError::OrderNotFound(table_id.to_string()));
    }

    async fn update_order(&mut self, table_id: &TableId, new_items: &[TableOrderItem]) -> Result<&TableOrder, ReadOrderError> {
        return self
            .data
            .get_mut(&table_id)
            .ok_or_else(|| ReadOrderError::OrderNotFound(table_id.to_string()))
            .map(|o| {
                o.items = item_slice_to_hashmap(new_items);
                return &*o;
            });
    }

    async fn delete_order(&mut self, table_id: &TableId) -> Result<(), ReadOrderError> {
        return match self.data.remove(&table_id) {
            Some(_) => Ok(()),
            None => Err(ReadOrderError::OrderNotFound(table_id.to_string())),
        };
    }

    async fn delete_order_item(&mut self, table_id: &TableId, item_id: &MenuItemId) -> Result<&TableOrder, ReadOrderItemError> {
        return self
            .data
            .get_mut(&table_id)
            .ok_or_else(|| ReadOrderItemError::OrderNotFound(table_id.to_string()))
            .and_then(|o| {
                return match o.items.remove(&item_id) {
                    Some(_) => Ok(&*o),
                    None => Err(ReadOrderItemError::OrderItemNotFound(item_id.to_string())),
                };
            });
    }
}

pub fn item_slice_to_hashmap(items: &[TableOrderItem]) -> HashMap<MenuItemId, TableOrderItem> {
    return items
        .iter()
        .map(|i| (i.item_id.clone(), i.clone()))
        .collect::<HashMap<MenuItemId, TableOrderItem>>();
}

#[cfg(test)]
pub fn get_underlying_data(memory_persistence: MemoryPersistence) -> HashMap<TableId, TableOrder> {
    return memory_persistence.data;
}
