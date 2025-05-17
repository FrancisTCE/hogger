# 🐗 Hogger - Axum Rust Rabbit Mongo(loid)DB

![Hogger Logo](https://i.ibb.co/6cJdq8GD/hogger.jpg)

**Hogger** is an open-source log (or rather, *hog*) aggregator, built in Rust for performance crackheads who want maximum throughput with minimal BS. Currently being used to process stock/crypto scraper data logs in my homelab.

It uses:
- 🦀 **Rust** for concurrency and safety
- 🐇 **RabbitMQ** for decoupled and scalable message handling
- 🍃 **MongoDB** for durable and flexible storage

## ⚙️ Philosophy

This project is **unopinionated**. That means if you're a grown-up, you're expected to make your own decisions about how you want to handle logs. Hogger doesn’t tell you how to live your life—it just keeps going hog.

## 🚧 Work in Progress

Features are being added, improved, or completely refactored. Contributions are welcome!

## 🧠 Goals

- Decouple log ingestion and processing
- Enable horizontal scaling with RabbitMQ workers
- Provide a blazing-fast backend powered by Rust
- Leave design decisions to the user

## 🚀 Getting Started

Documentation coming soon. For now, check out the [Docker setup](./docker-compose.yml) and explore the source code.

## 🐽 Why "Hogger"?

Because it **hogs** logs. Simple.

---

Feel free to contribute, file issues, or just stare at the hog 🐗.

## What works?

Currently 3 endpoints:

POST : ::3000/hogs (create a hog)
Mandatory fields:
* log_timestamp
* log_message

Sample payload:
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
    "hog_timestamp": "2025-05-17T19:40:49.405234765Z"
}

GET : ::3000/hogs (get all hogs)

Sample reply:
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

GET : ::3000/hogs/search (addss search fields by field on payload)

Example:

{
    "log_level": "DEBUG",
    "log_type": "system",
    "log_source": "data-ingestor"
}

returns all system debug log levels for the data-ingestor
