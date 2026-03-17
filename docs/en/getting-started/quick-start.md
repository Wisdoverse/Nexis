# Quick Start — Deploy in 5 Minutes

Get Nexis up and running with Docker Compose in under 5 minutes.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) ≥ 20.10
- [Docker Compose](https://docs.docker.com/compose/install/) ≥ 2.0
- 2 GB RAM minimum

## 1. Clone & Start

```bash
git clone https://github.com/gbrothersgroup/Nexis.git
cd Nexis

# One-command deploy
docker-compose up -d
```

That's it. Visit **http://localhost:8080** to access the platform.

## 2. Verify

```bash
# Check all services are running
docker-compose ps

# Health check
curl http://localhost:8080/health
# → {"status":"ok","version":"0.1.0"}
```

## 3. Send Your First Message

### via cURL

```bash
# Authenticate
TOKEN=$(curl -s -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}' | jq -r '.token')

# Send a message to #general channel
curl -X POST http://localhost:8080/api/v1/channels/general/messages \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content":"Hello from Nexis! 🚀"}'
```

### via WebSocket

```javascript
const ws = new WebSocket('ws://localhost:8080/ws?token=' + TOKEN);

ws.onopen = () => {
  // Subscribe to #general channel
  ws.send(JSON.stringify({
    type: 'subscribe',
    channel: 'general'
  }));
  
  // Send a message
  ws.send(JSON.stringify({
    type: 'message',
    channel: 'general',
    content: 'Real-time message via WebSocket! ⚡'
  }));
};

ws.onmessage = (event) => {
  console.log('Received:', JSON.parse(event.data));
};
```

## 4. SDK Integration

### TypeScript SDK

```bash
npm install @nexis/sdk
```

```typescript
import { NexisClient } from '@nexis/sdk';

const client = new NexisClient({
  baseUrl: 'http://localhost:8080',
  token: process.env.NEXIS_TOKEN,
});

async function main() {
  // List channels
  const channels = await client.channels.list();
  console.log('Channels:', channels);

  // Send a message
  await client.messages.send('general', {
    content: 'Hello from TypeScript SDK! 🎯',
  });

  // Subscribe to real-time events
  client.on('message', (msg) => {
    console.log(`[${msg.channel}] ${msg.author}: ${msg.content}`);
  });

  // AI-powered summary (when AI module is enabled)
  const summary = await client.channels.summarize('general', {
    last: '24h',
  });
  console.log('Summary:', summary.text);
}
```

### Python SDK

```bash
pip install nexis-sdk
```

```python
import nexis_sdk

client = nexis_sdk.Client(
    base_url="http://localhost:8080",
    token="your-token-here",
)

# Send a message
client.messages.send("general", content="Hello from Python SDK! 🐍")

# List messages
messages = client.messages.list("general", limit=10)
for msg in messages:
    print(f"[{msg.channel}] {msg.author}: {msg.content}")

# Get AI summary
summary = client.channels.summarize("general", last="24h")
print("AI Summary:", summary.text)
```

## What's Next?

- [Development Guide](/en/getting-started/development-guide) — set up a local dev environment
- [Architecture Overview](/en/architecture/) — understand the system design
- [API Reference](/en/api/metrics) — full REST & WebSocket API docs
- [Roadmap](/en/roadmap) — see what's coming next
