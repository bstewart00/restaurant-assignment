use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::time::SystemTime;

const BASE_URL: &'static str = "http://localhost:9000";

fn current_time() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

fn create_order(thread_id: i32, table_id: i32, client: &reqwest::blocking::Client) {
    let url = format!("{}/v0/orders/{}", BASE_URL, table_id);
    println!("{}|thread[{}]: POST {}", current_time(), thread_id, url);
    let resp = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(
            json!({
                "items": [
                    { "item_id": "1", "qty": 1 },
                    { "item_id": "2", "qty": 1 },
                    { "item_id": "3", "qty": 1 }
                ]
            })
            .to_string(),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}|thread[{}]:     response[{:?}]", current_time(), thread_id, resp);
}

fn update_order(thread_id: i32, table_id: i32, client: &reqwest::blocking::Client) {
    let url = format!("{}/v0/orders/{}", BASE_URL, table_id);
    println!("{}|thread[{}]: PUT {}", current_time(), thread_id, url);
    let resp = client
        .put(url)
        .header(CONTENT_TYPE, "application/json")
        .body(
            json!({
                "items": [
                    { "item_id": "4", "qty": 1 },
                    { "item_id": "3", "qty": 1 },
                    { "item_id": "5", "qty": 1 }
                ]
            })
            .to_string(),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}|thread[{}]:     response[{:?}]", current_time(), thread_id, resp);
}

fn get_order_items(thread_id: i32, table_id: i32, client: &reqwest::blocking::Client) {
    let url = format!("{}/v0/orders/{}", BASE_URL, table_id);
    println!("{}|thread[{}]: GET {}", current_time(), thread_id, url);
    let resp = client.get(url).send().unwrap().text().unwrap();
    println!("{}|thread[{}]:     response[{:?}]", current_time(), thread_id, resp);
}

fn get_order_item_details(thread_id: i32, table_id: i32, item_id: i32, client: &reqwest::blocking::Client) {
    let url = format!("{}/v0/orders/{}/items/{}", BASE_URL, table_id, item_id);
    println!("{}|thread[{}]: GET {}", current_time(), thread_id, url);
    let resp = client.get(url).send().unwrap().text().unwrap();
    println!("{}|thread[{}]:     response[{:?}]", current_time(), thread_id, resp);
}

fn delete_order_item(thread_id: i32, table_id: i32, item_id: i32, client: &reqwest::blocking::Client) {
    let url = format!("{}/v0/orders/{}/items/{}", BASE_URL, table_id, item_id);
    println!("{}|thread[{}]: DELETE {}", current_time(), thread_id, url);
    let resp = client.delete(url).send().unwrap().text().unwrap();
    println!("{}|thread[{}]:     response[{:?}]", current_time(), thread_id, resp);
}

fn delete_order(thread_id: i32, table_id: i32, client: &reqwest::blocking::Client) {
    let url = format!("{}/v0/orders/{}", BASE_URL, table_id);
    println!("{}|thread[{}]: DELETE {}", current_time(), thread_id, url);
    let resp = client.delete(url).send().unwrap().text().unwrap();
    println!("{}|thread[{}]:     response[{:?}]", current_time(), thread_id, resp);
}

fn table_staff_thread(thread_id: i32) {
    // Each staff handles 10 tables and then stops
    // 0:[0..9]
    // 1:[10..19] etc

    let table_start_id = thread_id * 10;
    let table_end_id = table_start_id + 10;
    println!("{}|thread[{}]: Table staff thread started. handling tables [{}, {}]", current_time(), thread_id, table_start_id, table_end_id);

    let client = reqwest::blocking::Client::new();

    for table_id in table_start_id..table_end_id {
        create_order(thread_id, table_id, &client);
        thread::sleep(Duration::from_millis(1000));
        update_order(thread_id, table_id, &client);
        thread::sleep(Duration::from_millis(1000));
        get_order_items(thread_id, table_id, &client);
        thread::sleep(Duration::from_millis(1000));
        get_order_item_details(thread_id, table_id, 3, &client);
        thread::sleep(Duration::from_millis(1000));
        get_order_item_details(thread_id, table_id, 5, &client);
        thread::sleep(Duration::from_millis(1000));
        delete_order_item(thread_id, table_id, 3, &client);
        thread::sleep(Duration::from_millis(1000));
        get_order_items(thread_id, table_id, &client);
        thread::sleep(Duration::from_millis(1000));
        delete_order(thread_id, table_id, &client);
        thread::sleep(Duration::from_millis(1000));
    }

    println!("{}:thread[{}]: Table staff thread finished", current_time(), thread_id);
}

fn dump_persistence(client: &reqwest::blocking::Client) {
    let url = format!("{}/debug/dump_persistence", BASE_URL);
    let resp = client.get(url).send().unwrap().text().unwrap();
    println!("persistence contents: {}", resp);
}

fn main() {
    let thread_count = 10;

    let client = reqwest::blocking::Client::new();
    dump_persistence(&client);

    let threads = (0..thread_count - 1)
        .map(|i| return thread::spawn(move || table_staff_thread(i)))
        .collect::<Vec<_>>();

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }

    dump_persistence(&client);
}
