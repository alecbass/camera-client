interface Options {
  onMessage?: (data: Blob) => void;
}

export class PersistentWebSocket {
  private ws: WebSocket;
  private options: Options;

  constructor(endpoint: string, options?: Options) {
    this.options = options ?? {};
    this.ws = new WebSocket(endpoint);
    this.ws.onopen = (e) => {};
    this.ws.onmessage = (message: MessageEvent) => {
      const data = message.data as Blob;
      if (this.options.onMessage) {
        this.options.onMessage(data);
      }
    };
    this.ws.onclose = (e) => {};
    this.ws.onerror = (e) => {};
  }

  open() {
    this.ws.send("hello");
  }

  close() {
    this.ws.close();
  }

  send(data: Parameters<WebSocket["send"]>[0]) {
    if (this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(data);
    } else {
      console.warn("Tried sending to a closed websocket");
    }
  }

  handleMessage(this: WebSocket, message: MessageEvent) {
    const data = message.data as Blob;
    console.debug(`Received`, data);
  }
}
