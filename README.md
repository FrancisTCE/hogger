# 🐗 Hogger - Axum Rust Rabbit Mongo(loid)DB

![Hogger Logo](https://i.ibb.co/6cJdq8GD/hogger.jpg)

**Hogger** is an open-source log (or rather, *hog*) aggregator, built in Rust for performance freaks who want maximum throughput with minimal BS. Currently being used to process stock/crypto scraper data logs in my homelab.

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
