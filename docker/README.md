# Docker Configuration for CasGarage

This directory contains Docker-related configurations for running CasGarage in various environments.

## Files

- `docker-compose.prod.yml` - Production 3-node cluster configuration
- Main `Dockerfile` (in project root) - Production multi-stage build
- `Dockerfile.dev` (in project root) - Development with hot reload

## Quick Start

### Development

```bash
# Start development environment
docker-compose up

# The following ports will be available:
# - 3900: Admin UI
# - 3901: S3 API
# - 8080: Frontend dev server (hot reload)
# - 9090: Metrics
```

### Production (Single Node)

```bash
# Build image
docker build -t casapps/casgarage:latest .

# Run container
docker run -d \
  -p 3900:3900 \
  -p 3901:3901 \
  -p 9090:9090 \
  -v casgarage-data:/data/casgarage \
  -v casgarage-tmp:/tmp/casgarage \
  --name casgarage \
  casapps/casgarage:latest
```

### Production (3-Node Cluster)

```bash
# Start cluster
docker-compose -f docker/docker-compose.prod.yml up -d

# Check cluster status
docker exec casgarage-node1 casgarage cluster status
```

## Volumes

All Docker configurations use project-scoped temporary directories:
- `/tmp/casgarage` - Temporary files (mounted volume)
- `/data/casgarage/db` - Database files
- `/data/casgarage/blocks` - Block storage

## Environment Variables

See main `CLAUDE.md` for all available environment variables.

## Health Checks

All containers include health checks that verify the server is responding:
- Interval: 30s
- Timeout: 10s
- Retries: 3
