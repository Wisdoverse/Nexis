# Nexis Gateway Helm Chart

Enterprise-grade Helm chart for deploying Nexis Gateway on Kubernetes.

## Features

- **Production-ready**: Security best practices (non-root, read-only filesystem, dropped capabilities)
- **Auto-scaling**: HPA support (3-100 replicas)
- **Network Policy**: Pod-to-pod communication restriction
- **TLS Support**: Ingress with cert-manager integration
- **Health Checks**: Liveness and readiness probes

## Quick Start

```bash
# Add namespace
kubectl create namespace production

# Install chart
helm install nexis-gateway ./deploy/helm/nexis-gateway \
  -n production \
  --set secrets.jwtSecret=your-secret-key \
  --set secrets.databaseUrl=postgresql://...
```

## Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of replicas | `3` |
| `image.repository` | Image repository | `nexis/gateway` |
| `image.tag` | Image tag | `0.1.0` |
| `service.type` | Service type | `ClusterIP` |
| `service.port` | Service port | `8080` |
| `autoscaling.enabled` | Enable HPA | `true` |
| `autoscaling.minReplicas` | Minimum replicas | `3` |
| `autoscaling.maxReplicas` | Maximum replicas | `100` |
| `networkPolicy.enabled` | Enable NetworkPolicy | `true` |
| `ingress.enabled` | Enable Ingress | `true` |
| `ingress.className` | Ingress class | `nginx` |

## Security Configuration

The chart applies these security best practices by default:

```yaml
securityContext:
  runAsNonRoot: true
  runAsUser: 1000
  readOnlyRootFilesystem: true
  allowPrivilegeEscalation: false
  capabilities:
    drop:
      - ALL
```

## TLS Configuration

```bash
helm install nexis-gateway ./deploy/helm/nexis-gateway \
  -n production \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=api.nexis.io
```

## Resources

Default resource configuration:

```yaml
resources:
  limits:
    cpu: 2000m
    memory: 2Gi
  requests:
    cpu: 500m
    memory: 512Mi
```

## Upgrading

```bash
helm upgrade nexis-gateway ./deploy/helm/nexis-gateway -n production
```

## Uninstalling

```bash
helm uninstall nexis-gateway -n production
```
