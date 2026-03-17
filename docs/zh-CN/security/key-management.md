# 密钥管理

本文档描述如何在 Nexis 中管理机密和加密密钥。

## 支持的密钥管理系统

| 系统 | 类型 | 使用场景 |
|------|------|----------|
| HashiCorp Vault | 自托管/云 | 企业级，完全控制 |
| AWS KMS | 云服务 | AWS 原生部署 |
| Azure Key Vault | 云服务 | Azure 原生部署 |
| Google Cloud KMS | 云服务 | GCP 原生部署 |

## 密钥类型

| 密钥 | 用途 | 轮换策略 |
|------|------|----------|
| `JWT_SECRET` | JWT 令牌签名 | 90 天 |
| `DATABASE_URL` | 数据库连接 | 变更时 |
| `REDIS_URL` | Redis 连接 | 变更时 |
| `AI_API_KEY` | AI 提供商密钥 | 90 天 |

## 环境变量

```bash
# 必需
export JWT_SECRET="your-secret-key-min-32-chars"
export DATABASE_URL="postgresql://user:pass@host:5432/nexis"
export REDIS_URL="redis://host:6379"

# 可选（AI 功能）
export AI_API_KEY="sk-..."
export AI_PROVIDER="openai"  # openai | anthropic | google

# 密钥管理
export NEXIS_VAULT_ENABLED="true"
export NEXIS_VAULT_ADDR="https://vault.example.com"
export NEXIS_VAULT_ROLE="nexis-gateway"
export NEXIS_VAULT_SECRET_PATH="secret/data/nexis"
```

## HashiCorp Vault 集成

### 1. 安装 Vault CLI

```bash
# macOS
brew install vault

# Linux
wget https://releases.hashicorp.com/vault/1.15.0/vault_1.15.0_linux_amd64.zip
unzip vault_1.15.0_linux_amd64.zip
sudo mv vault /usr/local/bin/
```

### 2. 配置 Vault

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

### 3. 启用 Kubernetes 认证

```bash
# 启用 Kubernetes 认证方法
vault auth enable kubernetes

# 配置 Kubernetes 认证
vault write auth/kubernetes/config \
  kubernetes_host="https://kubernetes.default.svc:443" \
  kubernetes_ca_cert=@/var/run/secrets/kubernetes.io/serviceaccount/ca.crt \
  token_reviewer_jwt=@/var/run/secrets/kubernetes.io/serviceaccount/token
```

### 4. 创建策略

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

### 5. 创建角色

```bash
vault write auth/kubernetes/role/nexis-gateway \
  bound_service_account_names=nexis-gateway \
  bound_service_account_namespaces=production \
  policies=nexis-policy \
  ttl=1h
```

## 密钥轮换

### 自动轮换（推荐）

```yaml
# Kubernetes CronJob 密钥轮换
apiVersion: batch/v1
kind: CronJob
metadata:
  name: nexis-key-rotation
spec:
  schedule: "0 0 1 */3 *"  # 每 90 天
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
              # 生成新密钥
              NEW_KEY=$(openssl rand -base64 32)
              
              # 存储到 Vault
              vault kv put secret/nexis jwt_secret_new=$NEW_KEY
              
              # 更新应用
              kubectl rollout restart deployment/nexis-gateway
```

### 手动轮换

```bash
# 1. 生成新密钥
NEW_JWT_SECRET=$(openssl rand -base64 32)

# 2. 存储到 Vault
vault kv put secret/nexis jwt_secret=$NEW_JWT_SECRET

# 3. 更新应用
kubectl rollout restart deployment/nexis-gateway -n production

# 4. 验证
kubectl logs -f deployment/nexis-gateway -n production | grep "JWT"
```

## 密钥泄露应急响应

### 第一阶段：立即行动（0-15 分钟）

1. **撤销泄露的密钥**
   ```bash
   vault kv delete secret/nexis
   ```

2. **生成新密钥**
   ```bash
   NEW_KEY=$(openssl rand -base64 32)
   vault kv put secret/nexis jwt_secret=$NEW_KEY
   ```

3. **重启所有服务**
   ```bash
   kubectl rollout restart deployment/nexis-gateway -n production
   ```

### 第二阶段：短期响应（15-60 分钟）

1. **审计访问日志**
   ```bash
   vault audit enable file file_path=/var/log/vault/audit.log
   grep "secret/nexis" /var/log/vault/audit.log
   ```

2. **使所有会话失效**
   - 强制所有用户重新认证
   - 通知受影响用户

3. **审查 IAM 策略**
   - 检查谁有访问权限
   - 移除不必要的权限

### 第三阶段：调查（1-24 小时）

1. **根本原因分析**
   - 密钥是如何暴露的？
   - 访问了哪些系统？

2. **取证日志**
   - 保存所有相关日志
   - 记录时间线

### 第四阶段：恢复（24-72 小时）

1. **更新所有密钥**
   - 轮换所有密钥，不只是泄露的那个
   - 更新所有配置

2. **安全审查**
   - 审查访问策略
   - 实施额外控制

3. **事后报告**
   - 记录经验教训
   - 更新安全程序

## 配置参考

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

## 最佳实践

1. **永远不要将密钥提交到 git**
   - 使用 `.env` 文件（添加到 `.gitignore`）
   - 使用密钥管理系统

2. **使用强密钥**
   - 最少 32 个字符
   - 使用加密安全的随机生成

3. **定期轮换**
   - JWT 密钥：90 天
   - 数据库凭证：人员变更时
   - API 密钥：90 天

4. **限制访问**
   - 最小权限原则
   - 定期审计访问

5. **监控使用**
   - 启用审计日志
   - 对可疑模式告警

## 故障排除

### Vault 连接问题

```bash
# 检查 Vault 状态
vault status

# 测试连接
curl -k https://vault.example.com:8200/v1/sys/health

# 检查 Kubernetes 认证
vault read auth/kubernetes/config
```

### 密钥验证错误

```bash
# 验证 JWT 密钥格式
echo -n "your-secret" | wc -c  # 应该 >= 32

# 测试 JWT 签名
node -e "console.log(require('jsonwebtoken').sign({test:1}, 'your-secret'))"
```

### 轮换失败

```bash
# 检查 CronJob 日志
kubectl logs job/nexis-key-rotation-<timestamp>

# 验证 Vault 权限
vault token capabilities secret/data/nexis
```
