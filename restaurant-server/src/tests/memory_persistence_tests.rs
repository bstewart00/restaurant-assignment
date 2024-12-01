#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        models::{
            menu::MenuItemId,
            orders::{TableId, TableOrder, TableOrderItem},
        },
        persistence::{
            memory_persistence::{get_underlying_data, item_slice_to_hashmap, MemoryPersistence},
            persistence::{CreateOrderError, Persistence, ReadOrderError, ReadOrderItemError},
        },
    };

    #[tokio::test]
    async fn create_order__no_existing_order__is_created() {
        let table_id = TableId(123);
        let items = vec![
            TableOrderItem { item_id: MenuItemId(1), name: "item1".to_string(), quantity: 1, total_preparation_time_mins: 10 },
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
        ];
        let mut sut = MemoryPersistence::default();

        let result = sut.create_order(&table_id, &items).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(TableId(123), result.table_id);
        assert_eq!(3, result.items.len());
        assert_eq!(items[0], *result.items.get(&items[0].item_id).unwrap());
        assert_eq!(items[1], *result.items.get(&items[1].item_id).unwrap());
        assert_eq!(items[2], *result.items.get(&items[2].item_id).unwrap());
        assert_eq!(1, get_underlying_data(sut).len());
    }

    #[tokio::test]
    async fn create_order__table_has_existing_order__is_error() {
        let table_id = TableId(123);
        let items = vec![
            TableOrderItem { item_id: MenuItemId(1), name: "item1".to_string(), quantity: 1, total_preparation_time_mins: 10 },
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
        ];
        let mut data: HashMap<TableId, TableOrder> = HashMap::new();
        data.insert(table_id.clone(), TableOrder { table_id: table_id.clone(), items: HashMap::default() });
        let mut sut = MemoryPersistence::new(data);

        let result = sut.create_order(&table_id, &items).await;

        assert!(result.is_err());
        assert_eq!(CreateOrderError::OrderAlreadyExistsForTable(table_id.to_string()), result.unwrap_err());
        assert_eq!(1, get_underlying_data(sut).len());
    }

    #[tokio::test]
    async fn update_order__no_existing_order__is_error() {
        let table_id = TableId(123);
        let items = vec![
            TableOrderItem { item_id: MenuItemId(1), name: "item1".to_string(), quantity: 1, total_preparation_time_mins: 10 },
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
        ];
        let mut sut = MemoryPersistence::default();

        let result = sut.update_order(&table_id, &items).await;

        assert!(result.is_err());
        assert_eq!(ReadOrderError::OrderNotFound(table_id.to_string()), result.unwrap_err());
        assert_eq!(0, get_underlying_data(sut).len());
    }

    #[tokio::test]
    async fn update_order__existing_order__replaces_items() {
        let table_id = TableId(123);
        let existing_items = vec![
            TableOrderItem { item_id: MenuItemId(1), name: "item1".to_string(), quantity: 1, total_preparation_time_mins: 10 },
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
        ];
        let mut data: HashMap<TableId, TableOrder> = HashMap::new();
        data.insert(table_id.clone(), TableOrder { table_id: table_id.clone(), items: item_slice_to_hashmap(&existing_items) });

        let mut sut = MemoryPersistence::new(data);

        let table_id = TableId(123);
        let new_items = vec![
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
            TableOrderItem { item_id: MenuItemId(4), name: "item4".to_string(), quantity: 1, total_preparation_time_mins: 13 },
            TableOrderItem { item_id: MenuItemId(5), name: "item5".to_string(), quantity: 1, total_preparation_time_mins: 14 },
        ];

        let result = sut.update_order(&table_id, &new_items).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(TableId(123), result.table_id);
        assert_eq!(4, result.items.len());
        assert_eq!(new_items[0], *result.items.get(&new_items[0].item_id).unwrap());
        assert_eq!(new_items[1], *result.items.get(&new_items[1].item_id).unwrap());
        assert_eq!(new_items[2], *result.items.get(&new_items[2].item_id).unwrap());
        assert_eq!(new_items[3], *result.items.get(&new_items[3].item_id).unwrap());
        assert_eq!(1, get_underlying_data(sut).len());
    }

    #[tokio::test]
    async fn find_order__unknown_id__is_error() {
        let table_id = TableId(123);
        let sut = MemoryPersistence::default();

        let result = sut.find_order(&table_id).await;

        assert!(result.is_err());
        assert_eq!(ReadOrderError::OrderNotFound(table_id.to_string()), result.unwrap_err());
    }

    #[tokio::test]
    async fn find_order__id_exists__returns_order() {
        let table_id = TableId(123);
        let mut data: HashMap<TableId, TableOrder> = HashMap::new();

        let expected_order = TableOrder { table_id: table_id.clone(), items: HashMap::default() };
        data.insert(table_id.clone(), expected_order.clone());

        let sut = MemoryPersistence::new(data);

        let result = sut.find_order(&table_id).await;

        assert!(result.is_ok());
        assert_eq!(expected_order, *result.unwrap());
    }

    #[tokio::test]
    async fn delete_order__order_does_not_exist__is_error() {
        let mut sut = MemoryPersistence::default();

        let table_id = TableId(123);
        let result = sut.delete_order(&table_id).await;

        assert!(result.is_err());
        assert_eq!(ReadOrderError::OrderNotFound(table_id.to_string()), result.unwrap_err());
    }

    #[tokio::test]
    async fn delete_order__order_exists__is_deleted() {
        let table_id = TableId(123);
        let mut data: HashMap<TableId, TableOrder> = HashMap::new();
        data.insert(table_id.clone(), TableOrder { table_id: table_id.clone(), items: HashMap::default() });
        let mut sut = MemoryPersistence::new(data);

        let table_id = TableId(123);
        let result = sut.delete_order(&table_id).await;

        assert!(result.is_ok());
        assert_eq!(0, get_underlying_data(sut).len());
    }

    #[tokio::test]
    async fn delete_order_item__order_does_not_exist__is_error() {
        let mut sut = MemoryPersistence::default();

        let table_id = TableId(123);
        let item_id = MenuItemId(1);
        let result = sut.delete_order_item(&table_id, &item_id).await;

        assert!(result.is_err());
        assert_eq!(ReadOrderItemError::OrderNotFound(table_id.to_string()), result.unwrap_err());
    }

    #[tokio::test]
    async fn delete_order_item__order_item_does_not_exist__is_error() {
        let table_id = TableId(123);
        let mut data: HashMap<TableId, TableOrder> = HashMap::new();
        let existing_items = vec![
            TableOrderItem { item_id: MenuItemId(1), name: "item1".to_string(), quantity: 1, total_preparation_time_mins: 10 },
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
        ];
        data.insert(table_id.clone(), TableOrder { table_id: table_id.clone(), items: item_slice_to_hashmap(&existing_items) });
        let mut sut = MemoryPersistence::new(data);

        let item_id = MenuItemId(9999);
        let result = sut.delete_order_item(&table_id, &item_id).await;

        assert!(result.is_err());
        assert_eq!(ReadOrderItemError::OrderItemNotFound(item_id.to_string()), result.unwrap_err());
        assert_eq!(3, get_underlying_data(sut).get(&table_id).unwrap().items.len());
    }

    #[tokio::test]
    async fn delete_order_item__order_and_item_exists__is_deleted() {
        let table_id = TableId(123);
        let mut data: HashMap<TableId, TableOrder> = HashMap::new();
        let existing_items = vec![
            TableOrderItem { item_id: MenuItemId(1), name: "item1".to_string(), quantity: 1, total_preparation_time_mins: 10 },
            TableOrderItem { item_id: MenuItemId(2), name: "item2".to_string(), quantity: 1, total_preparation_time_mins: 11 },
            TableOrderItem { item_id: MenuItemId(3), name: "item3".to_string(), quantity: 1, total_preparation_time_mins: 12 },
        ];
        data.insert(table_id.clone(), TableOrder { table_id: table_id.clone(), items: item_slice_to_hashmap(&existing_items) });
        let mut sut = MemoryPersistence::new(data);

        let item_id = MenuItemId(2);
        let result = sut.delete_order_item(&table_id, &item_id).await;

        assert!(result.is_ok());

        let mut order_item_ids = result.unwrap().items.iter().map(|kv| kv.1.item_id.clone()).collect::<Vec<MenuItemId>>();
        order_item_ids.sort();

        assert_eq!(vec![MenuItemId(1), MenuItemId(3)], order_item_ids);

        let mut underlying_item_ids = get_underlying_data(sut)
            .get(&table_id)
            .unwrap()
            .items
            .iter()
            .map(|kv| kv.1.item_id.clone())
            .collect::<Vec<MenuItemId>>();
        underlying_item_ids.sort();
        assert_eq!(vec![MenuItemId(1), MenuItemId(3)], underlying_item_ids);
    }
}
