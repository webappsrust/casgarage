# 🚗 CasGarage

**Self-hosted S3-compatible object storage with integrated administrative web UI**

[![CI](https://github.com/casapps/casgarage/actions/workflows/ci.yml/badge.svg)](https://github.com/casapps/casgarage/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docker](https://img.shields.io/docker/pulls/casapps/casgarage)](https://hub.docker.com/r/casapps/casgarage)
[![Release](https://img.shields.io/github/v/release/casapps/casgarage)](https://github.com/casapps/casgarage/releases)

A comprehensive, production-ready S3-compatible object storage platform built entirely in Rust. Single static binary with **zero external dependencies**, complete web UI, and enterprise features—all in one executable.

**Official Documentation**: [casgarage.readthedocs.io](https://casgarage.readthedocs.io)

---

## 📋 About

CasGarage combines the robust [Garage distributed storage engine](https://garagehq.deuxfleurs.fr/) with a comprehensive administrative web interface, packaged as a single binary. Built for self-hosters, SMBs, and enterprises who need reliable, affordable object storage without cloud vendor lock-in.

### ✨ Key Features

- 🚀 **Single Static Binary** - One executable, truly portable
- 🌐 **Full S3 Compatibility** - Drop-in replacement for AWS S3, MinIO
- 🎨 **Complete Web UI** - Manage everything via browser
- 💾 **Database-Driven Config** - No config files, all in database
- 🔒 **Zero Dependencies** - No databases, services, or libraries required
- 🌍 **Geo-Distribution** - Multi-datacenter replication built-in
- 🔐 **Enterprise Security** - JWT, RBAC, audit logs, Let's Encrypt
- ⚡ **Pure Rust Performance** - Memory-safe, async throughout
- 🐳 **Docker & Kubernetes** - Production-ready containers & manifests
- 🔄 **Multi-Database Support** - SQLite, PostgreSQL, MariaDB, MSSQL, Valkey

### 🎯 Perfect For

- **Self-hosters**: Replace cloud storage with your own infrastructure
- **SMBs**: Affordable, scalable object storage without cloud costs
- **Developers**: S3-compatible testing environment
- **Enterprises**: On-premises, compliant storage solution
- **Edge Computing**: Distributed storage across multiple locations

---

## 🚀 Production Installation

### Quick Install (Linux/macOS/BSD)

```bash
curl -fsSL https://raw.githubusercontent.com/casapps/casgarage/main/scripts/install.sh | sudo bash
```

This will:
- ✅ Download the appropriate binary for your system
- ✅ Create system user and directories
- ✅ Install and configure service (systemd, launchd, rc.d)
- ✅ Set up proper permissions and security

**Post-Install:**
```bash
# Start service
sudo systemctl start casgarage        # Linux (systemd)
sudo launchctl start com.casapps.casgarage  # macOS

# Check status
casgarage --status

# Access admin UI
http://localhost:64900
```

### Linux (with firewall & SELinux)

```bash
curl -fsSL https://raw.githubusercontent.com/casapps/casgarage/main/scripts/linux.sh | sudo bash
```

### macOS (with Homebrew)

```bash
curl -fsSL https://raw.githubusercontent.com/casapps/casgarage/main/scripts/macos.sh | sudo bash
```

### Windows (PowerShell as Administrator)

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/casapps/casgarage/main/scripts/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

**Full installation guide**: [scripts/README.md](scripts/README.md)

---

## 🐳 Docker Production Deployment

### Single Container

```bash
docker run -d \
  --name casgarage \
  -p 172.17.0.1:64900:80 \
  -v ./data:/data \
  -v ./config:/config \
  -e DATA_DIR=/data \
  -e CONFIG_DIR=/config \
  -e LOG_DIR=/var/log/casgarage \
  --restart unless-stopped \
  ghcr.io/casapps/casgarage:latest
```

### Docker Compose

```yaml
# docker-compose.yml
services:
  casgarage:
    image: ghcr.io/casapps/casgarage:latest
    container_name: casgarage
    ports:
      - "172.17.0.1:64900:80"
    volumes:
      - ./rootfs/data/casgarage:/data
      - ./rootfs/config/casgarage:/config
    environment:
      - DATA_DIR=/data
      - CONFIG_DIR=/config
      - LOG_DIR=/var/log/casgarage
    restart: unless-stopped
    networks:
      - casgarage
    healthcheck:
      test: ["/usr/local/bin/casgarage", "--status"]
      interval: 30s
      timeout: 3s
      retries: 3

networks:
  casgarage:
    name: casgarage
    driver: bridge
```

```bash
docker compose up -d
```

**Multi-node cluster**: See [docker/README.md](docker/README.md)

---

## ☸️ Kubernetes Production Deployment

```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/statefulset.yaml
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/ingress.yaml
```

**Full Kubernetes guide**: [k8s/README.md](k8s/README.md)

---

## ⚙️ Configuration

CasGarage uses **database-driven configuration** - no config files needed! Everything is managed via the web UI.

### Command-Line Options

```bash
casgarage --help                      # Show help
casgarage --version                   # Show version
casgarage --status                    # Health check (exit code 0 = healthy)
casgarage --port 8080                 # Single HTTP port
casgarage --port 80,443               # HTTP + HTTPS ports
casgarage --address 192.168.1.100    # Listen address
casgarage --datadir /custom/data     # Data directory
casgarage --configdir /custom/config # Config directory
casgarage --logdir /custom/logs      # Log directory
```

### Environment Variables

```bash
PORT=80,443                          # Port(s) to listen on
SERVER_ADDRESS=0.0.0.0               # Listen address
DATA_DIR=/data                       # Data directory
CONFIG_DIR=/config                   # Config directory
LOG_DIR=/var/log/casgarage          # Log directory
```

### First-Time Setup

1. **Start the server** - It will bind to a random port (64xxx range)
2. **Access web UI** - Navigate to `http://<server-ip>:<port>`
3. **Register first user** - Create your account
4. **Create administrator** - Set up the `administrator` account
5. **Complete setup wizard** - Configure storage, network, S3 keys, etc.

The database becomes the source of truth for all configuration. The web UI allows you to modify all settings at runtime.

### Automatic Let's Encrypt

When running on ports **80,443**, CasGarage automatically:
- Checks for existing certificates in `/etc/letsencrypt/live`
- If not found, requests certificates via Let's Encrypt
- Supports HTTP-01, DNS-01 (all providers), TLS-ALPN-01 challenges
- Automatically renews before expiry
- Stores certificates in `CONFIG_DIR/ssl/certs`

---

## 📊 Monitoring & Operations

### Health Check

```bash
casgarage --status
echo $?  # 0 = healthy, non-zero = unhealthy
```

### Metrics (Prometheus)

```bash
curl http://localhost:64900/api/v1/metrics
```

### Logs

```bash
# Systemd (Linux)
journalctl -u casgarage -f

# Files
tail -f /var/log/casgarage/access.log  # Apache format
tail -f /var/log/casgarage/error.log

# Docker
docker logs -f casgarage
```

### Admin API

```bash
# Health
curl http://localhost:64900/api/v1/health

# Server info
curl http://localhost:64900/api/v1/info

# All endpoints documented at /api/v1/docs (Swagger)
```

---

## 🔐 Security

### Built-in Features

- ✅ **Automatic HTTPS** - Let's Encrypt integration
- ✅ **JWT Authentication** - Secure API access
- ✅ **RBAC** - Role-based access control
- ✅ **Audit Logging** - Track all administrative actions
- ✅ **Encrypted Connections** - TLS 1.3 support
- ✅ **Rate Limiting** - Prevent abuse
- ✅ **IP Whitelisting** - Per-key access control
- ✅ **Security Hardening** - Systemd security policies

### Security Best Practices

1. **Run behind reverse proxy** (Nginx, Caddy, Traefik)
2. **Use ports 80,443** for automatic SSL
3. **Enable 2FA** for administrator account (web UI)
4. **Review audit logs** regularly
5. **Keep binary updated** - `casgarage --version`

---

## 🌍 S3 Compatibility

### AWS CLI

```bash
aws configure set aws_access_key_id YOUR_KEY
aws configure set aws_secret_access_key YOUR_SECRET
aws configure set default.s3.endpoint_url http://localhost:64900

aws s3 mb s3://my-bucket
aws s3 cp file.txt s3://my-bucket/
aws s3 ls s3://my-bucket/
```

### Python boto3

```python
import boto3

s3 = boto3.client(
    's3',
    endpoint_url='http://localhost:64900',
    aws_access_key_id='YOUR_KEY',
    aws_secret_access_key='YOUR_SECRET'
)

s3.create_bucket(Bucket='my-bucket')
s3.upload_file('local.txt', 'my-bucket', 'remote.txt')
```

### Supported Operations

✅ Buckets: create, list, delete, configure
✅ Objects: PUT, GET, DELETE, HEAD
✅ Multipart uploads
✅ Pre-signed URLs
✅ Versioning
✅ Tagging
✅ Policies
✅ CORS
✅ Website hosting

**Full S3 API documentation**: [docs/api/s3-api.md](docs/api/s3-api.md)

---

## 🔄 Upgrading

### Binary Upgrade

```bash
# Download new version
wget https://github.com/casapps/casgarage/releases/latest/download/casgarage-linux-amd64

# Stop service
sudo systemctl stop casgarage

# Replace binary
sudo mv casgarage-linux-amd64 /usr/local/bin/casgarage
sudo chmod +x /usr/local/bin/casgarage

# Start service
sudo systemctl start casgarage
```

### Docker Upgrade

```bash
docker pull ghcr.io/casapps/casgarage:latest
docker compose down
docker compose up -d
```

**Zero-downtime upgrades**: See [docs/guides/upgrading.md](docs/guides/upgrading.md)

---

## 📚 Documentation

- 📖 **Official Docs**: [casgarage.readthedocs.io](https://casgarage.readthedocs.io)
- 🚀 **Quick Start**: [docs/guides/quickstart.md](docs/guides/quickstart.md)
- 📦 **Installation**: [scripts/README.md](scripts/README.md)
- 🐳 **Docker**: [docker/README.md](docker/README.md)
- ☸️ **Kubernetes**: [k8s/README.md](k8s/README.md)
- 🏗️ **Architecture**: [docs/architecture.md](docs/architecture.md)
- 🔌 **API Reference**: [docs/api/](docs/api/)

---

## 🛠️ Development

### Prerequisites

- Rust 1.83+ (stable)
- Docker (for containerized dev environment)
- Trunk (for frontend WASM builds)

### Development Environment

```bash
# Using Docker (recommended - keeps host clean)
docker compose up

# Or using Incus/LXD (Alpine container)
incus launch images:alpine/edge casgarage-dev
incus exec casgarage-dev -- sh
# ... install dependencies and build

# Access at http://localhost:64900
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/casapps/casgarage.git
cd casgarage

# Build everything
make build

# Run tests
make test

# Build Docker image
make docker

# Create release
make release
```

### Project Structure

```
casgarage/
├── src/              # Rust backend source
├── frontend/         # Leptos WASM frontend
├── scripts/          # Production scripts
├── tests/            # Development & test files
├── docker/           # Docker configurations
├── k8s/              # Kubernetes manifests
├── docs/             # Documentation
├── Makefile          # Build automation
├── Dockerfile        # Production Alpine image
└── TODO.md           # Task tracking
```

### Running Tests

```bash
# All tests in Docker
docker compose -f docker-compose.test.yml up

# Individual suites
cargo test --lib                 # Unit tests
cargo test --test '*'            # Integration tests
cargo clippy --all-targets       # Linting
cargo fmt -- --check             # Format check
```

### Code Style

- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **No inline TODOs**: Use `TODO.md` instead
- **Comments**: Explain "why", not "what"
- **Tests**: Required for new features
- **Docs**: Update relevant documentation

**Contributing guide**: [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 🤝 Community & Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/casapps/casgarage/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/casapps/casgarage/discussions)
- 📧 **Email**: casjay@yahoo.com
- 💼 **Author**: Jason Hempstead ([@casapps](https://github.com/casapps))

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

This means you can:
- ✅ Use commercially
- ✅ Modify and distribute
- ✅ Use privately
- ✅ No warranty provided

---

## 🙏 Acknowledgments

Built on top of the excellent **[Garage](https://garagehq.deuxfleurs.fr/)** distributed storage engine by Deuxfleurs.

Special thanks to the Rust community and all contributors!

---

## ⭐ Star History

If you find CasGarage useful, please consider starring the repository!

[![Star History Chart](https://api.star-history.com/svg?repos=casapps/casgarage&type=Date)](https://star-history.com/#casapps/casgarage&Date)

---

**Made with ❤️ and 🦀 Rust**
