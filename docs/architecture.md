# CasGarage Architecture

## Overview

CasGarage is built as a single static binary written entirely in Rust, integrating the Garage storage engine with a custom administrative web interface.

## Components

### 1. Core Application (`src/`)

#### Garage Storage Engine (`src/garage/`)
- **API Layer**: S3 protocol implementation
- **Data Layer**: Block storage and data management
- **Model Layer**: Data structures and types
- **RPC Layer**: Inter-node communication

#### Web Server (`src/web/`)
- **Server**: Axum-based HTTP server
- **Routes**: API endpoint routing
- **Handlers**: Request processing logic
- **Middleware**: Authentication, CORS, logging

#### Admin System (`src/admin/`)
- **Auth**: JWT authentication and password hashing
- **Users**: User management
- **Metrics**: Prometheus metrics collection
- **Backup**: Backup and restore operations

#### Database (`src/db/`)
- **Schema**: SQLite table definitions
- **Migrations**: Database schema versioning
- **Queries**: Data access layer

#### CLI (`src/cli/`)
- **Commands**: All CLI subcommands
- **Parser**: Argument parsing utilities

### 2. Frontend (`frontend/`)

Built with Leptos (Rust WASM framework):
- **Components**: Reusable UI components
- **Pages**: Application views
- **API Client**: Backend communication
- **Styles**: Dark theme CSS

## Data Flow

```
User Request → Axum Router → Middleware → Handler → Database/Storage → Response
                                                   ↓
                                              Garage Engine
```

## Storage Architecture

- **SQLite**: Configuration and metadata
- **Block Storage**: Object data in local filesystem
- **Replication**: Multi-node data distribution

## Deployment Patterns

1. **Single Node**: Standalone server
2. **Multi-Node Cluster**: 3+ nodes with replication
3. **Geo-Distributed**: Multiple datacenters

## Technology Stack

- **Language**: Rust 2021 edition
- **Web Framework**: Axum
- **Frontend**: Leptos (WASM)
- **Database**: SQLite (via tokio-rusqlite)
- **CLI**: Clap
- **Async Runtime**: Tokio
- **Storage Engine**: Garage (integrated)

## Performance Considerations

- Zero-copy I/O where possible
- Async/await throughout
- Connection pooling
- Efficient serialization (bincode)
- Memory-mapped files for large objects
