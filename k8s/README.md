# Kubernetes Deployment for CasGarage

This directory contains Kubernetes manifests for deploying CasGarage in a production Kubernetes cluster.

## Files

- `namespace.yaml` - Creates the `casgarage` namespace
- `statefulset.yaml` - StatefulSet with 3 replicas and persistent storage
- `service.yaml` - Services for admin UI, S3 API, and metrics
- `ingress.yaml` - Ingress configuration with TLS
- `configmap.yaml` - Configuration settings

## Prerequisites

- Kubernetes cluster (1.20+)
- kubectl configured
- Storage class for persistent volumes
- Ingress controller (e.g., nginx-ingress)
- cert-manager for TLS certificates (optional)

## Deployment

### 1. Create Namespace

```bash
kubectl apply -f namespace.yaml
```

### 2. Create ConfigMap

```bash
kubectl apply -f configmap.yaml
```

### 3. Deploy StatefulSet

```bash
kubectl apply -f statefulset.yaml
```

### 4. Create Services

```bash
kubectl apply -f service.yaml
```

### 5. Configure Ingress

Edit `ingress.yaml` to set your domain names:
- Replace `casgarage.example.com` with your admin UI domain
- Replace `s3.casgarage.example.com` with your S3 API domain

```bash
kubectl apply -f ingress.yaml
```

### Or Apply All at Once

```bash
kubectl apply -f k8s/
```

## Accessing the Application

Once deployed, you can access:

- **Admin UI**: https://casgarage.example.com
- **S3 API**: https://s3.casgarage.example.com
- **Metrics**: Via Prometheus ServiceMonitor

## Monitoring

Check deployment status:

```bash
# Check pods
kubectl get pods -n casgarage

# Check services
kubectl get svc -n casgarage

# Check persistent volumes
kubectl get pvc -n casgarage

# View logs
kubectl logs -n casgarage -l app=casgarage -f

# Check cluster status
kubectl exec -n casgarage casgarage-0 -- casgarage cluster status
```

## Scaling

Scale the StatefulSet:

```bash
kubectl scale statefulset casgarage -n casgarage --replicas=5
```

## Storage

Each pod gets two persistent volumes:
- `/data/casgarage` - 100Gi for block storage and database
- `/tmp/casgarage` - 10Gi for temporary files

Adjust sizes in `statefulset.yaml` as needed.

## Security

- Pods run as non-root user (UID 1000)
- Network policies can be added for additional isolation
- TLS is handled by ingress with cert-manager

## Backup

Backup persistent volumes regularly:

```bash
# Example: Create volume snapshot
kubectl create -f - <<EOF
apiVersion: snapshot.storage.k8s.io/v1
kind: VolumeSnapshot
metadata:
  name: casgarage-backup-$(date +%Y%m%d)
  namespace: casgarage
spec:
  volumeSnapshotClassName: csi-snapclass
  source:
    persistentVolumeClaimName: data-casgarage-0
EOF
```

## Troubleshooting

```bash
# Describe pod
kubectl describe pod casgarage-0 -n casgarage

# Get events
kubectl get events -n casgarage --sort-by='.lastTimestamp'

# Shell into pod
kubectl exec -it casgarage-0 -n casgarage -- /bin/sh

# Check health
kubectl exec -it casgarage-0 -n casgarage -- casgarage health
```
