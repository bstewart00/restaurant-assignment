# Restaurant API Assignment

## API Summary:

```
POST    /v0/orders/:table_id
- JSON Body: { items: [{ item_id: number, qty: number }] }
- Create initial table order (1 or more items)

PUT    /v0/orders/:table_id
- JSON Body: { items: [{ item_id: number, qty: number }] }
- Modify table order (replaces all items in the order, potentially adding or deleting)

GET     /v0/orders/:table_id
- Get summary of this table order (all items)
GET     /v0/orders/:table_id/items/:item_number
- Get details of specific item in this table order

DELETE  /v0/orders/:table_id/items/:item_number
- Delete item from table order

DELETE  /v0/orders/:table_id
- Delete table order entirely (e.g. the table is empty)

```

Assumptions:
- Items are not automatically removed by the server e.g. after the preparation time. Clients will explicitly make a delete item request.
    - This is how I interpreted the last requirement:
    > in other words, the time does not have to be counted down in real time, only upon item creation and then removed with the item upon item deletion
    - So, the client would periodically check the status of table/items to see if they are ready.
    - In practice, I think the server would notify clients when items have finished preparing.
- Table orders represent a transaction for one group of guests at that table. So after all items from the order are finished, the client would DELETE the table from the "active orders".
- API parameters are valid. Would ideally validate and return 4xx errors

## Running the application:

I developed this on Windows using the docker devcontainer. If you have a rust environment locally you should be able to run it directly.

1. `make toolchain`
2. `make build`
3. `make run-server` in one terminal
4. `make run-client` in another terminal

Tests:
`make test`
