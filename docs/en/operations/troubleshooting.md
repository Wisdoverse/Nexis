# Troubleshooting Guide

This guide covers common issues, log analysis techniques, and performance tuning for Nexis.

## Common Issues

### Service Won't Start

#### Symptoms
- Service exits immediately after starting
- Error logs during startup

#### Possible Causes & Solutions

1. **Missing Environment Variables**
   ```
   Error: Required environment variable DATABASE_URL is not set
   ```
   
   **Solution**: Ensure all required environment variables are set:
   ```bash
   # Check .env file
   cat .env
   
   # Verify environment in container
   docker exec <container> env
   ```

2. **Database Connection Failure**
   ```
   Error: Failed to connect to database: connection refused
   ```
   
   **Solution**:
   - Verify database is running: `docker ps | grep postgres`
   - Check connection string: `echo $DATABASE_URL`
   - Test connectivity: `nc -zv <db-host> 5432`
   - Check firewall rules

3. **Port Already in Use**
   ```
   Error: Address already in use (os error 98)
   ```
   
   **Solution**:
   ```bash
   # Find process using port
   lsof -i :8080
   netstat -tlnp | grep 8080
   
   # Kill process or change port
   kill -9 <PID>
   # Or update PORT in .env
   ```

4. **Permission Denied**
   ```
   Error: Permission denied (os error 13)
   ```
   
   **Solution**:
   ```bash
   # Check file permissions
   ls -la /path/to/file
   
   # Fix permissions
   chmod 644 /path/to/file
   chown -R user:group /data
   ```

### High Latency

#### Symptoms
- Slow response times
- Timeouts in client applications

#### Diagnosis

1. **Check Response Times**:
   ```bash
   curl -w "@curl-format.txt" -o /dev/null -s http://localhost:8080/api/health
   ```
   
   Create `curl-format.txt`:
   ```
   time_namelookup:  %{time_namelookup}\n
   time_connect:  %{time_connect}\n
   time_appconnect:  %{time_appconnect}\n
   time_pretransfer:  %{time_pretransfer}\n
   time_redirect:  %{time_redirect}\n
   time_starttransfer:  %{time_starttransfer}\n
   time_total:  %{time_total}\n
   ```

2. **Analyze Metrics**:
   ```bash
   # Check P95 latency
   curl 'http://localhost:9090/api/v1/query?query=histogram_quantile(0.95,rate(nexis_request_duration_seconds_bucket[5m]))'
   ```

3. **Profile Application**:
   ```bash
   # CPU profiling
   curl http://localhost:8080/debug/pprof/profile?seconds=30 > cpu.prof
   
   # Memory profiling
   curl http://localhost:8080/debug/pprof/heap > heap.prof
   ```

#### Solutions

1. **Database Optimization**
   - Add missing indexes
   - Optimize slow queries
   - Increase connection pool size
   - Enable query caching

2. **Caching**
   - Enable Redis caching
   - Increase cache TTL
   - Implement request coalescing

3. **Horizontal Scaling**
   - Add more instances
   - Use load balancing
   - Enable auto-scaling

### Memory Issues

#### Symptoms
- Out of memory (OOM) errors
- Container killed unexpectedly
- Increasing memory usage over time

#### Diagnosis

1. **Check Memory Usage**:
   ```bash
   # Container memory
   docker stats <container>
   
   # Process memory
   ps aux | grep nexis
   top -p <PID>
   
   # Prometheus metrics
   curl 'http://localhost:9090/api/v1/query?query=process_resident_memory_bytes{job="nexis"}'
   ```

2. **Memory Profiling**:
   ```bash
   # Capture heap profile
   curl http://localhost:8080/debug/pprof/heap > heap.prof
   
   # Analyze with pprof
   go tool pprof heap.prof
   ```

3. **Detect Leaks**:
   - Monitor memory over time
   - Compare heap profiles before/after load test
   - Look for growing goroutines

#### Solutions

1. **Fix Memory Leaks**
   - Review code for unclosed connections
   - Check for unbounded data structures
   - Implement proper cleanup in loops

2. **Optimize Memory Usage**
   - Reduce buffer sizes
   - Implement streaming for large datasets
   - Use object pooling

3. **Increase Limits**
   ```yaml
   # Kubernetes
   resources:
     limits:
       memory: 4Gi
     requests:
       memory: 2Gi
   ```

### Database Issues

#### Connection Pool Exhausted

**Symptoms**:
- "Connection pool exhausted" errors
- Queries timing out

**Diagnosis**:
```bash
# Check pool metrics
curl 'http://localhost:9090/api/v1/query?query=nexis_db_connections'

# Check active queries
psql -c "SELECT * FROM pg_stat_activity;"
```

**Solutions**:
1. Increase pool size in configuration
2. Reduce query duration
3. Implement connection timeout
4. Add read replicas for load distribution

#### Slow Queries

**Diagnosis**:
```bash
# Enable slow query log
# postgresql.conf
log_min_duration_statement = 1000

# Check slow queries
psql -c "SELECT query, calls, total_time, mean_time FROM pg_stat_statements ORDER BY total_time DESC LIMIT 10;"
```

**Solutions**:
1. Add appropriate indexes
2. Rewrite inefficient queries
3. Use EXPLAIN ANALYZE
4. Implement query result caching

## Log Analysis

### Log Locations

- **Container Logs**: `docker logs <container>`
- **Kubernetes Logs**: `kubectl logs -f deployment/nexis -n nexis`
- **System Logs**: `/var/log/nexis/`
- **JSON Logs**: `/var/log/nexis/nexis.json`

### Log Levels

| Level | Description | When to Use |
|-------|-------------|-------------|
| ERROR | Errors requiring attention | Failures, exceptions |
| WARN | Warning conditions | Degraded performance |
| INFO | General operational info | Requests, state changes |
| DEBUG | Detailed debug info | Development, debugging |
| TRACE | Very detailed traces | Deep debugging only |

### Log Format

Nexis uses structured JSON logging:

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "message": "Request processed",
  "request_id": "abc-123",
  "method": "GET",
  "path": "/api/users",
  "status": 200,
  "duration_ms": 45,
  "user_id": "user-456"
}
```

### Useful Log Queries

#### Find Errors

```bash
# Docker logs
docker logs <container> 2>&1 | grep ERROR

# Kubernetes logs
kubectl logs -f deployment/nexis -n nexis | grep -E '"level":"ERROR"'

# JSON logs with jq
cat /var/log/nexis/nexis.json | jq 'select(.level == "ERROR")'
```

#### Search by Request ID

```bash
# Trace request flow
docker logs <container> 2>&1 | grep "request_id: abc-123"

# With jq
cat /var/log/nexis/nexis.json | jq 'select(.request_id == "abc-123")'
```

#### Slow Requests

```bash
# Find requests > 1 second
cat /var/log/nexis/nexis.json | jq 'select(.duration_ms > 1000)'

# Top 10 slowest requests
cat /var/log/nexis/nexis.json | jq -s 'sort_by(.duration_ms) | reverse | .[0:10]'
```

#### Database Errors

```bash
# Find database connection errors
kubectl logs -f deployment/nexis -n nexis | grep -i "database.*error"

# Connection timeouts
cat /var/log/nexis/nexis.json | jq 'select(.message | contains("connection")) | select(.level == "ERROR")'
```

### Log Aggregation

For production, aggregate logs using:

1. **ELK Stack** (Elasticsearch, Logstash, Kibana)
2. **Loki** (Grafana Loki)
3. **Datadog**
4. **Splunk**

#### Example: Loki Configuration

```yaml
# promtail-config.yml
server:
  http_listen_port: 9080

positions:
  filename: /tmp/positions.yaml

clients:
  - url: http://loki:3100/loki/api/v1/push

scrape_configs:
  - job_name: nexis
    static_configs:
      - targets:
          - localhost
        labels:
          job: nexis
          __path__: /var/log/nexis/*.json
```

### Log Analysis Tools

1. **jq**: JSON processor
   ```bash
   cat logs.json | jq 'select(.level == "ERROR") | {time: .timestamp, msg: .message}'
   ```

2. **grep/awk**: Text processing
   ```bash
   grep "ERROR" logs.txt | awk '{print $1, $2, $NF}'
   ```

3. **stern**: Kubernetes log tailing
   ```bash
   stern nexis -n nexis --since 1h
   ```

4. **kail**: Kubernetes log viewer
   ```bash
   kail -d nexis -n nexis
   ```

## Performance Tuning

### System-Level Tuning

#### File Descriptor Limits

```bash
# Check current limits
ulimit -n

# Increase limits (add to /etc/security/limits.conf)
* soft nofile 65535
* hard nofile 65535
```

#### TCP Tuning

```bash
# /etc/sysctl.conf
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 65535
net.ipv4.ip_local_port_range = 1024 65535
net.ipv4.tcp_tw_reuse = 1

# Apply changes
sysctl -p
```

### Application-Level Tuning

#### Worker Processes

```yaml
# config/production.yaml
server:
  workers: auto  # or specific number based on CPU cores
  max_connections: 10000
```

#### Connection Pooling

```yaml
database:
  pool_size: 100
  min_connections: 10
  max_connections: 200
  connection_timeout: 30s
  idle_timeout: 600s
```

#### Buffer Sizes

```yaml
server:
  read_buffer_size: 16384
  write_buffer_size: 16384
```

### Database Tuning

#### PostgreSQL

```sql
--postgresql.conf
shared_buffers = 4GB
effective_cache_size = 12GB
maintenance_work_mem = 1GB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 20MB
min_wal_size = 1GB
max_wal_size = 4GB
max_worker_processes = 8
max_parallel_workers_per_gather = 4
max_parallel_workers = 8
max_parallel_maintenance_workers = 4
```

#### Index Optimization

```sql
-- Analyze query performance
EXPLAIN ANALYZE SELECT * FROM users WHERE email = 'user@example.com';

-- Create index for frequent queries
CREATE INDEX CONCURRENTLY idx_users_email ON users(email);

-- Remove unused indexes
SELECT schemaname, tablename, indexname, idx_scan 
FROM pg_stat_user_indexes 
WHERE idx_scan = 0;
```

### Caching Strategies

#### Redis Configuration

```yaml
# config/redis.yaml
cache:
  enabled: true
  backend: redis
  ttl: 3600
  redis:
    host: redis
    port: 6379
    db: 0
    pool_size: 50
    timeout: 5s
```

#### Cache Invalidation

1. **Time-based**: TTL expiration
2. **Event-based**: Publish/subscribe
3. **Manual**: Admin API endpoint

### Load Testing

#### Using k6

```javascript
// load-test.js
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 100 },  // Ramp up to 100 users
    { duration: '5m', target: 100 },  // Stay at 100 users
    { duration: '2m', target: 200 },  // Ramp up to 200 users
    { duration: '5m', target: 200 },  // Stay at 200 users
    { duration: '2m', target: 0 },    // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests < 500ms
    http_req_failed: ['rate<0.01'],    // Error rate < 1%
  },
};

export default function() {
  let res = http.get('http://nexis:8080/api/health');
  check(res, { 'status was 200': (r) => r.status == 200 });
  sleep(1);
}
```

Run load test:
```bash
k6 run load-test.js
```

#### Using Apache Bench

```bash
# Simple test
ab -n 1000 -c 100 http://localhost:8080/api/health

# With POST data
ab -n 1000 -c 100 -p data.json -T application/json http://localhost:8080/api/users
```

### Performance Monitoring

1. **Real-time Metrics**: Use Grafana dashboards
2. **APM Tools**: Jaeger, Zipkin for distributed tracing
3. **Profiling**: Continuous profiling with tools like Pyroscope
4. **Synthetic Monitoring**: Regular health checks from multiple locations

## Troubleshooting Checklist

When issues occur:

1. [ ] Check service status and logs
2. [ ] Verify network connectivity
3. [ ] Review recent changes
4. [ ] Check resource usage (CPU, memory, disk)
5. [ ] Verify database connectivity and performance
6. [ ] Review metrics and dashboards
7. [ ] Check for error patterns in logs
8. [ ] Validate configuration
9. [ ] Test with minimal reproduction
10. [ ] Document findings and resolution

## Getting Help

If you can't resolve an issue:

1. **Check Documentation**: [docs/](../)
2. **Search Issues**: [GitHub Issues](https://github.com/your-org/nexis/issues)
3. **Community Support**: [Discord](https://discord.gg/nexis)
4. **Create Bug Report**: Include logs, metrics, and reproduction steps

## Related Documentation

- [Deployment Guide](./deployment.md)
- [Monitoring Guide](./monitoring.md)
