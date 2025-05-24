# 🐗 Hogger - Axum Rust Rabbit Mongo(loid)DB

![Hogger Logo](https://i.ibb.co/6cJdq8GD/hogger.jpg)

**Hogger** is an open-source log (or rather, *hog*) aggregator, built in Rust for performance crackheads who want maximum throughput with minimal BS. Currently being used to process stock/crypto scraper data logs in my homelab.

It uses:
- 🦀 **Rust** for blazingly fast stuff
- 🐇 **RabbitMQ** for decoupled and scalable message handling
- 🍃 **MongoDB** for durable and flexible storage

## 🚀 K6 stress test  
![Hogger Stressed](https://i.ibb.co/jvZsz4hw/Screenshot-2025-05-18-135635.png)

## ⚙️ Philosophy

This project is **unopinionated**. That means if you're a grown-up, you're expected to make your own decisions about how you want to handle logs. Hogger doesn’t tell you how to live your life—it just keeps going hog.

## 🚧 Work in Progress

Features are being added, improved, or completely refactored. Contributions are welcome!

## 🧠 Goals

- Decouple log ingestion and processing
- Enable horizontal scaling with RabbitMQ workers
- Provide a blazing-fast backend powered by Rust
- Leave design decisions to the user

## 🐽 Why "Hogger"?

Because it **hogs** logs. Simple.

---

## ✅ What works?

## POST : ::3000/hogs (create a hog)
Mandatory fields:
* log_timestamp
* log_message

Note: log_data accepts any JSON, so it's possible to store and parse through specific log data from the services natively through key, value or both.

Sample payload:
```json
{
    "log_timestamp": "2025-05-18T15:28:34.549Z",
    "log_level": "WARN",
    "log_message": "This is a test log message 919",
    "log_data": {
        "header_id": "Authorization",
        "header_value": "abc-def-asdaaaf"
    },
    "log_type": "security",
    "log_source": "api-gateway",
    "log_source_id": "435",
}
```

Sample reply:
```json
{
    "log_timestamp": "2025-05-18T15:28:34.549Z",
    "log_level": "WARN",
    "log_message": "This is a test log message 919",
    "log_data": {
        "header_id": "Authorization",
        "header_value": "abc-def-asdaaaf"
    },
    "log_type": "security",
    "log_source": "api-gateway",
    "log_source_id": "435",
    "hog_uuid": "de641d8c-9ded-419f-a9e3-ecc90de7afe2",
    "hog_timestamp": "2025-05-18T15:28:34.587Z"
}
```

## GET : ::3000/hogs (get all hogs)

Sample reply:
```json
[    
    {
        "log_timestamp": "2025-05-17T19:40:47.449Z",
        "log_level": "INFO",
        "log_message": "This is a test log message 829",
        "log_data": {
            "key": "value",
            "number": 29.771739567336052
        },
        "log_type": "security",
        "log_source": "data-ingestor",
        "log_source_id": "7373",
        "hog_uuid": "9f4ccd9c-d55a-43f1-b68c-927449961fba",
        "hog_timestamp": "2025-05-17T19:40:47.497343002Z",
        "_id": {
            "$oid": "6828e63f50181961a07748bb"
        }
    },
    {
        "log_timestamp": "2025-05-17T19:40:48.453Z",
        "log_level": "ERROR",
        "log_message": "This is a test log message 17",
        "log_data": {
            "key": "value",
            "number": 17.56451763743483
        },
        "log_type": "security",
        "log_source": "data-ingestor",
        "log_source_id": "4162",
        "hog_uuid": "f5e3ad06-e102-4605-92b8-b1e65827b631",
        "hog_timestamp": "2025-05-17T19:40:48.493633373Z",
        "_id": {
            "$oid": "6828e64050181961a07748bc"
        }
    },
    {
        "log_timestamp": "2025-05-17T19:40:49.367Z",
        "log_level": "DEBUG",
        "log_message": "This is a test log message 249",
        "log_data": {
            "key": "value",
            "number": 43.713470436225535
        },
        "log_type": "system",
        "log_source": "data-ingestor",
        "log_source_id": "4654",
        "hog_uuid": "286fb5c4-d7c4-4c96-89b7-649f5b7d54eb",
        "hog_timestamp": "2025-05-17T19:40:49.405234765Z",
        "_id": {
            "$oid": "6828e64150181961a07748bd"
        }
    }
]
```

## POST : ::3000/hogs/search (adds search fields by field on payload)

Examples:
### returns all system debug log from the data-ingestor
```json
{
    "log_level": "DEBUG",
    "log_type": "system",
    "log_source": "data-ingestor"
}
```
### returns all INFO logs in a specific time range
```json
{
    "log_timestamp_start": "2025-05-18T02:40:44.100Z",
    "log_timestamp_end": "2025-05-18T03:40:44.100Z",
    "log_level": "INFO"
}
```

### returns 2 records of Errors that were logged with timeout between 2 timestamps through a partial message query
```json
{
    "log_level": "ERROR",
    "log_timestamp_start": "2025-05-18T15:00:00.800Z", 
    "log_timestamp_end": "2025-05-18T15:19:00.800Z", 
    "log_message": "timeout",
    "hog_partial": true,
    "hog_limit": 2
}
```

## The rest? Just works.

| Field                  | Sample Value                                   | Description                                      |
|------------------------|------------------------------------------------|--------------------------------------------------|
| hog_uuid               | `"b2f98561-3d7d-4db8-b6ae-2b2b176d9c3e"`       | Match on enriched hog UUID                      |
| hog_limit              | `10`                                           | Limits records returned                         |
| hog_parcial            | `true`                                         | Parcial matches (default is false)              |
| hog_timestamp          | `"2025-05-18T13:45:00.000Z"`                   | Exact match on hog timestamp                    |
| hog_timestamp_start    | `"2025-05-17T00:00:00.000Z"`                   | Start range for hog timestamp                   |
| hog_timestamp_end      | `"2025-05-18T23:59:59.999Z"`                   | End range for hog timestamp                     |
| log_timestamp          | `"2025-05-18T12:30:00.000Z"`                   | Exact match on log timestamp                    |
| log_timestamp_start    | `"2025-05-18T00:00:00.000Z"`                   | Start range for log timestamp                   |
| log_timestamp_end      | `"2025-05-18T23:59:59.999Z"`                   | End range for log timestamp                     |
| log_level              | `"ERROR"`                                      | Match log level                                 |
| log_message            | `"connection failed"`                          | Partial or full text match on log message       |
| log_data               | `{ "user_id": 1234, "debug": true }`           | Nested JSON for structured query                |
| log_data_field         | `"field_name_in_your_json"`                    | Nested JSON field query                         |
| log_data_value         | `"filed_value_in_your_json"`                   | Nested JSON value query (aggregate if no field) |
| log_type               | `"application"`                                | Exact match on log type                         |
| log_source             | `"auth-service"`                               | Exact match on log source                       |
| log_source_id          | `"service-abc-42"`                             | Match on unique service/source ID               |


### Feel free to contribute, file issues, or just stare at the hog 🐗.

