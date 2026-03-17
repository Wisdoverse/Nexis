# Key Management

This document describes how to manage secrets and cryptographic keys in Nexis.

## Supported Key Management Systems

| System | Type | Use Case |
|--------|------|----------|
| HashiCorp Vault | Self-hosted/Cloud | Enterprise, full control |
| AWS KMS | Cloud | AWS-native deployments |
| Azure Key Vault | Cloud | Azure-native deployments |
| Google Cloud KMS | Cloud | GCP-native deployments |

## Secret Types

| Secret | Purpose | Rotation Policy |
|--------|---------|-----------------|
| `JWT_SECRET` | JWT token signing | 90 days |
| `DATABASE_URL` | Database connection | On change |
| `REDIS_URL` | Redis connection | On change |
| `AI_API_KEY` | AI provider API key | 90 days |

## Environment Variables

```bash
# Required
export JWT_SECRET="your-secret-key-min-32-chars"
export DATABASE_URL="postgresql://user:pass@host:5432/nexis"
export REDIS_URL="redis://host:6379"

# Optional (for AI features)
export AI_API_KEY="sk-..."
export AI_PROVIDER="openai"  # openai | anthropic | google

# Key Management
export NEXIS_VAULT_ENABLED="true"
export NEXIS_VAULT_ADDR="https://vault.example.com"
export NEXIS_VAULT_ROLE="nexis-gateway"
export NEXIS_VAULT_SECRET_PATH="secret/data/nexis"
```

## HashiCorp Vault Integration

### 1. Install Vault CLI

```bash
# macOS
brew install vault

# Linux
wget https://releases.hashicorp.com/vault/1.15.0/vault_1.15.0_linux_amd64.zip
unzip vault_1.15.0_linux_amd64.zip
sudo mv vault /usr/local/bin/
```

### 2. Configure Vault

```hcl
# vault.hcl
storage "file" {
  path = "/opt/vault/data"
}

listener "tcp" {
  address     = "0.0.0.0:8200"
  tls_disable = false
  tls_cert_file = "/opt/vault/tls/cert.pem"
  tls_key_file  = "/opt/vault/tls/key.pem"
}

api_addr = "https://vault.example.com:8200"
ui = true
```

### 3. Enable Kubernetes Auth

```bash
# Enable Kubernetes auth method
vault auth enable kubernetes

# Configure Kubernetes auth
vault write auth/kubernetes/config \
  kubernetes_host="https://kubernetes.default.svc:443" \
  kubernetes_ca_cert=@/var/run/secrets/kubernetes.io/serviceaccount/ca.crt \
  token_reviewer_jwt=@/var/run/secrets/kubernetes.io/serviceaccount/token
```

### 4. Create Policy

```hcl
# nexis-policy.hcl
path "secret/data/nexis" {
  capabilities = ["read"]
}

path "secret/data/nexis/*" {
  capabilities = ["read"]
}
```

```bash
vault policy write nexis-policy nexis-policy.hcl
```

### 5. Create Role

```bash
vault write auth/kubernetes/role/nexis-gateway \
  bound_service_account_names=nexis-gateway \
  bound_service_account_namespaces=production \
  policies=nexis-policy \
  ttl=1h
```

## Key Rotation

### Automatic Rotation (Recommended)

```yaml
# Kubernetes CronJob for key rotation
apiVersion: batch/v1
kind: CronJob
metadata:
  name: nexis-key-rotation
spec:
  schedule: "0 0 1 */3 *"  # Every 90 days
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: rotate
            image: nexis/key-rotation:latest
            command:
            - /bin/sh
            - -c
            - |
              # Generate new key
              NEW_KEY=$(openssl rand -base64 32)
              
              # Store in Vault
              vault kv put secret/nexis jwt_secret_new=$NEW_KEY
              
              # Update application
              kubectl rollout restart deployment/nexis-gateway
```

### Manual Rotation

```bash
# 1. Generate new key
NEW_JWT_SECRET=$(openssl rand -base64 32)

# 2. Store in Vault
vault kv put secret/nexis jwt_secret=$NEW_JWT_SECRET

# 3. Update application
kubectl rollout restart deployment/nexis-gateway -n production

# 4. Verify
kubectl logs -f deployment/nexis-gateway -n production | grep "JWT"
```

## Key Leak Emergency Response

### Phase 1: Immediate Action (0-15 minutes)

1. **Revoke compromised key**
   ```bash
   vault kv delete secret/nexis
   ```

2. **Generate new key**
   ```bash
   NEW_KEY=$(openssl rand -base64 32)
   vault kv put secret/nexis jwt_secret=$NEW_KEY
   ```

3. **Restart all services**
   ```bash
   kubectl rollout restart deployment/nexis-gateway -n production
   ```

### Phase 2: Short-term Response (15-60 minutes)

1. **Audit access logs**
   ```bash
   vault audit enable file file_path=/var/log/vault/audit.log
   grep "secret/nexis" /var/log/vault/audit.log
   ```

2. **Invalidate all sessions**
   - Force re-authentication for all users
   - Notify affected users

3. **Review IAM policies**
   - Check who had access
   - Remove unnecessary permissions

### Phase 3: Investigation (1-24 hours)

1. **Root cause analysis**
   - How was the key exposed?
   - What systems were accessed?

2. **Forensic logging**
   - Preserve all relevant logs
   - Document timeline

### Phase 4: Recovery (24-72 hours)

1. **Update all secrets**
   - Rotate ALL keys, not just compromised one
   - Update all configurations

2. **Security review**
   - Review access policies
   - Implement additional controls

3. **Post-incident report**
   - Document lessons learned
   - Update security procedures

## Configuration Reference

### Docker Compose

```yaml
services:
  nexis-gateway:
    image: nexis/gateway:latest
    environment:
      - JWT_SECRET=${JWT_SECRET}
      - DATABASE_URL=${DATABASE_URL}
      - REDIS_URL=${REDIS_URL}
    env_file:
      - .env.production
```

### Kubernetes

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: nexis-secrets
type: Opaque
stringData:
  jwt-secret: "your-secret-here"
  database-url: "postgresql://..."
  redis-url: "redis://..."
---
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: nexis-gateway
        envFrom:
        - secretRef:
            name: nexis-secrets
```

## Best Practices

1. **Never commit secrets to git**
   - Use `.env` files (add to `.gitignore`)
   - Use secret management systems

2. **Use strong keys**
   - Minimum 32 characters
   - Use cryptographically secure random generation

3. **Rotate regularly**
   - JWT secrets: 90 days
   - Database credentials: On personnel changes
   - API keys: 90 days

4. **Limit access**
   - Principle of least privilege
   - Audit access regularly

5. **Monitor usage**
   - Enable audit logging
   - Alert on suspicious patterns

## Troubleshooting

### Vault Connection Issues

```bash
# Check Vault status
vault status

# Test connectivity
curl -k https://vault.example.com:8200/v1/sys/health

# Check Kubernetes auth
vault read auth/kubernetes/config
```

### Key Validation Errors

```bash
# Verify JWT secret format
echo -n "your-secret" | wc -c  # Should be >= 32

# Test JWT signing
node -e "console.log(require('jsonwebtoken').sign({test:1}, 'your-secret'))"
```

### Rotation Failures

```bash
# Check CronJob logs
kubectl logs job/nexis-key-rotation-<timestamp>

# Verify Vault permissions
vault token capabilities secret/data/nexis
```
