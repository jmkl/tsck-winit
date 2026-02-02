// socketContext.ts
import { getContext, requireContext, setContext } from "../utils/Context";
import { SocketServer } from "./socketServer";

const SOCKET_KEY = Symbol("SocketContext");

export function initSocketServer(): SocketServer {
  let instance = getContext<SocketServer>(SOCKET_KEY);

  if (!instance) {
    instance = new SocketServer();
    setContext(SOCKET_KEY, instance);
  }

  return instance;
}

export function getSocketServerState(): SocketServer {
  return requireContext<SocketServer>(
    SOCKET_KEY,
    "SocketServer not initialized. Call initSocketServer() first.",
  );
}

// import { getContext, setContext } from "../utils/Context";
// import { SocketServer } from "./socketServer";

// const SCKCTX = Symbol("SocketContext");
// const owner = { randomKey: 69 };

// let socketInstance: SocketServer | null = null;

// function getInstance(): SocketServer {
//   if (!socketInstance) socketInstance = new SocketServer();
//   return socketInstance;
// }

// export function setSocketServerState() {
//   setContext(owner, SCKCTX, getInstance());
// }

// export function getSocketServerState(): SocketServer {
//   const ctx = getContext<SocketServer>(owner, SCKCTX);
//   if (!ctx) throw new Error("SocketServer context not initialized. Call setSocketServerState() first.");
//   return ctx;
// }
