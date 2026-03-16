# Deployment Guide

This guide covers the deployment process, environment configuration, and health checks for Nexis.

## Deployment Process

### Prerequisites

Before deploying Nexis, ensure you have the following:

- Docker 20.10+ or Kubernetes 1.20+
- Minimum 4GB RAM, 2 CPU cores
- Persistent storage (minimum 10GB for production)
- Network access to required dependencies

### Deployment Methods

#### Docker Compose (Development/Staging)

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/nexis.git
   cd nexis
   ```

2. Configure environment:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. Start services:
   ```bash
   docker-compose up -d
   ```

4. Verify deployment:
   ```bash
   docker-compose ps
   docker-compose logs -f
   ```

#### Kubernetes (Production)

1. Apply namespace and secrets:
   ```bash
   kubectl apply -f k8s/namespace.yaml
   kubectl create secret generic nexis-secrets --from-env-file=.env
   ```

2. Deploy using Helm or kubectl:
   ```bash
   # Using Helm
   helm install nexis ./deploy/helm -n nexis
   
   # Using kubectl
   kubectl apply -f k8s/ -n nexis
   ```

3. Verify deployment:
   ```bash
   kubectl get pods -n nexis
   kubectl logs -f deployment/nexis -n nexis
   ```

### Rolling Updates

For zero-downtime deployments:

1. Update the container image version in your deployment configuration
2. Apply the update:
   ```bash
   kubectl set image deployment/nexis nexis=your-registry/nexis:v2.0.0 -n nexis
   ```
3. Monitor the rollout:
   ```bash
   kubectl rollout status deployment/nexis -n nexis
   ```

4. Rollback if needed:
   ```bash
   kubectl rollout undo deployment/nexis -n nexis
   ```

## Environment Configuration

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `NODE_ENV` | Environment mode | `production` |
| `DATABASE_URL` | Database connection string | `postgresql://user:pass@host:5432/nexis` |
| `REDIS_URL` | Redis connection string | `redis://host:6379` |
| `LOG_LEVEL` | Logging level | `info`, `warn`, `error` |
| `PORT` | Application port | `8080` |

### Optional Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `METRICS_PORT` | Prometheus metrics port | `9090` |
| `TRACING_ENABLED` | Enable distributed tracing | `false` |
| `CACHE_TTL` | Cache TTL in seconds | `3600` |
| `MAX_CONNECTIONS` | Max database connections | `100` |

### Configuration Best Practices

1. **Security**: Never commit `.env` files to version control
2. **Secrets Management**: Use Kubernetes Secrets or external secret managers (HashiCorp Vault, AWS Secrets Manager)
3. **Environment Separation**: Maintain separate configurations for dev/staging/prod
4. **Validation**: Validate all configurations at startup

### Example Configuration Files

#### Production Configuration

```yaml
# config/production.yaml
server:
  port: 8080
  workers: 4

database:
  pool_size: 100
  timeout: 30s

cache:
  enabled: true
  ttl: 3600
  backend: redis

logging:
  level: info
  format: json
```

## Health Checks

### Health Check Endpoints

Nexis provides several health check endpoints:

#### `/health` - Basic Health Check

Returns 200 OK if the service is running:
```bash
curl http://localhost:8080/health
```

Response:
```json
{
  "status": "ok",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### `/health/ready` - Readiness Check

Returns 200 OK if the service is ready to accept traffic:
```bash
curl http://localhost:8080/health/ready
```

Response:
```json
{
  "status": "ready",
  "checks": {
    "database": "ok",
    "cache": "ok",
    "dependencies": "ok"
  }
}
```

#### `/health/live` - Liveness Check

Returns 200 OK if the service is alive (not deadlocked):
```bash
curl http://localhost:8080/health/live
```

### Kubernetes Health Probes

Configure health probes in your Kubernetes deployment:

```yaml
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3

readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 5
  timeoutSeconds: 3
  failureThreshold: 3
```

### Docker Health Check

For Docker deployments, add a health check:

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
```

### Monitoring Health Status

1. **Prometheus**: Health metrics are exposed at `/metrics`
2. **Logs**: Health check failures are logged with ERROR level
3. **Alerts**: Configure alerts for consecutive health check failures

### Troubleshooting Health Checks

- **Database Connection Failures**: Check DATABASE_URL and network connectivity
- **Cache Failures**: Verify Redis is running and accessible
- **Timeout Issues**: Increase timeout values or optimize database queries
- **Memory Issues**: Check memory limits and increase if necessary

## Deployment Checklist

Before deploying to production:

- [ ] All environment variables configured
- [ ] Database migrations applied
- [ ] Secrets properly stored
- [ ] Health checks configured
- [ ] Monitoring and logging set up
- [ ] Backup strategy in place
- [ ] Rollback plan documented
- [ ] Load testing completed
- [ ] Security scan passed
- [ ] Documentation updated

## Post-Deployment Verification

1. **Check Service Health**:
   ```bash
   curl http://your-service/health/ready
   ```

2. **Verify Metrics**:
   ```bash
   curl http://your-service:9090/metrics
   ```

3. **Check Logs**:
   ```bash
   kubectl logs -f deployment/nexis -n nexis
   ```

4. **Run Smoke Tests**:
   ```bash
   ./scripts/smoke-test.sh
   ```

5. **Monitor Performance**: Check Grafana dashboards for anomalies

## Next Steps

- [Monitoring Guide](./monitoring.md) - Set up monitoring and alerting
- [Troubleshooting Guide](./troubleshooting.md) - Debug common issues
