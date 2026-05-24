# Quick Start Guide

Get CasGarage running in minutes.

## Using Docker (Recommended)

```bash
# Run single node
docker run -d \
  -p 3900:3900 \
  -p 3901:3901 \
  -v casgarage-data:/data/casgarage \
  -v casgarage-tmp:/tmp/casgarage \
  --name casgarage \
  casapps/casgarage:latest

# Access admin UI
open http://localhost:3900

# S3 API endpoint
# http://localhost:3901
```

## Using Docker Compose

```bash
# Clone repository
git clone https://github.com/casapps/casgarage.git
cd casgarage

# Start development server
docker-compose up -d

# Check logs
docker-compose logs -f
```

## Using Binary

```bash
# Download latest release
wget https://github.com/casapps/casgarage/releases/latest/download/casgarage-linux-amd64
chmod +x casgarage-linux-amd64
mv casgarage-linux-amd64 /usr/local/bin/casgarage

# Start server
casgarage server

# Access at http://localhost:3900
```

## First Steps

1. Access admin UI at `http://localhost:3900`
2. Create your first bucket
3. Generate an access key
4. Configure your S3 client

## Testing S3 API

```bash
# Using AWS CLI
aws configure set aws_access_key_id YOUR_KEY
aws configure set aws_secret_access_key YOUR_SECRET
aws configure set default.s3.endpoint_url http://localhost:3901

# Create bucket
aws s3 mb s3://test-bucket

# Upload file
echo "Hello CasGarage" > test.txt
aws s3 cp test.txt s3://test-bucket/

# List objects
aws s3 ls s3://test-bucket/
```

## Next Steps

- [Full Installation Guide](installation.md)
- [Configuration Options](configuration.md)
- [Cluster Setup](cluster.md)
