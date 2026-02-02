import { SocketEventMap } from "./socketType";

const SOCKET_URL = "ws://127.0.0.1:1818";

export class SocketServer extends EventTarget {
  private socket: WebSocket | null = null;
  private reconnectTimeout: number | null = null;
  delay: number = 3000;

  constructor() {
    super();
    this.connect();
  }

  connect() {
    // Clear any existing reconnect timeout
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }

    // Close existing socket if any
    if (this.socket) {
      this.socket.onclose = null;
      this.socket.onerror = null;
      this.socket.onmessage = null;
      this.socket.onopen = null;
      if (
        this.socket.readyState === WebSocket.OPEN ||
        this.socket.readyState === WebSocket.CONNECTING
      ) {
        this.socket.close();
      }
    }

    this.socket = new WebSocket(SOCKET_URL);

    this.socket.onopen = () => {
      console.log("WebSocket connected");
      this.dispatchEvent(new Event("open"));
    };

    this.socket.onmessage = (e) => {
      try {
        const data = JSON.parse(e.data);
        const event = new CustomEvent("payload|server", { detail: data });
        this.dispatchEvent(event);
      } catch (err) {}
    };

    this.socket.onclose = (event) => {
      this.dispatchEvent(
        new CustomEvent("error", {
          detail: `reconnect in ${this.delay / 1000}s`,
        }),
      );
      this.reconnectTimeout = window.setTimeout(
        () => this.connect(),
        this.delay,
      );
    };

    this.socket.onerror = (e) => {
      this.dispatchEvent(new CustomEvent("error", { detail: e }));
    };

    // REMOVED: Don't immediately close the socket after connecting!
    // this.socket.close();
  }
  toggleLoading(loading: boolean) {
    this.sendMessage({
      from_server: false,
      type: "show-loading",
      content: loading,
    });
  }
  sendFacerestorePreviewImage(base64image: string[]) {
    this.sendMessage({
      from_server: false,
      type: "facerestore-preview-image",
      content: base64image,
    });
  }
  // Type-safe sending
  sendMessage<T>(message: T): boolean {
    if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
      return false;
    }
    this.socket.send(JSON.stringify(message));
    return true;
  }

  // Typed addEventListener
  on<K extends keyof SocketEventMap>(
    type: K,
    listener: (event: SocketEventMap[K]) => void,
  ) {
    this.addEventListener(type, listener as EventListener);
  }

  off<K extends keyof SocketEventMap>(
    type: K,
    listener: (event: SocketEventMap[K]) => void,
  ) {
    this.removeEventListener(type, listener as EventListener);
  }

  // Clean disconnect
  disconnect() {
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }
    if (this.socket) {
      this.socket.onclose = null; // Prevent reconnect
      this.socket.close();
      this.socket = null;
    }
  }
}
