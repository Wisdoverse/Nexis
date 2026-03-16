# Monitoring Guide

This guide covers monitoring setup with Prometheus, Grafana dashboards, and alerting rules for Nexis.

## Prometheus Metrics

### Available Metrics

Nexis exposes the following Prometheus metrics at the `/metrics` endpoint (default port: 9090):

#### Application Metrics

| Metric Name | Type | Description | Labels |
|-------------|------|-------------|--------|
| `nexis_requests_total` | Counter | Total number of HTTP requests | `method`, `path`, `status` |
| `nexis_request_duration_seconds` | Histogram | HTTP request duration | `method`, `path` |
| `nexis_requests_in_flight` | Gauge | Number of requests currently being processed | `method` |
| `nexis_connections_total` | Counter | Total number of connections | `type` |
| `nexis_active_connections` | Gauge | Number of active connections | `type` |

#### Database Metrics

| Metric Name | Type | Description | Labels |
|-------------|------|-------------|--------|
| `nexis_db_connections` | Gauge | Number of database connections | `state` |
| `nexis_db_query_duration_seconds` | Histogram | Database query duration | `query_type` |
| `nexis_db_errors_total` | Counter | Total database errors | `error_type` |

#### Cache Metrics

| Metric Name | Type | Description | Labels |
|-------------|------|-------------|--------|
| `nexis_cache_hits_total` | Counter | Total cache hits | `cache_name` |
| `nexis_cache_misses_total` | Counter | Total cache misses | `cache_name` |
| `nexis_cache_evictions_total` | Counter | Total cache evictions | `cache_name` |

#### Business Metrics

| Metric Name | Type | Description | Labels |
|-------------|------|-------------|--------|
| `nexis_users_active` | Gauge | Number of active users | - |
| `nexis_operations_total` | Counter | Total operations performed | `operation_type` |

### Prometheus Configuration

#### Basic Prometheus Setup

1. Install Prometheus:
   ```bash
   docker run -d --name=prometheus \
     -p 9090:9090 \
     -v /path/to/prometheus.yml:/etc/prometheus/prometheus.yml \
     prom/prometheus
   ```

2. Configure `prometheus.yml`:
   ```yaml
   global:
     scrape_interval: 15s
     evaluation_interval: 15s

   scrape_configs:
     - job_name: 'nexis'
       static_configs:
         - targets: ['nexis:9090']
       metrics_path: /metrics
   ```

3. Verify metrics are being collected:
   ```bash
   curl http://localhost:9090/api/v1/targets
   ```

#### Advanced Configuration with Kubernetes

For Kubernetes deployments using Prometheus Operator:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: nexis
  namespace: nexis
spec:
  selector:
    matchLabels:
      app: nexis
  endpoints:
  - port: metrics
    interval: 30s
    path: /metrics
```

### Custom Metrics

To add custom metrics to your application:

```rust
use prometheus::{Counter, HistogramVec, Registry};

lazy_static! {
    static ref CUSTOM_COUNTER: Counter = Counter::new(
        "nexis_custom_operations_total",
        "Total custom operations"
    ).unwrap();
    
    static ref CUSTOM_HISTOGRAM: HistogramVec = HistogramVec::new(
        HistogramOpts::new("nexis_custom_duration_seconds", "Custom operation duration"),
        &["operation"]
    ).unwrap();
}

// Register metrics
registry.register(Box::new(CUSTOM_COUNTER.clone())).unwrap();
registry.register(Box::new(CUSTOM_HISTOGRAM.clone())).unwrap();
```

## Grafana Configuration

### Installing Grafana

1. Install Grafana:
   ```bash
   docker run -d --name=grafana \
     -p 3000:3000 \
     grafana/grafana
   ```

2. Access Grafana at `http://localhost:3000` (default: admin/admin)

3. Add Prometheus as a data source:
   - Go to Configuration → Data Sources
   - Add data source → Prometheus
   - URL: `http://prometheus:9090`
   - Save & Test

### Pre-built Dashboards

Nexis provides pre-built Grafana dashboards in the `deploy/grafana/dashboards/` directory:

1. **Overview Dashboard**: High-level system health and metrics
2. **Performance Dashboard**: Detailed performance metrics and latency
3. **Database Dashboard**: Database connections, queries, and performance
4. **Business Dashboard**: Business-specific metrics and KPIs

#### Importing Dashboards

1. In Grafana, go to Dashboards → Import
2. Upload the JSON file from `deploy/grafana/dashboards/`
3. Select Prometheus data source
4. Click Import

### Key Dashboard Panels

#### System Health Panel

```yaml
title: System Health
type: stat
targets:
  - expr: up{job="nexis"}
    legendFormat: "Status"
```

#### Request Rate Panel

```yaml
title: Request Rate
type: graph
targets:
  - expr: rate(nexis_requests_total[5m])
    legendFormat: "{{method}} {{path}}"
```

#### Latency Heatmap

```yaml
title: Request Latency
type: heatmap
targets:
  - expr: rate(nexis_request_duration_seconds_bucket[5m])
    format: heatmap
    legendFormat: "{{le}}"
```

#### Error Rate Panel

```yaml
title: Error Rate
type: graph
targets:
  - expr: rate(nexis_requests_total{status=~"5.."}[5m])
    legendFormat: "5xx Errors"
```

### Dashboard Variables

Create reusable dashboard variables:

- `$instance`: Filter by instance
- `$job`: Filter by job
- `$interval`: Time interval for aggregations

Example variable configuration:
```yaml
name: instance
type: query
query: label_values(up, instance)
refresh: on_time_range_change
```

## Alerting Rules

### AlertManager Configuration

1. Install AlertManager:
   ```bash
   docker run -d --name=alertmanager \
     -p 9093:9093 \
     -v /path/to/alertmanager.yml:/etc/alertmanager/alertmanager.yml \
     prom/alertmanager
   ```

2. Configure `alertmanager.yml`:
   ```yaml
   global:
     resolve_timeout: 5m
     smtp_smarthost: 'smtp.example.com:587'
     smtp_from: 'alerts@example.com'
     smtp_auth_username: 'alerts@example.com'
     smtp_auth_password: 'password'

   route:
     group_by: ['alertname', 'severity']
     group_wait: 10s
     group_interval: 10s
     repeat_interval: 12h
     receiver: 'team-email'
     routes:
       - match:
           severity: critical
         receiver: 'team-pagerduty'

   receivers:
     - name: 'team-email'
       email_configs:
         - to: 'team@example.com'
     - name: 'team-pagerduty'
       pagerduty_configs:
         - service_key: 'your-service-key'
   ```

### Pre-defined Alert Rules

Nexis provides alert rules in `deploy/prometheus/alerts.yml`:

#### High Error Rate

```yaml
groups:
  - name: nexis
    rules:
      - alert: HighErrorRate
        expr: rate(nexis_requests_total{status=~"5.."}[5m]) > 0.1
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} req/s"
```

#### High Latency

```yaml
      - alert: HighLatency
        expr: histogram_quantile(0.95, rate(nexis_request_duration_seconds_bucket[5m])) > 1
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High latency detected"
          description: "95th percentile latency is {{ $value }}s"
```

#### Service Down

```yaml
      - alert: ServiceDown
        expr: up{job="nexis"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Service is down"
          description: "{{ $labels.instance }} is down"
```

#### Database Connection Pool Exhausted

```yaml
      - alert: DatabasePoolExhausted
        expr: nexis_db_connections{state="idle"} < 5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Database connection pool nearly exhausted"
          description: "Only {{ $value }} idle connections remaining"
```

#### High Memory Usage

```yaml
      - alert: HighMemoryUsage
        expr: process_resident_memory_bytes{job="nexis"} / 1024 / 1024 / 1024 > 8
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage"
          description: "Memory usage is {{ $value }}GB"
```

#### Cache Hit Rate Low

```yaml
      - alert: LowCacheHitRate
        expr: rate(nexis_cache_hits_total[5m]) / (rate(nexis_cache_hits_total[5m]) + rate(nexis_cache_misses_total[5m])) < 0.7
        for: 15m
        labels:
          severity: warning
        annotations:
          summary: "Low cache hit rate"
          description: "Cache hit rate is {{ $value | humanizePercentage }}"
```

### Adding Alert Rules

1. Create new alert rules in `deploy/prometheus/alerts.yml`
2. Reload Prometheus configuration:
   ```bash
   curl -X POST http://localhost:9090/-/reload
   ```
3. Verify alerts are loaded:
   ```bash
   curl http://localhost:9090/api/v1/rules
   ```

### Alert Severity Levels

- **Critical**: Requires immediate attention (service down, data loss risk)
- **Warning**: Needs investigation but not urgent
- **Info**: Informational, no action required

## Monitoring Best Practices

1. **Use Labels Consistently**: Apply consistent labels across metrics
2. **Set Meaningful Thresholds**: Base thresholds on SLIs/SLOs
3. **Document Alerts**: Include runbooks in alert annotations
4. **Avoid Alert Fatigue**: Tune thresholds to reduce noise
5. **Test Alerts**: Regularly verify alert delivery
6. **Monitor the Monitor**: Track Prometheus and Grafana health

## Additional Resources

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [AlertManager Configuration](https://prometheus.io/docs/alerting/latest/configuration/)
- [Troubleshooting Guide](./troubleshooting.md)
