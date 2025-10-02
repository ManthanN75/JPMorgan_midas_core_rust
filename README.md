 🦀 JPMorgan Midas Core Backend (Rust Reimplementation)

This project is a complete rewrite of the JPMorgan Chase **Forage Virtual Experience Program's backend system**, originally designed in Java/Spring. Instead, it is built entirely in **Rust** to explore safe, performant, and async-first backend architecture with real-world components like **Kafka**, **SQLite**, and **REST APIs**.

> ✅ All 5 phases Completed — Kafka → SQLite → Incentive API → Actix REST integration

---

 🔥 Why Rust?

- 🛡️ Memory safety without garbage collection
- ⚡ High performance and async concurrency  
- 🧠 Transparent logic without framework abstraction (no Spring, no JPA)

---

 📦 Tech Stack

| Tech           | Purpose                              |
|----------------|--------------------------------------|
| **Rust**        | Core backend language                |
| **Tokio**       | Async runtime                        |
| **Actix-web**   | REST API framework                   |
| **rdkafka**     | Kafka message consumer               |
| **SQLx**        | Async SQLite access                  |
| **serde**       | JSON serialization/deserialization   |
| **dotenvy**     | Load environment configs from `.env` |
| **reqwest**     | Call external REST APIs              |
| **sqlite3**     | Local database for transaction records |

---
 🧱 Architecture Overview

```
Kafka Topic ("transactions")
           ↓
Kafka Consumer (async)
           ↓
Deserialize JSON → Transaction
           ↓
Validate: sender exists, has funds, recipient exists
           ↓
Call external REST Incentive API → incentive amount
           ↓
Update balances: deduct from sender, add to recipient + incentive
           ↓
Record full transaction in SQLite DB
           ↓
Query balances via CLI or REST (Phase 5)
```

---

 🧩 Features by Phase

 ✅ Phase 1: Setup & DB
- `.env` support for config
- SQLite schema creation (`users`, `transactions`)
- Rust modules initialized

 ✅ Phase 2: Kafka Integration
- Async Kafka consumer with `rdkafka`
- Parses incoming JSON transactions
- Inserts raw data into SQLite

### ✅ Phase 3: Validation & Balance Tracking
- Validates:
  - Sender exists
  - Recipient exists
  - Sender has enough balance
- Atomic balance updates with SQLx transactions
- Final balance of `waldorf` used for submission

### ✅ Phase 4: Incentive API Integration
- Created mock Incentive API using Actix
- POSTs transaction to `/incentive`
- Receives incentive amount (e.g., 10% of amount)
- Adds incentive only to recipient
- Saves incentive alongside transaction

### ✅ Phase 5: REST API (Actix)
- `GET /balance/{user_id}` → returns user balance
- `GET /transactions?user_id=bob` → filters transactions
- Optional pagination (`limit`, `offset`)
- CORS and JSON error handling

---

## 💻 Getting Started

### ✅ Prerequisites
- Rust + Cargo installed
- Kafka running locally (KRaft mode)
- SQLite installed (CLI optional)

### 📁 Run Kafka Server
```bash
cd C:\Ckafka
.\bin\windows\kafka-server-start.bat config\kraft\server.properties
```

### 🦀 Run Rust Backend
```bash
cargo run --bin midas_core_rust_jpmc
```

### 🌐 Run Mock Incentive API
```bash
cargo run --bin mock_incentive_api
```

### 🧪 Test with Kafka CLI
```bash
cd C:\Ckafka
.\bin\windows\kafka-console-producer.bat --topic transactions --bootstrap-server localhost:9092
```

Then paste:
```json
{"sender_id": "alice", "recipient_id": "bob", "amount": 100}
```

### 🧠 Example Query (SQLite)
```bash
sqlite3 midas.db
> SELECT * FROM users;
> SELECT * FROM transactions;
```

---

## 🚀 API Endpoints

### Balance Endpoint
```bash
GET /balance/{user_id}
```

**Response:**
```json
{
  "user_id": "bob",
  "balance": 1050.0
}
```

### Transactions Endpoint
```bash
GET /transactions?user_id=bob&limit=10&offset=0
```

**Response:**
```json
[
  {
    "id": 1,
    "sender_id": "alice",
    "recipient_id": "bob",
    "amount": 100.0,
    "incentive": 10.0,
    "timestamp": "2025-01-15T10:30:00Z"
  }
]
```

---

## 📂 Project Structure

```
src/
├── main.rs              # Main Kafka consumer loop
├── mock_incentive_api.rs # Mock incentive API server
├── models/
│   └── transaction.rs   # Transaction struct + serde
├── database/
│   └── sqlite.rs        # SQLite schema + queries
├── kafka/
│   └── consumer.rs      # Kafka consumer logic
├── incentive/
│   └── api.rs           # Incentive API client
└── rest/
    └── api.rs           # Actix REST endpoints
```

---

## 🔧 Configuration

Create a `.env` file in the project root:

```env
DATABASE_URL=sqlite:midas.db
KAFKA_BROKERS=localhost:9092
KAFKA_TOPIC=transactions
INCENTIVE_API_URL=http://localhost:8081
REST_API_PORT=8080
```

---

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
# Start Kafka and mock API first
cargo run --bin mock_incentive_api &
cargo run --bin midas_core_rust_jpmc &

# Send test transaction
echo '{"sender_id": "alice", "recipient_id": "bob", "amount": 100}' | \
  C:\Ckafka\bin\windows\kafka-console-producer.bat --topic transactions --bootstrap-server localhost:9092

# Check balance
curl http://localhost:8080/balance/bob
```

---

## 🎯 Key Learnings

- **Async Rust**: Tokio's async runtime handles concurrent Kafka consumption and REST requests
- **Error Handling**: Extensive use of `Result<T, E>` for safe error propagation
- **Memory Safety**: No null pointer exceptions or memory leaks
- **Performance**: Zero-copy deserialization with serde and efficient SQLite queries
- **Concurrency**: Lock-free message processing with async/await

---

## 🚧 Future Improvements

- [ ] Add Redis for caching user balances
- [ ] Implement distributed tracing with `tracing`
- [ ] Add comprehensive logging with `log` crate
- [ ] Deploy with Docker containers
- [ ] Add authentication middleware
- [ ] Implement circuit breaker for external API calls
- [ ] Add metrics collection with Prometheus

---

## 📜 License

This project is created for educational purposes as part of the JPMorgan Chase Forage Virtual Experience Program.

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📞 Support

For questions or issues, please create an issue in the GitHub repository or contact the development team.

---

**Built with ❤️ and 🦀 Rust**

Inspired by the JPMorgan Chase Forage Virtual Experience Program
Built with the amazing Rust ecosystem
Thanks to the open-source community for the excellent tools and libraries


⭐ Star this repository if you found it helpful!

🧑‍💻 Author
Manthan Naglaksh
Rust Developer | Web3 Enthusiast | Systems Thinker

🌐 LinkedIn - Manthan Naglaksh
💻 GitHub
🐦 Twitter / Manthan75
