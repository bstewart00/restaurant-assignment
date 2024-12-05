#[allow(non_snake_case)]

#[cfg(test)]

mod tests {
    use crate::{api::v0::view_models::{TableOrderItemDetailViewModel, TableOrderItemSummaryViewModel, TableOrderViewModel}, app::create_app, persistence::memory_persistence::MemoryPersistence};

    use axum::{
        body::Body,
        http::{self, Request, Response, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::{Service, ServiceExt};

    async fn assert_response(response: Response<Body>, expected_status: StatusCode, expected_body: &str) {
        assert_eq!(expected_status, response.status());
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str: &str = std::str::from_utf8(&body).unwrap();
        assert_eq!(expected_body, body_str);
    }

    fn get_assertable_items_sorted(items: &[TableOrderItemSummaryViewModel]) -> Vec<(String, String, i32)> {
        let mut result = items.iter().map(|i| (i.item_id.clone(), i.name.clone(), i.quantity)).collect::<Vec<(String, String, i32)>>();
        result.sort_by(|a, b| a.cmp(b));
        return result;
    }

    async fn get_body_json(response: Response<Body>) -> Value {
        let body = response.into_body().collect().await.unwrap().to_bytes();
        return serde_json::from_slice(&body).unwrap();
    }

    #[tokio::test]
    async fn get_order__order_does_not_exist__is_404() {
        let sut = create_app(MemoryPersistence::default());

        let response = sut
            .oneshot(Request::builder().uri("/v0/orders/123").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_response(response, StatusCode::NOT_FOUND, "Order id 123 not found.").await;
    }

    #[tokio::test]
    async fn end_to_end_test() {
        let mut sut = create_app(MemoryPersistence::default());

        // No orders initially
        {
            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .oneshot(Request::builder().method(http::Method::GET).uri("/v0/orders/123").body(Body::empty()).unwrap())
                .await
                .unwrap();

            assert_response(response, StatusCode::NOT_FOUND, "Order id 123 not found.").await;
        }

        // Can add order
        {
            let body = json!({
                "items": [
                    { "item_id": "1", "qty": 1 },
                    { "item_id": "2", "qty": 2 },
                    { "item_id": "3", "qty": 3 }
                ]
            });

            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .call(
                    Request::builder()
                        .method(http::Method::POST)
                        .uri("/v0/orders/123")
                        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                        .body(Body::from(serde_json::to_string(&body).unwrap()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(StatusCode::CREATED, response.status());

            let response_json = get_body_json(response).await;
            let response_order: TableOrderViewModel = serde_json::from_value(response_json).unwrap();

            // TODO: Would be easier to assert the whole order object, but would need to refactor the RNG to be seedable
            assert_eq!("123", response_order.table_id);
            assert_eq!(vec![
                ("1".to_string(), "menu item 1".to_string(), 1),
                ("2".to_string(), "menu item 2".to_string(), 2),
                ("3".to_string(), "menu item 3".to_string(), 3)
            ], get_assertable_items_sorted(&response_order.items));
        }

        // Can find the order after creation
        {
            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .call(Request::builder().method(http::Method::GET).uri("/v0/orders/123").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(StatusCode::OK, response.status());

            let response_json = get_body_json(response).await;
            let response_order: TableOrderViewModel = serde_json::from_value(response_json).unwrap();

            assert_eq!("123", response_order.table_id);
            assert_eq!(vec![
                ("1".to_string(), "menu item 1".to_string(), 1),
                ("2".to_string(), "menu item 2".to_string(), 2),
                ("3".to_string(), "menu item 3".to_string(), 3)
            ], get_assertable_items_sorted(&response_order.items));
        }

        // Can get item details
        {
            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
            .await.unwrap()
            .call(Request::builder().method(http::Method::GET).uri("/v0/orders/123/items/2").body(Body::empty()).unwrap())
            .await
            .unwrap();
            assert_eq!(StatusCode::OK, response.status());

            let response_json: Value = get_body_json(response).await;
            let response_order_item_details: TableOrderItemDetailViewModel = serde_json::from_value(response_json).unwrap();

            assert_eq!("2", response_order_item_details.item_id);
            assert_eq!("menu item 2", response_order_item_details.name);
            assert_eq!("menu item desc 2", response_order_item_details.description);
            assert_eq!(2, response_order_item_details.quantity);

            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .oneshot(Request::builder().method(http::Method::GET).uri("/v0/orders/123/items/404").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(StatusCode::NOT_FOUND, response.status());
            assert_response(response, StatusCode::NOT_FOUND, "Order item id 404 not found.").await;
        }

        // Can update the order with deleted and new items
        {
            let body = json!({
                "items": [
                    { "item_id": "1", "qty": 1 },
                    { "item_id": "4", "qty": 4 },
                ]
            });

            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .call(
                    Request::builder()
                        .method(http::Method::PUT)
                        .uri("/v0/orders/123")
                        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                        .body(Body::from(serde_json::to_string(&body).unwrap()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(StatusCode::OK, response.status());

            let response_json = get_body_json(response).await;
            let response_order: TableOrderViewModel = serde_json::from_value(response_json).unwrap();

            assert_eq!("123", response_order.table_id);
            assert_eq!(vec![
                ("1".to_string(), "menu item 1".to_string(), 1),
                ("4".to_string(), "menu item 4".to_string(), 4),
            ], get_assertable_items_sorted(&response_order.items));
        }

        // Can delete a single item
        {
            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .call(Request::builder().method(http::Method::DELETE).uri("/v0/orders/123/items/4").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(StatusCode::OK, response.status());

            let response_json = get_body_json(response).await;
            let response_order: TableOrderViewModel = serde_json::from_value(response_json).unwrap();

            assert_eq!("123", response_order.table_id);
            assert_eq!(vec![
                ("1".to_string(), "menu item 1".to_string(), 1),
            ], response_order.items.iter().map(|i| (i.item_id.clone(), i.name.clone(), i.quantity)).collect::<Vec<(String, String, i32)>>());
        }

        // Can delete the order
        {
            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .call(Request::builder().method(http::Method::DELETE).uri("/v0/orders/123").body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(StatusCode::NO_CONTENT, response.status());
        }

        // Can no longer find the order
        {
            let response = ServiceExt::<Request<Body>>::ready(&mut sut)
                .await.unwrap()
                .oneshot(Request::builder().method(http::Method::GET).uri("/v0/orders/123").body(Body::empty()).unwrap())
                .await
                .unwrap();

            assert_response(response, StatusCode::NOT_FOUND, "Order id 123 not found.").await;
        }
    }
}
