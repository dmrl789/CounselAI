#!/bin/bash

# Counsel AI - Production Deployment Script
# This script handles the complete deployment process for production

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ENV_FILE="${PROJECT_ROOT}/.env"
DOCKER_COMPOSE_FILE="${PROJECT_ROOT}/docker-compose.prod.yml"

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        log_error "This script should not be run as root"
        exit 1
    fi
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    # Check if Docker Compose is installed
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    
    # Check if .env file exists
    if [[ ! -f "$ENV_FILE" ]]; then
        log_error ".env file not found. Please copy .env.example to .env and configure it."
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Validate environment configuration
validate_config() {
    log_info "Validating environment configuration..."
    
    # Source the .env file
    set -a
    source "$ENV_FILE"
    set +a
    
    # Check required variables
    local required_vars=(
        "API_KEY"
        "ENCRYPTION_KEY"
        "GRAFANA_PASSWORD"
        "REDIS_PASSWORD"
    )
    
    for var in "${required_vars[@]}"; do
        if [[ -z "${!var:-}" ]]; then
            log_error "Required environment variable $var is not set"
            exit 1
        fi
    done
    
    # Validate API key length
    if [[ ${#API_KEY} -lt 16 ]]; then
        log_error "API_KEY must be at least 16 characters long"
        exit 1
    fi
    
    # Validate encryption key length
    if [[ ${#ENCRYPTION_KEY} -lt 32 ]]; then
        log_error "ENCRYPTION_KEY must be at least 32 characters long"
        exit 1
    fi
    
    log_success "Configuration validation passed"
}

# Create necessary directories
create_directories() {
    log_info "Creating necessary directories..."
    
    local directories=(
        "$PROJECT_ROOT/data"
        "$PROJECT_ROOT/data/encrypted"
        "$PROJECT_ROOT/data/qdrant"
        "$PROJECT_ROOT/logs"
        "$PROJECT_ROOT/logs/nginx"
        "$PROJECT_ROOT/backups"
        "$PROJECT_ROOT/monitoring"
    )
    
    for dir in "${directories[@]}"; do
        mkdir -p "$dir"
        log_info "Created directory: $dir"
    done
    
    log_success "Directories created successfully"
}

# Set proper permissions
set_permissions() {
    log_info "Setting proper permissions..."
    
    # Set permissions for data directories
    chmod 755 "$PROJECT_ROOT/data"
    chmod 700 "$PROJECT_ROOT/data/encrypted"
    chmod 755 "$PROJECT_ROOT/logs"
    
    # Set permissions for .env file
    chmod 600 "$ENV_FILE"
    
    log_success "Permissions set successfully"
}

# Build Docker images
build_images() {
    log_info "Building Docker images..."
    
    cd "$PROJECT_ROOT"
    
    # Build MCP Gateway
    log_info "Building MCP Gateway image..."
    docker-compose -f "$DOCKER_COMPOSE_FILE" build mcp-gateway
    
    # Build other services if needed
    log_info "Building other service images..."
    docker-compose -f "$DOCKER_COMPOSE_FILE" build
    
    log_success "Docker images built successfully"
}

# Deploy services
deploy_services() {
    log_info "Deploying services..."
    
    cd "$PROJECT_ROOT"
    
    # Start services
    docker-compose -f "$DOCKER_COMPOSE_FILE" up -d
    
    log_success "Services deployed successfully"
}

# Wait for services to be ready
wait_for_services() {
    log_info "Waiting for services to be ready..."
    
    local max_attempts=30
    local attempt=1
    
    while [[ $attempt -le $max_attempts ]]; do
        if curl -f http://localhost:5142/health &> /dev/null; then
            log_success "MCP Gateway is ready"
            break
        fi
        
        if [[ $attempt -eq $max_attempts ]]; then
            log_error "Services failed to start within expected time"
            docker-compose -f "$DOCKER_COMPOSE_FILE" logs
            exit 1
        fi
        
        log_info "Waiting for services... (attempt $attempt/$max_attempts)"
        sleep 10
        ((attempt++))
    done
}

# Run health checks
run_health_checks() {
    log_info "Running health checks..."
    
    # Check MCP Gateway
    if curl -f http://localhost:5142/health &> /dev/null; then
        log_success "MCP Gateway health check passed"
    else
        log_error "MCP Gateway health check failed"
        exit 1
    fi
    
    # Check Qdrant
    if curl -f http://localhost:6333/health &> /dev/null; then
        log_success "Qdrant health check passed"
    else
        log_error "Qdrant health check failed"
        exit 1
    fi
    
    log_success "All health checks passed"
}

# Display deployment information
display_info() {
    log_success "Deployment completed successfully!"
    echo
    echo "Service URLs:"
    echo "  - MCP Gateway API: http://localhost:5142"
    echo "  - Health Check: http://localhost:5142/health"
    echo "  - Metrics: http://localhost:5142/metrics"
    echo "  - Grafana Dashboard: http://localhost:3000"
    echo "  - Prometheus: http://localhost:9090"
    echo
    echo "Default credentials:"
    echo "  - Grafana: admin / $GRAFANA_PASSWORD"
    echo
    echo "Useful commands:"
    echo "  - View logs: docker-compose -f $DOCKER_COMPOSE_FILE logs -f"
    echo "  - Stop services: docker-compose -f $DOCKER_COMPOSE_FILE down"
    echo "  - Restart services: docker-compose -f $DOCKER_COMPOSE_FILE restart"
    echo
}

# Main deployment function
main() {
    log_info "Starting Counsel AI production deployment..."
    
    check_root
    check_prerequisites
    validate_config
    create_directories
    set_permissions
    build_images
    deploy_services
    wait_for_services
    run_health_checks
    display_info
    
    log_success "Deployment completed successfully!"
}

# Run main function
main "$@"