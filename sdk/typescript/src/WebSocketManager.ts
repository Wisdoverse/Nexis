import WebSocket from 'ws';
import type { ClientMessage, MessageHandler } from './types';

export class WebSocketManager {
  private ws: WebSocket | null = null;
  private messageHandler: MessageHandler | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private shouldReconnect = false;
  private url: string = '';
  private token: string = '';

  connect(url: string, token: string): void {
    this.url = url;
    this.token = token;
    this.shouldReconnect = true;
    this.reconnectAttempts = 0;
    this.doConnect();
  }

  private doConnect(): void {
    const headers = this.token ? { Authorization: `Bearer ${this.token}` } : undefined;
    this.ws = new WebSocket(this.url, { headers } as WebSocket.ClientOptions);

    this.ws.on('open', () => {
      this.reconnectAttempts = 0;
      console.log('WebSocket connected');
    });

    this.ws.on('message', (data: WebSocket.Data) => {
      try {
        const message = JSON.parse(data.toString());
        this.messageHandler?.(message);
      } catch {
        // ignore malformed messages
      }
    });

    this.ws.on('close', () => {
      if (this.shouldReconnect && this.reconnectAttempts < this.maxReconnectAttempts) {
        this.scheduleReconnect();
      }
    });

    this.ws.on('error', () => {
      this.ws?.close();
    });
  }

  send(message: ClientMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    } else {
      throw new Error('WebSocket is not connected');
    }
  }

  onMessage(handler: MessageHandler): void {
    this.messageHandler = handler;
  }

  close(): void {
    this.shouldReconnect = false;
    this.ws?.close();
    this.ws = null;
  }

  isConnected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  private scheduleReconnect(): void {
    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
    console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);
    setTimeout(() => {
      if (this.shouldReconnect) {
        this.doConnect();
      }
    }, delay);
  }
}
