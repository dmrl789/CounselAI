# Counsel AI - Production Legal Assistant

A comprehensive, privacy-preserving Italian legal assistant system designed for production use. This system provides secure, offline-capable legal analysis and document generation with enterprise-grade security and monitoring.

## 🚀 Production Features

### Core Legal Functionality
- **Interactive Case Intake**: Chat-style CLI for building comprehensive case files
- **AI-Powered Reasoning**: Advanced legal analysis with citation support
- **Document Generation**: Professional DOCX and PDF output using Jinja2 templates
- **Audit Trail**: Cryptographically secure, hash-chained audit ledger

### Security & Privacy
- **End-to-End Encryption**: All data encrypted at rest and in transit
- **Local Processing**: Complete offline capability with local AI models
- **API Authentication**: Secure API key-based authentication
- **Input Validation**: Comprehensive input sanitization and validation
- **Rate Limiting**: Protection against abuse and DoS attacks

### Production Infrastructure
- **Health Monitoring**: Comprehensive health checks and metrics
- **Error Handling**: Robust error handling with detailed logging
- **Configuration Management**: Environment-based configuration system
- **Docker Support**: Containerized deployment with multi-stage builds
- **CI/CD Pipeline**: Automated testing, security scanning, and deployment

## 🛠️ Installation

### Prerequisites
- Python 3.10+
- Node.js 18+
- Rust 1.70+
- Docker (optional)

### Quick Start

1. **Clone and setup Python environment:**
```bash
git clone <repository-url>
cd counsel-ai
python -m venv .venv && source .venv/bin/activate
pip install -e .[test]
```

2. **Install TypeScript dependencies:**
```bash
npm install
```

3. **Build Rust services:**
```bash
cd services/mcp-gateway
cargo build --release
```

### Docker Deployment

```bash
# Build all services
docker-compose -f docker-compose.local.yml build

# Start the full stack
docker-compose -f docker-compose.local.yml up -d
```

## 📖 Usage

### Command Line Interface

1. **Case Intake:**
```bash
counsel-ai intake --case-id HT-2025-0001 > CaseFile.json
```

2. **Generate Legal Opinion:**
```bash
counsel-ai opinion CaseFile.json --out Opinion.json
```

3. **Export Documents:**
```bash
counsel-ai export CaseFile.json Opinion.json
```

### API Usage

Start the MCP Gateway:
```bash
cd services/mcp-gateway
API_KEY=your-secure-api-key cargo run --release
```

**Health Check:**
```bash
curl http://localhost:5142/health
```

**Query Processing:**
```bash
curl -X POST http://localhost:5142/query \
  -H "Authorization: Bearer your-secure-api-key" \
  -H "Content-Type: application/json" \
  -d '{"text": "Legal question about contract law"}'
```

### Desktop Application

Launch the Tauri desktop app:
```bash
cd apps/ui-desktop
npm run tauri dev
```

## 🏗️ Architecture

### MCP Gateway Service
The `services/mcp-gateway` provides a secure REST API with:
- **Hybrid AI Processing**: OpenAI integration with local model fallback
- **Rate Limiting**: Configurable request throttling
- **Authentication**: API key-based security
- **Health Monitoring**: Comprehensive system health checks
- **Input Validation**: Robust sanitization and validation

### Configuration
Environment variables for production deployment:

```bash
# Security
API_KEY=your-secure-api-key-here
ENCRYPTION_KEY=your-32-char-encryption-key

# AI Services
OPENAI_API_KEY=your-openai-key
GPT_MODEL=gpt-4
LOCAL_MODEL_PATH=/models/mistral-7b-instruct.Q4_K_M.gguf

# Infrastructure
BIND_ADDR=0.0.0.0:5142
VECTOR_DB_URL=http://qdrant:6333
STORAGE_PATH=/data/encrypted
LOG_LEVEL=info

# Rate Limiting
RATE_LIMIT_PER_SECOND=10
RATE_LIMIT_BURST_SIZE=20
```

### Local Model Setup
Download a quantized model for offline operation:
```bash
mkdir -p services/mcp-gateway/models
cd services/mcp-gateway/models
wget https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Python tests
pytest tests/ -v --cov=counsel_ai

# Rust tests
cd services/mcp-gateway
cargo test

# TypeScript tests
npm test
```

## 🔒 Security

- **Encryption**: AES-256-GCM encryption for all stored data
- **Authentication**: Secure API key authentication
- **Input Validation**: Comprehensive sanitization and validation
- **Rate Limiting**: Protection against abuse
- **Audit Logging**: Cryptographically secure audit trail

## 📊 Monitoring

- **Health Endpoint**: `/health` for system status
- **Metrics**: `/metrics` for Prometheus monitoring
- **Logging**: Structured logging with configurable levels
- **Error Tracking**: Comprehensive error handling and reporting

## 🚀 Deployment

### Production Checklist
- [ ] Set secure API keys and encryption keys
- [ ] Configure rate limiting appropriately
- [ ] Set up monitoring and alerting
- [ ] Configure backup and disaster recovery
- [ ] Review and update security settings
- [ ] Test all endpoints and functionality

### Docker Production
```bash
# Build production images
docker-compose -f docker-compose.prod.yml build

# Deploy with proper secrets
docker-compose -f docker-compose.prod.yml up -d
```

## 📚 Documentation

- **API Documentation**: Available at `/docs` when running
- **Architecture**: See `docs/architecture/`
- **PRD**: Product requirements in `docs/PRD.md`

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 📄 License

Proprietary - IPPAN Labs
