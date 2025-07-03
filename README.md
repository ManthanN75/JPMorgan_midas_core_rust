🎯 Overview
Midas Core is a high-performance financial transaction processing system inspired by the JPMorgan Chase Forage Virtual Experience Program. Originally designed for Java/Spring + H2, this implementation leverages async Rust, Apache Kafka, and SQLite to create a fault-tolerant, event-driven architecture capable of processing financial transactions in real-time.
✨ Features

🚀 Asynchronous Processing - Built on Tokio for high-performance async operations
📊 Real-time Transaction Processing - Kafka-driven event streaming architecture
🔒 Atomic Operations - SQL transactions ensure data consistency
✅ Account Validation - Comprehensive sender/recipient verification
💰 Balance Management - Automatic balance tracking and updates
🛡️ Fault Tolerance - Graceful error handling and transaction rollback
🔧 Environment Configuration - Easy deployment with .env configuration
📈 Scalable Architecture - Modular design for easy extension

🛠️ Tech Stack
ComponentPurposeVersionRustCore languageLatest stableTokioAsync runtime1.xActix WebWeb framework4.xrdkafkaKafka client0.36+SQLxDatabase toolkit0.7+serdeSerialization1.xdotenvyEnvironment config0.15+
🏗️ Architecture
mermaidgraph TD
    A[Kafka Topic: transactions] --> B[Kafka Consumer]
    B --> C[JSON Deserializer]
    C --> D[Transaction Validator]
    D --> E{Valid Transaction?}
    E -->|Yes| F[Update Balances]
    E -->|No| G[Log & Discard]
    F --> H[SQLite Database]
    G --> I[Error Logs]
    H --> J[Transaction History]
Data Flow

Message Ingestion: Kafka consumer receives transaction messages
Deserialization: JSON messages parsed into Transaction structs
Validation: Verify sender/recipient existence and sufficient balance
Processing: Atomic balance updates using SQL transactions
Storage: Transaction history and updated balances persisted to SQLite

🚀 Quick Start
Prerequisites

Rust (latest stable) - Install here
Apache Kafka - Download
Git - Download

Installation

Clone the repository
bashgit clone https://github.com/ManthanN75/midas-core.git
cd midas-core

Set up environment variables
bashcp .env.example .env
# Edit .env with your configuration

Install dependencies
bashcargo build

Start Kafka (Windows example)
bashcd C:\Ckafka
.\bin\windows\kafka-server-start.bat config\kraft\server.properties

Run the application
bashcargo run


📋 Usage
Sending Transactions
Use the Kafka console producer to send test transactions:
bash# Start Kafka producer
cd C:\Ckafka
.\bin\windows\kafka-console-producer.bat --topic transactions --bootstrap-server localhost:9092
Send a transaction (JSON format):
json{
  "sender_id": "alice",
  "recipient_id": "bob", 
  "amount": 50
}
Database Inspection
View transactions and balances:
bashsqlite3 midas.db
sql-- View all users and balances
SELECT * FROM users;

-- View transaction history
SELECT * FROM transactions;

-- Check specific user balance
SELECT * FROM users WHERE id = 'alice';
🧪 Testing
Manual Testing

Start the application
Send test transactions via Kafka CLI
Monitor logs for processing status
Verify balance updates in SQLite

Example Test Cases
json// Valid transaction
{"sender_id": "alice", "recipient_id": "bob", "amount": 25}

// Invalid - insufficient balance
{"sender_id": "alice", "recipient_id": "bob", "amount": 10000}

// Invalid - non-existent user
{"sender_id": "invalid_user", "recipient_id": "bob", "amount": 10}
📈 Roadmap
✅ Completed (Phase 1-3)

 Basic Actix Web server setup
 Kafka consumer integration
 SQLite database layer
 Transaction validation logic
 Atomic balance updates
 Error handling and logging

🔄 In Progress (Phase 4)

 REST API endpoints
 Account management endpoints
 Transaction history API
 Health check endpoints

🎯 Future Enhancements

 Authentication & authorization
 Rate limiting
 Metrics and monitoring
 Docker containerization
 Integration tests
 Performance benchmarks

🤝 Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

Fork the repository
Create your feature branch (git checkout -b feature/AmazingFeature)
Commit your changes (git commit -m 'Add some AmazingFeature')
Push to the branch (git push origin feature/AmazingFeature)
Open a Pull Request

📄 License
This project is licensed under the MIT License - see the LICENSE file for details.
👨‍💻 Author
Manthan Naglaksh

🐦 Twitter: @ManthanN75
💼 LinkedIn: manthan-naglaksh
🐙 GitHub: @ManthanN75

🙏 Acknowledgments

Inspired by the JPMorgan Chase Forage Virtual Experience Program
Built with the amazing Rust ecosystem
Thanks to the open-source community for the excellent tools and libraries


⭐ Star this repository if you found it helpful!
