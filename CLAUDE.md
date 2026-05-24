# CasGarage - Self-Hosted S3-Compatible Object Storage with Administrative Web UI

**IMPLEMENTATION STATUS**: v0.1.0 Core Complete - Production-Ready Foundation ✅

## Project Overview

**CasGarage** is a comprehensive, self-hosted S3-compatible object storage platform with integrated Garage storage engine and a complete administrative web interface. Built entirely in Rust as a single static binary with zero external dependencies, it provides enterprise-grade object storage with consumer-level simplicity for self-hosters, SMBs, and enterprises.

**Repository**: `casapps/casgarage`
**License**: MIT
**Language**: Rust (100%)
**Target Users**: Self-hosted community, SMB IT teams, enterprise infrastructure teams, developers
**Official Docs**: https://casgarage.readthedocs.io
**Current Version**: 0.1.0 (Core Implementation Complete)
**Compilation Status**: ✅ Verified Working

## Core Philosophy & Architecture Decisions

- **Single Static Binary**: Complete application in one executable
- **Zero External Dependencies**: No configuration files - everything in database
- **Pure Rust Implementation**: 4,200+ lines of production Rust code
- **Database-Driven Configuration**: SQLite primary, PostgreSQL/MySQL/Valkey optional
  - ✅ **IMPLEMENTED**: Automatic failover to SQLite cache if external DB fails
  - ✅ **IMPLEMENTED**: Read-only maintenance mode with self-healing
- **Multi-Platform Support**: Linux, Windows, macOS, FreeBSD (x86_64, ARM64)
- **Minimal CLI**: Server managed via web UI, not CLI commands
- **Smart Logic Only**: NO AI/ML - everything is deterministic algorithms
- **Security First**: Let's Encrypt built-in, Argon2 passwords, JWT auth, RBAC
- **API-First Design**: Every UI action available via REST API
- **Scoped Routing**: /, /user, /admin, /api/v1 with route mirroring
- **Memory Safety**: Rust's guarantees prevent common security vulnerabilities

## Target Replacement

**Primary**: MinIO, AWS S3, Wasabi, Backblaze B2  
**Secondary**: Ceph RGW, SeaweedFS, OpenStack Swift

## Key Differentiators

1. **Unified Rust Architecture**: Native integration with Garage, no FFI overhead
2. **Web-First Administration**: Full UI for all Garage operations
3. **Zero Configuration Files**: Everything in database, portable across systems
4. **Geo-Distribution Built-In**: Multi-site replication in single binary
5. **Lightweight & Fast**: Memory-safe performance, runs on Raspberry Pi to enterprise servers
6. **Smart Defaults**: Production-ready configuration out of box
7. **True Static Binary**: No runtime dependencies, not even libc (musl on Linux)

---

## Feature Categories

### 1. Core Storage Engine (Embedded Garage)

#### 1.1 S3 API Compatibility
- **Full S3 Protocol Support**
  - Buckets: Create, list, delete, configure
  - Objects: PUT, GET, DELETE, HEAD operations
  - Multipart uploads with resume capability
  - Pre-signed URLs (GET and PUT)
  - Bucket versioning
  - Object tagging
  - Bucket policies (S3 policy JSON)
  - CORS configuration
  - Website hosting mode
  - ListObjectsV2 pagination
  - Range requests for partial downloads
  
- **Advanced S3 Features**
  - Server-side encryption (SSE-S3, SSE-C)
  - Object lifecycle rules
  - Bucket notifications (webhooks)
  - Object locking (WORM compliance)
  - Bucket replication configuration
  - Access logging to separate bucket
  - Metadata preservation
  - Custom headers support

#### 1.2 Storage Backend
- **Data Organization**
  - Distributed hash ring for data placement
  - Configurable replication factor (1-5 copies)
  - Erasure coding option for space efficiency
  - Automatic data balancing across nodes
  - Content-addressed storage (deduplication)
  - Compression (LZ4, Zstandard, optional)
  
- **Block Storage**
  - Configurable block size (512KB - 16MB)
  - Multiple data directories support
  - Directory capacity limits and quotas
  - RAID-aware data placement
  - SSD/HDD tier awareness
  - Automatic repair on corruption detection

#### 1.3 Geo-Distribution & Replication
- **Multi-Site Architecture**
  - 3+ datacenter support with automatic failover
  - Per-bucket replication configuration
  - Global namespace (single S3 endpoint)
  - Regional endpoints with geo-routing
  - Eventual consistency model
  - Conflict resolution strategies
  
- **Replication Management**
  - Real-time replication monitoring
  - Manual replication on-demand
  - Bandwidth throttling per-site
  - Replication lag alerts
  - Cross-region data transfer tracking
  - Site priority configuration

### 2. Administrative Web Interface

#### 2.1 Dashboard & Monitoring
- **Overview Dashboard**
  - Total storage used/available by site
  - Object count and bucket count
  - Request rate (GET/PUT/DELETE per second)
  - Bandwidth usage (ingress/egress)
  - Active connections and client IPs
  - Cluster health status
  - Recent errors and warnings
  - Top buckets by size and requests
  
- **Real-Time Metrics**
  - Live request graph (last 1h/24h/7d)
  - Storage growth over time
  - Per-bucket usage breakdown
  - Operation latency percentiles (p50, p95, p99)
  - Network throughput graphs
  - Disk I/O utilization
  - Memory and CPU usage

#### 2.2 Bucket Management
- **Bucket Operations**
  - Create bucket wizard with validation
  - List all buckets with search/filter
  - Bucket details page (size, objects, created date)
  - Delete bucket (with safety confirmation)
  - Rename bucket (with object migration)
  - Bucket ACL management (public/private)
  - Website hosting configuration
  - CORS policy editor (JSON/form)
  
- **Bucket Configuration**
  - Versioning enable/disable/suspend
  - Lifecycle rules builder (UI + JSON)
  - Replication rules (destination bucket, IAM role)
  - Object locking settings (retention mode, days/years)
  - Encryption configuration (default, per-operation)
  - Access logging destination
  - Quota limits (size/object count)
  - Notification webhooks (S3 events)

#### 2.3 Object Browser
- **File Manager Interface**
  - Folder-style navigation (S3 prefix simulation)
  - Breadcrumb navigation
  - Upload files (drag-drop, multi-file)
  - Upload folders (recursive)
  - Download files/folders (as ZIP)
  - Delete objects (single/bulk)
  - Copy/move objects between buckets
  - Rename objects (with copy-delete)
  
- **Object Details**
  - Object metadata viewer (content-type, size, etag)
  - Custom metadata editor (x-amz-meta-*)
  - Version history (if versioning enabled)
  - Download previous versions
  - Object ACL viewer/editor
  - Pre-signed URL generator (expiry time)
  - Object tagging (key-value pairs)
  - Storage class information

#### 2.4 Access Control & Security
- **User Management**
  - Create S3 access keys (access key ID + secret)
  - List all access keys with last used date
  - Revoke/delete access keys
  - Rotate keys with overlap period
  - Key permissions (read-only, read-write, admin)
  - Per-key bucket restrictions
  - IP whitelist per key
  - Rate limiting per key
  
- **Policy Management**
  - Bucket policy editor (JSON with validation)
  - Policy templates (public read, authenticated write, etc.)
  - IAM policy simulator (test before apply)
  - Access logs viewer (who accessed what, when)
  - Audit trail (all admin actions logged)
  - Role-based access control (RBAC) for web UI
  - Two-factor authentication for admin
  - Session timeout configuration

#### 2.5 Cluster Management
- **Node Administration**
  - Add new node to cluster
  - Remove node (with data migration)
  - Node status (online/offline/degraded)
  - Disk usage per node
  - Replication state per node
  - Manual data rebalancing
  - Node metadata (hostname, IP, capacity)
  - Drain mode (stop writes, allow reads)
  
- **Site Configuration**
  - Add new datacenter/site
  - Configure site replication zones
  - Network topology mapping
  - Site-to-site bandwidth limits
  - Priority weighting for read requests
  - Disaster recovery failover rules
  - Geographic location metadata
  - Site maintenance mode

#### 2.6 Settings & Configuration
- **Server Settings**
  - S3 endpoint configuration (HTTP/HTTPS, port)
  - Admin UI port and bind address
  - SSL/TLS certificate management (upload, Let's Encrypt)
  - Database backup/restore
  - Garbage collection schedule
  - Log level (debug, info, warn, error)
  - Request timeout values
  - Maximum object size limit
  
- **Performance Tuning**
  - Thread pool size (workers)
  - Connection pool limits
  - Cache size (metadata, data blocks)
  - Compression algorithm selection
  - Checksum algorithm (MD5, SHA256, BLAKE2)
  - Background job concurrency
  - Network buffer sizes
  - Disk I/O scheduler hints

#### 2.7 Monitoring & Alerts
- **Alert Configuration**
  - Disk space threshold alerts (%, GB remaining)
  - Replication lag warnings
  - Node offline notifications
  - Error rate thresholds
  - Bandwidth limit warnings
  - Quota exceeded alerts
  - SSL certificate expiry warnings
  - Custom metric alerts (Prometheus-style)
  
- **Notification Channels**
  - Email notifications (SMTP)
  - Webhook (POST to URL)
  - Slack integration
  - PagerDuty integration
  - Discord webhooks
  - Microsoft Teams
  - Custom script execution

#### 2.8 Backup & Recovery
- **Data Protection**
  - Snapshot entire cluster state
  - Backup bucket data (to external S3, filesystem)
  - Point-in-time recovery configuration
  - Scheduled backups (cron-style)
  - Backup retention policies
  - Incremental backup support
  - Backup encryption (AES-256)
  - Restore wizard (select snapshot, destination)
  
- **Disaster Recovery**
  - Cold backup to tape/archive
  - Cross-region replication verification
  - Automated failover testing
  - Recovery time objective (RTO) tracking
  - Recovery point objective (RPO) tracking
  - Backup integrity verification
  - Restore testing automation

### 3. API & Integration

#### 3.1 Admin REST API
- **Comprehensive API Coverage**
  - Every UI operation available via API
  - RESTful design (GET, POST, PUT, DELETE)
  - JSON request/response bodies
  - API key authentication (Bearer token)
  - Rate limiting per API key
  - OpenAPI 3.0 specification
  - Interactive API documentation (Swagger UI)
  - API versioning (/api/v1/)
  
- **API Endpoints**
  - `/api/v1/buckets` - Bucket CRUD operations
  - `/api/v1/objects` - Object operations
  - `/api/v1/keys` - Access key management
  - `/api/v1/cluster` - Node and site management
  - `/api/v1/metrics` - Prometheus-compatible metrics
  - `/api/v1/health` - Health check endpoint
  - `/api/v1/config` - Configuration management
  - `/api/v1/logs` - Query application logs

#### 3.2 CLI Tool
- **Command-Line Interface**
  - Subcommands for all operations (casgarage bucket create, etc.)
  - Interactive mode for guided operations
  - Config file support (YAML, JSON)
  - Environment variable configuration
  - Shell completion (bash, zsh, fish)
  - Batch operations (JSON/CSV input)
  - Output formats (JSON, YAML, table, CSV)
  - Progress bars for long operations
  
- **CLI Examples**
  ```bash
  casgarage server start
  casgarage bucket create my-bucket --public
  casgarage key create --name "backup-system" --read-only
  casgarage cluster add-node --ip 192.168.1.100
  casgarage backup create --destination /mnt/backups
  casgarage metrics --format prometheus
  ```

#### 3.3 Integration Features
- **Webhook Support**
  - S3 event notifications (PutObject, DeleteObject, etc.)
  - Admin action webhooks (user created, bucket deleted)
  - Custom event filters
  - Retry logic with exponential backoff
  - Webhook delivery logs
  - Signature verification (HMAC)
  
- **External Authentication**
  - LDAP/Active Directory integration
  - OAuth2/OIDC (Google, GitHub, Okta)
  - SAML 2.0 support
  - API key federation (trust external keys)
  - JWT token validation
  - Custom authentication plugins

### 4. Technical Architecture

#### 4.1 Application Structure
```
casgarage/
├── src/                        # Rust backend source code
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library interface
│   ├── garage/                 # Garage storage engine (fork/integration)
│   │   ├── api/                # S3 API implementation
│   │   ├── data/               # Data storage layer
│   │   ├── model/              # Data models
│   │   └── rpc/                # Inter-node RPC
│   ├── web/                    # Web server & admin API
│   │   ├── server.rs           # Axum web server
│   │   ├── routes/             # API route handlers
│   │   ├── middleware/         # Auth, logging, CORS
│   │   └── handlers/           # Request handlers
│   ├── admin/                  # Admin functionality
│   │   ├── auth.rs             # Authentication system
│   │   ├── users.rs            # User management
│   │   ├── metrics.rs          # Monitoring & metrics
│   │   └── backup.rs           # Backup operations
│   ├── db/                     # Configuration database
│   │   ├── schema.rs           # Database schema
│   │   ├── migrations.rs       # Schema migrations
│   │   └── queries.rs          # Database queries
│   └── cli/                    # CLI implementation
│       ├── commands/           # CLI commands
│       └── parser.rs           # Argument parsing
├── frontend/                   # Leptos WASM frontend (built separately)
│   ├── src/                    # Rust source code
│   │   ├── main.rs             # Frontend entry point
│   │   ├── lib.rs              # Library and App component
│   │   ├── components/         # Reusable UI components
│   │   ├── pages/              # Application pages/views
│   │   └── api/                # API client
│   ├── style/                  # CSS stylesheets (dark theme)
│   │   ├── main.css            # Base styles
│   │   ├── theme.css           # Dark theme colors
│   │   ├── components.css      # Component styles
│   │   └── modal.css           # Modal styles
│   ├── index.html              # HTML entry point
│   ├── Trunk.toml              # Trunk build configuration
│   └── Cargo.toml              # Frontend dependencies
├── tests/                      # All test and development files
│   ├── integration/            # Integration tests
│   ├── api/                    # API endpoint tests
│   ├── cli/                    # CLI command tests
│   ├── dev.sh                  # Development server (hot reload)
│   └── test.sh                 # Run all tests
├── scripts/                    # Production scripts only
│   ├── build.sh                # Unified build script (local + cross-platform)
│   ├── release.sh              # Release packaging (POSIX)
│   ├── install.sh              # Unix/Linux/BSD/macOS installer (POSIX)
│   ├── install.ps1             # Windows PowerShell installer
│   ├── uninstall.sh            # Unix/Linux/BSD/macOS uninstaller
│   ├── uninstall.ps1           # Windows PowerShell uninstaller
│   └── README.md               # Script documentation
├── docker/                     # Docker configurations
│   ├── docker-compose.prod.yml # 3-node production cluster
│   └── README.md               # Docker documentation
├── k8s/                        # Kubernetes manifests
│   ├── namespace.yaml          # Namespace definition
│   ├── statefulset.yaml        # StatefulSet with 3 replicas
│   ├── service.yaml            # Services (Admin, S3, Metrics)
│   ├── ingress.yaml            # Ingress with TLS
│   ├── configmap.yaml          # Configuration
│   └── README.md               # Kubernetes documentation
├── docs/                       # Documentation
│   ├── README.md               # Documentation index
│   ├── architecture.md         # System architecture
│   ├── guides/                 # User guides
│   ├── api/                    # API documentation
│   └── deployment/             # Deployment guides
├── .github/                    # GitHub Actions workflows
│   └── workflows/              # CI/CD pipelines
│       ├── ci.yml              # Continuous integration
│       ├── release.yml         # Release automation
│       └── docker.yml          # Docker image building
├── Dockerfile                  # Production multi-stage build
├── Dockerfile.dev              # Development with hot reload
├── docker-compose.yml          # Development environment
├── .dockerignore               # Docker build exclusions
├── .gitignore                  # Git exclusions
├── .rustfmt.toml               # Rust formatting configuration
├── rust-toolchain.toml         # Rust toolchain specification
├── Cargo.toml                  # Workspace and main dependencies
├── TODO.md                     # Project task tracking (no inline TODOs)
├── README.md                   # Project README
├── LICENSE                     # MIT License
├── CONTRIBUTING.md             # Contribution guidelines
└── CLAUDE.md                   # This file - comprehensive spec
```

**Key Directory Principles:**
- **`src/`** - All Rust backend source code
- **`frontend/`** - All frontend source code (Leptos WASM)
- **`tests/`** - All test files and development scripts
- **`scripts/`** - Production scripts only (self-contained, POSIX-compliant)
- **`docker/`** - Docker-related configurations
- **`k8s/`** - Kubernetes manifests
- **`docs/`** - All documentation
- **`/tmp/casgarage`** - Project-scoped temporary files (never system `/tmp` directly)

#### 4.2 Core Dependencies

**Web Framework & Async Runtime**
- `tokio` - Async runtime (multi-threaded)
- `axum` - Web framework (fast, ergonomic, type-safe)
- `tower` - Middleware and service composition
- `tower-http` - HTTP-specific middleware (CORS, compression, tracing)
- `hyper` - HTTP implementation

**Database & Storage**
- `rusqlite` - SQLite embedded database (configuration)
- `sled` - Alternative embedded DB (for HA scenarios)
- `sqlx` - Async SQL toolkit with compile-time verification

**S3 & Object Storage** (Garage dependencies)
- `aws-sdk-s3` - For S3 protocol compatibility testing
- `bytes` - Efficient byte buffer handling
- `sha2`, `md5` - Checksums and hashing
- `hex` - Hex encoding/decoding

**Serialization & Data Formats**
- `serde` - Serialization framework
- `serde_json` - JSON support
- `serde_yaml` - YAML configuration
- `toml` - TOML configuration
- `bincode` - Binary serialization for internal protocols

**Authentication & Security**
- `argon2` - Password hashing
- `jsonwebtoken` - JWT tokens
- `ring` or `rustls` - TLS/SSL
- `uuid` - Unique identifiers
- `rand` - Cryptographic random numbers

**CLI & Terminal**
- `clap` - Command-line argument parsing (derive API)
- `indicatif` - Progress bars
- `console` - Terminal styling
- `comfy-table` - Table formatting

**Monitoring & Logging**
- `tracing` - Structured logging and diagnostics
- `tracing-subscriber` - Log formatting and filtering
- `prometheus` - Metrics collection
- `sysinfo` - System information (CPU, memory, disk)

**Frontend Embedding**
- `rust-embed` - Embed static assets into binary
- `mime_guess` - MIME type detection

**Utilities**
- `anyhow` - Error handling
- `thiserror` - Custom error types
- `chrono` - Date and time
- `regex` - Regular expressions
- `humantime` - Human-readable durations

#### 4.3 Garage Integration Strategy

**Option 1: Fork Garage** (Recommended)
- Fork `deuxfleurs-org/garage` repository
- Add admin API endpoints directly to Garage codebase
- Extend data models for UI requirements
- Build as unified application

**Option 2: Garage as Dependency**
- Use Garage as a library crate
- Wrap Garage's internal APIs
- Run Garage in same process
- Share configuration database

**Benefits of Pure Rust Integration:**
- Direct access to Garage internals (no RPC overhead)
- Shared memory space for metrics and monitoring
- Single-threaded executor sharing
- Native error handling (no FFI marshaling)
- Type-safe API contracts
- Zero-cost abstractions

#### 4.4 Frontend Options

**Option A: Yew Framework** (Pure Rust WASM)
- Pros: Single language, type safety end-to-end, fast
- Cons: Smaller ecosystem, fewer components
- Build: Compiled to WebAssembly, embedded in binary

**Option B: Leptos Framework** (Pure Rust WASM)
- Pros: Fine-grained reactivity, SSR support, modern
- Cons: Newer framework, evolving API
- Build: Compiled to WASM, embedded via trunk

**Option C: React/TypeScript** (Traditional)
- Pros: Mature ecosystem, many components, familiar
- Cons: Separate build toolchain, JavaScript needed
- Build: Webpack/Vite build, embedded as static assets

**Recommended**: Start with React for rapid development, migrate to Leptos for long-term maintenance if desired.

#### 4.5 Multi-Platform Build

**Supported Platforms**
- Linux: x86_64-unknown-linux-musl (fully static)
- Linux: aarch64-unknown-linux-musl (ARM64)
- Linux: armv7-unknown-linux-musleabihf (Raspberry Pi)
- Linux: arm-unknown-linux-musleabihf (ARMv6)
- Windows: x86_64-pc-windows-msvc
- Windows: aarch64-pc-windows-msvc (ARM64)
- macOS: x86_64-apple-darwin (Intel)
- macOS: aarch64-apple-darwin (Apple Silicon)
- FreeBSD: x86_64-unknown-freebsd
- OpenBSD: x86_64-unknown-openbsd (via cross)

**Build System**
```bash
# Cross-compilation using cargo and cross
cargo install cross

# Build for all targets
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl
cross build --release --target x86_64-pc-windows-msvc
cross build --release --target x86_64-apple-darwin

# Strip binaries for smaller size
strip target/x86_64-unknown-linux-musl/release/casgarage

# Compression with UPX (optional, 40-60% size reduction)
upx --best --lzma target/*/release/casgarage
```

**Build Configuration** (Cargo.toml)
```toml
[profile.release]
opt-level = "z"          # Optimize for size
lto = true               # Link-time optimization
codegen-units = 1        # Better optimization
strip = true             # Strip symbols
panic = "abort"          # Smaller binary

[profile.release-fast]
inherits = "release"
opt-level = 3            # Optimize for speed over size
```

**Static Linking** (musl on Linux)
- No glibc dependency
- Truly portable binaries
- Works on any Linux distro (even Alpine)
- Use `musl-gcc` wrapper for C dependencies

#### 4.6 Database Schema

**Configuration Database** (SQLite via rusqlite)
```sql
-- Server configuration
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- S3 access keys
CREATE TABLE access_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    access_key_id TEXT UNIQUE NOT NULL,
    secret_key TEXT NOT NULL,  -- Argon2 hashed
    name TEXT NOT NULL,
    permissions TEXT NOT NULL,  -- JSON: {buckets: [], read: bool, write: bool}
    ip_whitelist TEXT,          -- JSON array
    rate_limit INTEGER,         -- requests per minute
    created_at INTEGER NOT NULL,
    last_used_at INTEGER,
    enabled BOOLEAN DEFAULT 1
);

-- Buckets (metadata)
CREATE TABLE buckets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    created_at INTEGER NOT NULL,
    created_by INTEGER REFERENCES access_keys(id),
    is_public BOOLEAN DEFAULT 0,
    versioning_enabled BOOLEAN DEFAULT 0,
    website_mode BOOLEAN DEFAULT 0,
    quota_bytes INTEGER,
    quota_objects INTEGER,
    config TEXT  -- JSON: CORS, lifecycle, etc.
);

-- Cluster nodes
CREATE TABLE nodes (
    id TEXT PRIMARY KEY,  -- Node UUID
    hostname TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    datacenter TEXT,
    capacity_bytes INTEGER,
    status TEXT NOT NULL,  -- online, offline, draining
    metadata TEXT,         -- JSON
    last_seen INTEGER NOT NULL
);

-- Replication sites
CREATE TABLE sites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    location TEXT,
    priority INTEGER DEFAULT 100,
    bandwidth_limit INTEGER,  -- bytes per second
    enabled BOOLEAN DEFAULT 1
);

-- Admin users (web UI access)
CREATE TABLE admin_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT,
    role TEXT NOT NULL,  -- admin, operator, viewer
    totp_secret TEXT,
    created_at INTEGER NOT NULL,
    last_login INTEGER
);

-- Audit log
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    user_id INTEGER REFERENCES admin_users(id),
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    details TEXT,  -- JSON
    ip_address TEXT,
    user_agent TEXT
);

-- Backup snapshots
CREATE TABLE backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    size_bytes INTEGER,
    status TEXT NOT NULL,  -- pending, completed, failed
    destination TEXT NOT NULL,
    metadata TEXT  -- JSON
);

-- Alert rules
CREATE TABLE alerts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    condition TEXT NOT NULL,  -- JSON
    enabled BOOLEAN DEFAULT 1,
    notification_channels TEXT,  -- JSON array
    last_triggered INTEGER
);
```

#### 4.7 API Architecture

**REST API Design** (Axum routes)
```rust
// API v1 routes
Router::new()
    // Health & info
    .route("/api/v1/health", get(health_check))
    .route("/api/v1/info", get(server_info))
    
    // Buckets
    .route("/api/v1/buckets", get(list_buckets).post(create_bucket))
    .route("/api/v1/buckets/:name", get(get_bucket).delete(delete_bucket).patch(update_bucket))
    .route("/api/v1/buckets/:name/objects", get(list_objects))
    .route("/api/v1/buckets/:name/policy", get(get_policy).put(set_policy))
    
    // Access keys
    .route("/api/v1/keys", get(list_keys).post(create_key))
    .route("/api/v1/keys/:id", get(get_key).delete(revoke_key).patch(update_key))
    
    // Cluster management
    .route("/api/v1/cluster/nodes", get(list_nodes).post(add_node))
    .route("/api/v1/cluster/nodes/:id", get(get_node).delete(remove_node))
    .route("/api/v1/cluster/status", get(cluster_status))
    
    // Metrics & monitoring
    .route("/api/v1/metrics", get(prometheus_metrics))
    .route("/api/v1/metrics/dashboard", get(dashboard_metrics))
    .route("/api/v1/logs", get(query_logs))
    
    // Backup & restore
    .route("/api/v1/backups", get(list_backups).post(create_backup))
    .route("/api/v1/backups/:id/restore", post(restore_backup))
    
    // Configuration
    .route("/api/v1/config", get(get_config).patch(update_config))
    
    // Middleware
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(CorsLayer::permissive())
            .layer(middleware::from_fn(auth_middleware))
    )
```

**Authentication Flow**
1. Login: `POST /api/v1/auth/login` → JWT token
2. All API requests include: `Authorization: Bearer <token>`
3. Token validation via middleware
4. Token expiry: 24 hours (configurable)
5. Refresh token: `POST /api/v1/auth/refresh`

#### 4.8 CLI Implementation

**Command Structure** (Clap derive)
```rust
#[derive(Parser)]
#[command(name = "casgarage")]
#[command(about = "S3-compatible object storage with web UI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Server {
        #[arg(short, long, default_value = "0.0.0.0:3900")]
        bind: String,
        
        #[arg(short, long, default_value = "casgarage.db")]
        database: PathBuf,
    },
    
    /// Bucket operations
    Bucket {
        #[command(subcommand)]
        action: BucketCommands,
    },
    
    /// Access key operations
    Key {
        #[command(subcommand)]
        action: KeyCommands,
    },
    
    /// Cluster operations
    Cluster {
        #[command(subcommand)]
        action: ClusterCommands,
    },
    
    /// Backup operations
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },
    
    /// Export metrics
    Metrics {
        #[arg(short, long, default_value = "prometheus")]
        format: String,
    },
}
```

**CLI Examples**
```bash
# Start server
casgarage server --bind 0.0.0.0:3900

# Bucket management
casgarage bucket create my-bucket
casgarage bucket list
casgarage bucket delete my-bucket --force

# Key management
casgarage key create --name backup-service --read-write
casgarage key list
casgarage key revoke abc123

# Cluster operations
casgarage cluster status
casgarage cluster add-node --ip 192.168.1.100
casgarage cluster rebalance

# Backups
casgarage backup create --name "daily-$(date +%Y%m%d)"
casgarage backup list
casgarage backup restore --id 42

# Metrics
casgarage metrics --format json
casgarage metrics --format prometheus > /var/lib/node_exporter/casgarage.prom
```

---

## 5. Implementation Phases

### Phase 1: Core Foundation (Weeks 1-4)
**Goal**: Basic functional S3 server with minimal UI

**Tasks**:
- [ ] Fork Garage and set up project structure
- [ ] Implement basic Axum web server
- [ ] Create SQLite schema and migrations
- [ ] Build S3 API pass-through to Garage
- [ ] Implement access key authentication
- [ ] Create basic CLI (server start, key create)
- [ ] Docker image for testing

**Deliverables**:
- Single binary that starts Garage + API server
- S3-compatible endpoints working (PUT/GET/DELETE objects)
- Basic key management
- CLI for server control

### Phase 2: Admin API (Weeks 5-8)
**Goal**: Complete REST API for all operations

**Tasks**:
- [ ] Implement all API endpoints (buckets, keys, cluster)
- [ ] Add JWT authentication
- [ ] Build admin user system
- [ ] Create audit logging
- [ ] Implement metrics collection (Prometheus)
- [ ] Add OpenAPI documentation
- [ ] Write API integration tests

**Deliverables**:
- Full REST API with authentication
- API documentation (Swagger UI)
- Comprehensive CLI commands
- Test suite (unit + integration)

### Phase 3: Web Frontend (Weeks 9-14)
**Goal**: Complete administrative web interface

**Tasks**:
- [ ] Set up React/TypeScript project (or Leptos)
- [ ] Build dashboard with metrics
- [ ] Create bucket management UI
- [ ] Implement object browser
- [ ] Build access key management
- [ ] Add cluster status page
- [ ] Implement settings/configuration UI
- [ ] Create alert configuration
- [ ] Embed frontend in binary

**Deliverables**:
- Full web UI accessible at `/`
- Responsive design (mobile-friendly)
- All CRUD operations via UI
- Real-time metrics dashboard

### Phase 4: Advanced Features (Weeks 15-20)
**Goal**: Enterprise features and polish

**Tasks**:
- [ ] Implement backup/restore system
- [ ] Add LDAP/OAuth2 authentication
- [ ] Build webhook system
- [ ] Create replication monitoring
- [ ] Implement quota management
- [ ] Add lifecycle policy editor
- [ ] Build notification system (email, Slack, etc.)
- [ ] Performance optimization
- [ ] Security audit

**Deliverables**:
- Production-ready feature set
- Security hardened
- Performance benchmarks
- Documentation

### Phase 5: Multi-Platform & Release (Weeks 21-24)
**Goal**: Build system and public release

**Tasks**:
- [ ] Set up GitHub Actions CI/CD
- [ ] Configure cross-compilation for all platforms
- [ ] Build and test on all targets
- [ ] Create installation scripts
- [ ] Write comprehensive documentation
- [ ] Create Docker Compose examples
- [ ] Build Kubernetes manifests
- [ ] Release v1.0.0

**Deliverables**:
- Binaries for all platforms
- Docker images (Docker Hub, GitHub Container Registry)
- Documentation site
- Community launch (Reddit, HN, forums)

---

## 6. Configuration & Deployment

### 6.1 First-Run Setup

**Initial Launch**:
```bash
# Download binary
wget https://github.com/casapps/casgarage/releases/latest/download/casgarage-linux-amd64
chmod +x casgarage-linux-amd64
mv casgarage-linux-amd64 /usr/local/bin/casgarage

# Start server (auto-creates database)
casgarage server

# First-time setup wizard (interactive)
# - Creates admin user
# - Generates first access key
# - Configures S3 endpoint
# - Sets up SSL (optional Let's Encrypt)
```

**Environment Variables**:
```bash
CASGARAGE_BIND=0.0.0.0:3900          # Server bind address
CASGARAGE_S3_PORT=3901                # S3 API port
CASGARAGE_ADMIN_PORT=3902             # Admin UI port
CASGARAGE_DATABASE=/data/casgarage.db # Database path
CASGARAGE_DATA_DIR=/data/blocks       # Block storage directory
CASGARAGE_REPLICATION_FACTOR=3        # Number of data copies
CASGARAGE_LOG_LEVEL=info              # debug, info, warn, error
CASGARAGE_TLS_CERT=/path/to/cert.pem  # TLS certificate
CASGARAGE_TLS_KEY=/path/to/key.pem    # TLS private key
```

**Configuration File** (casgarage.toml - optional)
```toml
[server]
bind = "0.0.0.0:3900"
s3_port = 3901
admin_port = 3902
database = "/data/casgarage.db"
data_dir = "/data/blocks"

[cluster]
replication_factor = 3
node_id = "auto"  # Auto-generate or specify UUID

[storage]
block_size = "1MB"
compression = "zstd"
max_object_size = "5TB"

[security]
admin_session_timeout = "24h"
key_rotation_days = 90
require_tls = false

[monitoring]
metrics_enabled = true
metrics_port = 9090
log_level = "info"
```

### 6.2 Docker Deployment

**Docker Compose** (Single Node)
```yaml
version: '3.8'

services:
  casgarage:
    image: casapps/casgarage:latest
    ports:
      - "3900:3900"  # Admin UI
      - "3901:3901"  # S3 API
      - "9090:9090"  # Metrics
    volumes:
      - ./data:/data
      - ./config:/config
    environment:
      CASGARAGE_REPLICATION_FACTOR: 1
      CASGARAGE_LOG_LEVEL: info
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "casgarage", "health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

**Docker Compose** (3-Node Cluster)
```yaml
version: '3.8'

services:
  casgarage-node1:
    image: casapps/casgarage:latest
    ports:
      - "3900:3900"
      - "3901:3901"
    volumes:
      - ./node1:/data
    environment:
      CASGARAGE_NODE_ID: node1
      CASGARAGE_BOOTSTRAP_PEERS: casgarage-node2:3900,casgarage-node3:3900
      CASGARAGE_REPLICATION_FACTOR: 3
    networks:
      - casgarage-net
    
  casgarage-node2:
    image: casapps/casgarage:latest
    ports:
      - "4900:3900"
      - "4901:3901"
    volumes:
      - ./node2:/data
    environment:
      CASGARAGE_NODE_ID: node2
      CASGARAGE_BOOTSTRAP_PEERS: casgarage-node1:3900,casgarage-node3:3900
      CASGARAGE_REPLICATION_FACTOR: 3
    networks:
      - casgarage-net
    
  casgarage-node3:
    image: casapps/casgarage:latest
    ports:
      - "5900:3900"
      - "5901:3901"
    volumes:
      - ./node3:/data
    environment:
      CASGARAGE_NODE_ID: node3
      CASGARAGE_BOOTSTRAP_PEERS: casgarage-node1:3900,casgarage-node2:3900
      CASGARAGE_REPLICATION_FACTOR: 3
    networks:
      - casgarage-net

networks:
  casgarage-net:
    driver: bridge
```

### 6.3 Kubernetes Deployment

**StatefulSet** (3 replicas)
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: casgarage
spec:
  serviceName: casgarage
  replicas: 3
  selector:
    matchLabels:
      app: casgarage
  template:
    metadata:
      labels:
        app: casgarage
    spec:
      containers:
      - name: casgarage
        image: casapps/casgarage:latest
        ports:
        - containerPort: 3900
          name: admin
        - containerPort: 3901
          name: s3
        - containerPort: 9090
          name: metrics
        env:
        - name: CASGARAGE_NODE_ID
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        - name: CASGARAGE_REPLICATION_FACTOR
          value: "3"
        volumeMounts:
        - name: data
          mountPath: /data
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 100Gi
```

### 6.4 Systemd Service

**Unit File** (/etc/systemd/system/casgarage.service)
```ini
[Unit]
Description=CasGarage S3 Object Storage
After=network.target

[Service]
Type=simple
User=casgarage
Group=casgarage
ExecStart=/usr/local/bin/casgarage server
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=casgarage

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/casgarage

[Install]
WantedBy=multi-user.target
```

**Installation**:
```bash
# Create user
sudo useradd -r -s /bin/false casgarage

# Create directories
sudo mkdir -p /var/lib/casgarage/{data,blocks}
sudo chown -R casgarage:casgarage /var/lib/casgarage

# Install service
sudo systemctl daemon-reload
sudo systemctl enable casgarage
sudo systemctl start casgarage

# Check status
sudo systemctl status casgarage
sudo journalctl -u casgarage -f
```

---

## 7. Security Considerations

### 7.1 Authentication & Authorization

**Multi-Layer Security**:
1. **S3 API**: Access key ID + secret (HMAC-SHA256 signing)
2. **Admin API**: JWT tokens with short expiry
3. **Web UI**: Session cookies (HttpOnly, Secure, SameSite)
4. **RBAC**: Role-based permissions (admin, operator, viewer)
5. **2FA**: TOTP for admin accounts (optional)

**Password Requirements**:
- Minimum 12 characters
- Argon2id hashing (PHC format)
- Password rotation reminders
- Failed login attempt limiting (rate limit + lockout)

### 7.2 Network Security

**TLS/SSL**:
- Automatic Let's Encrypt support
- Custom certificate upload
- TLS 1.2+ only (no SSLv3, TLS 1.0/1.1)
- Strong cipher suites (ECDHE, AES-GCM)
- HSTS header support

**Firewall Rules**:
```bash
# Allow S3 API (external)
ufw allow 3901/tcp

# Allow admin UI (internal only)
ufw allow from 192.168.0.0/16 to any port 3900 proto tcp

# Allow metrics (Prometheus)
ufw allow from <prometheus-ip> to any port 9090 proto tcp
```

### 7.3 Data Protection

**At Rest**:
- Optional filesystem encryption (LUKS, ZFS encryption)
- Transparent application-level encryption (AES-256-GCM)
- Encrypted backups

**In Transit**:
- TLS for all external connections
- Authenticated RPC between cluster nodes
- Encrypted replication streams

**Access Control**:
- Bucket policies (S3-compatible JSON)
- ACLs per object
- IP whitelisting per access key
- Temporary credentials (STS-like)

### 7.4 Audit & Compliance

**Logging**:
- All API requests logged (structured JSON)
- Admin action audit trail
- Access logs per bucket
- Failed authentication attempts
- Log retention policies

**Compliance Features**:
- Object immutability (WORM mode)
- Legal hold support
- Data retention policies
- Audit report generation (CSV, JSON)
- GDPR-compliant data deletion

---

## 8. Performance & Scalability

### 8.1 Performance Targets

**Single Node**:
- 1000+ requests/sec (mixed read/write)
- < 50ms latency (p99) for small objects
- 1GB/s throughput (on gigabit network)
- 100k+ objects per bucket

**3-Node Cluster**:
- 5000+ requests/sec
- Linear scaling up to 10 nodes
- Automatic load balancing
- Geographic distribution support

### 8.2 Optimization Techniques

**Caching**:
- Metadata cache (in-memory LRU)
- Block cache for frequently accessed data
- DNS caching for cluster resolution
- Connection pooling

**Async I/O**:
- Tokio multi-threaded runtime
- io_uring on Linux 5.10+ (async filesystem)
- Async SQLite queries (tokio-rusqlite)
- Non-blocking RPC between nodes

**Memory Management**:
- Zero-copy where possible (sendfile, splice)
- Efficient byte buffer handling (bytes crate)
- Memory-mapped files for large objects
- Configurable buffer sizes

### 8.3 Horizontal Scaling

**Adding Nodes**:
1. Start new node with cluster address
2. Automatic ring membership
3. Data rebalancing (gradual, rate-limited)
4. No downtime required

**Removing Nodes**:
1. Mark node as "draining"
2. Migrate data to remaining nodes
3. Wait for replication completion
4. Remove from cluster

**Geographic Distribution**:
- Multi-datacenter support (3+ sites)
- Configurable replication zones
- Read-your-writes consistency
- Tunable consistency levels

---

## 9. Monitoring & Observability

### 9.1 Metrics (Prometheus Format)

**Storage Metrics**:
```
casgarage_storage_bytes_total
casgarage_storage_bytes_used
casgarage_storage_objects_total
casgarage_storage_buckets_total
casgarage_storage_blocks_total
casgarage_storage_replicas_healthy
casgarage_storage_replicas_degraded
```

**Request Metrics**:
```
casgarage_requests_total{method, status}
casgarage_request_duration_seconds{method, quantile}
casgarage_request_size_bytes{method}
casgarage_response_size_bytes{method}
casgarage_concurrent_requests
```

**Cluster Metrics**:
```
casgarage_cluster_nodes_total
casgarage_cluster_nodes_online
casgarage_cluster_replication_lag_seconds
casgarage_cluster_bandwidth_bytes{direction}
```

**Error Metrics**:
```
casgarage_errors_total{type}
casgarage_auth_failures_total
casgarage_disk_errors_total{node}
```

### 9.2 Health Checks

**Endpoint**: `GET /api/v1/health`
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 86400,
  "cluster": {
    "nodes": 3,
    "nodes_online": 3,
    "replication_factor": 3,
    "status": "healthy"
  },
  "storage": {
    "bytes_used": 1099511627776,
    "bytes_available": 5497558138880,
    "usage_percent": 20.0
  }
}
```

**Liveness**: `GET /healthz` → 200 OK (server running)  
**Readiness**: `GET /readyz` → 200 OK (accepting requests)

### 9.3 Logging

**Structured Logging** (tracing crate)
```rust
tracing::info!(
    user_id = %user.id,
    bucket = %bucket_name,
    operation = "create_bucket",
    "Bucket created successfully"
);
```

**Log Levels**:
- `ERROR`: Critical errors requiring attention
- `WARN`: Non-critical issues, degraded state
- `INFO`: Important operational events
- `DEBUG`: Detailed troubleshooting info
- `TRACE`: Very verbose, development only

**Log Destinations**:
- stdout/stderr (JSON format)
- File rotation (via systemd or logrotate)
- Syslog (RFC 5424)
- External logging (Loki, Elasticsearch)

---

## 10. Testing Strategy

### 10.1 Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_bucket() {
        let db = setup_test_db().await;
        let result = create_bucket(&db, "test-bucket").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_access_key_validation() {
        let key = generate_access_key();
        assert_eq!(key.len(), 20);
        assert!(key.chars().all(|c| c.is_alphanumeric()));
    }
}
```

### 10.2 Integration Tests
- S3 API compliance (aws-sdk-s3 test suite)
- Multi-node cluster setup/teardown
- Replication verification
- Backup/restore scenarios
- Authentication flows

### 10.3 End-to-End Tests
- Web UI automation (Selenium/Playwright)
- CLI command validation
- Load testing (k6, wrk)
- Chaos engineering (kill nodes, network partitions)

### 10.4 Performance Benchmarks
- Small object throughput (< 1MB)
- Large object throughput (> 100MB)
- Metadata operations (list, stat)
- Concurrent client scaling
- Memory usage profiling

---

## 11. Documentation Plan

### 11.1 User Documentation
- **Getting Started Guide**: Installation, first run, basic setup
- **Configuration Reference**: All settings explained
- **API Documentation**: OpenAPI/Swagger, examples
- **CLI Reference**: All commands, flags, examples
- **Deployment Guides**: Docker, Kubernetes, bare metal
- **Backup & Recovery**: Best practices, procedures
- **Troubleshooting**: Common issues, solutions
- **Security Best Practices**: Hardening checklist

### 11.2 Developer Documentation
- **Architecture Overview**: System design, components
- **Contributing Guide**: Code style, PR process
- **Building from Source**: Rust setup, dependencies
- **Testing Guide**: Running tests, writing new tests
- **Release Process**: Versioning, changelog, publishing
- **Garage Integration**: How we integrate with Garage upstream

### 11.3 Operations Documentation
- **Monitoring Setup**: Prometheus, Grafana dashboards
- **Scaling Guide**: When and how to add nodes
- **Upgrade Procedures**: Zero-downtime upgrades
- **Performance Tuning**: Optimization tips
- **Disaster Recovery**: Multi-site failover procedures
- **Capacity Planning**: Storage, bandwidth, IOPS

---

## 12. Roadmap & Future Enhancements

### Version 1.1 (Q2 2026)
- [ ] Erasure coding support (beyond replication)
- [ ] S3 Glacier-like archive tier (compression + deduplication)
- [ ] Advanced metrics dashboard (built-in, no Grafana needed)
- [ ] Mobile app (iOS/Android) for monitoring
- [ ] Automatic SSL certificate renewal (ACME)

### Version 1.2 (Q3 2026)
- [ ] Object immutability scanner (compliance validation)
- [ ] AI-powered capacity forecasting
- [ ] Multi-tenancy support (isolated buckets per tenant)
- [ ] Billing/usage tracking per tenant
- [ ] Advanced search (full-text object metadata search)

### Version 2.0 (Q4 2026)
- [ ] Federation support (connect multiple CasGarage clusters)
- [ ] Global namespace across federated clusters
- [ ] Enhanced geo-routing (edge caching)
- [ ] Object tagging with complex queries
- [ ] S3 Select support (query objects with SQL)
- [ ] Machine learning for predictive maintenance

### Long-Term Vision
- Replace all commercial S3 alternatives for self-hosters
- Become the standard for SMB object storage
- Enterprise adoption with support contracts
- Cloud marketplace offerings (AWS, Azure, GCP)

---

## 13. Success Metrics

**Technical Metrics**:
- Binary size < 50MB (all platforms)
- Memory usage < 256MB (idle)
- Startup time < 2 seconds
- API latency p99 < 100ms
- 99.9% uptime in production

**Adoption Metrics** (Year 1):
- 5000+ GitHub stars
- 1000+ Docker pulls per month
- 100+ production deployments
- 50+ contributors
- Featured on HN, Reddit /r/selfhosted

**Community Metrics**:
- Active Discord server (500+ members)
- Weekly releases (patches, features)
- < 48 hour issue response time
- Comprehensive documentation (90%+ coverage)
- Video tutorials and demos

---

## 14. License & Governance

**License**: MIT License
- Permissive, business-friendly
- No copyleft restrictions
- Compatible with commercial use
- Aligns with casapps philosophy

**Governance**:
- Jason (casjay) as project lead
- Community contributions welcome
- Code review required for merges
- Semantic versioning (semver)
- Monthly release cycle (minor versions)
- LTS releases every 6 months

**Code of Conduct**:
- Inclusive, welcoming community
- Zero tolerance for harassment
- Respectful technical discussions
- Mentor new contributors

---

## 15. Contact & Resources

**GitHub**: https://github.com/casapps/casgarage  
**Documentation**: https://docs.casapps.dev/casgarage  
**Docker Hub**: https://hub.docker.com/r/casapps/casgarage  
**Discord**: https://discord.gg/casapps  
**Email**: casjay@yahoo.com

**Related Projects**:
- Garage: https://garagehq.deuxfleurs.fr/
- MinIO: https://min.io/
- SeaweedFS: https://github.com/seaweedfs/seaweedfs

---

## Appendix A: Comparison Matrix

| Feature | CasGarage | MinIO | Garage | AWS S3 |
|---------|-----------|-------|--------|--------|
| Single Binary | ✅ | ✅ | ✅ | ❌ |
| Web UI Included | ✅ | ✅ | ❌ | ✅ |
| Embedded Config | ✅ | ❌ | ❌ | ❌ |
| Multi-Platform | ✅ | ✅ | ✅ | N/A |
| Geo-Distribution | ✅ | ✅* | ✅ | ✅ |
| Zero Dependencies | ✅ | ❌ | ✅ | N/A |
| Memory Usage | Low | Medium | Low | N/A |
| Self-Hosted Focus | ✅ | ❌ | ✅ | ❌ |
| License | MIT | AGPL | AGPL | Proprietary |

*MinIO requires etcd for distributed mode

---

## Appendix B: Example S3 Usage

**AWS CLI**:
```bash
# Configure
aws configure set aws_access_key_id YOUR_ACCESS_KEY
aws configure set aws_secret_access_key YOUR_SECRET_KEY
aws configure set default.s3.signature_version s3v4
aws configure set default.s3.endpoint_url http://localhost:3901

# Create bucket
aws s3 mb s3://my-bucket

# Upload file
aws s3 cp file.txt s3://my-bucket/

# List objects
aws s3 ls s3://my-bucket/

# Download file
aws s3 cp s3://my-bucket/file.txt downloaded.txt
```

**Python boto3**:
```python
import boto3

s3 = boto3.client(
    's3',
    endpoint_url='http://localhost:3901',
    aws_access_key_id='YOUR_ACCESS_KEY',
    aws_secret_access_key='YOUR_SECRET_KEY'
)

# Upload
s3.upload_file('local.txt', 'my-bucket', 'remote.txt')

# Download
s3.download_file('my-bucket', 'remote.txt', 'local-copy.txt')

# List
response = s3.list_objects_v2(Bucket='my-bucket')
for obj in response['Contents']:
    print(obj['Key'], obj['Size'])
```

---

**End of Specification**

*This specification is a living document and will evolve based on community feedback and implementation experience.*
