import { getContext, requireContext, setContext } from "../utils/Context";
import { TokenHelper } from "./token";

const TOKEN_KEY = Symbol("TokenContext");

export async function initTokenState(): Promise<TokenHelper> {
  let instance = getContext<TokenHelper>(TOKEN_KEY);

  if (!instance) {
    instance = new TokenHelper();
    Promise.resolve(instance.init());
    setContext(TOKEN_KEY, instance);
  }

  return instance;
}

export function getTokenHelperState(): TokenHelper {
  return requireContext<TokenHelper>(
    TOKEN_KEY,
    "SocketServer not initialized. Call initSocketServer() first.",
  );
}

// import { getContext, setContext } from "../utils/Context";
// import { TokenHelper } from "./token";

// const SCKCTX = Symbol("TokenContext");
// const owner = { randomKey: 70 };

// let socketInstance: TokenHelper | null = null;

// function getInstance(): TokenHelper {
//   if (!socketInstance) socketInstance = new TokenHelper();
//   Promise.resolve(socketInstance.init());
//   return socketInstance;
// }

// export function setTokenHelperState() {
//   setContext(owner, SCKCTX, getInstance());
// }

// export function getTokenHelperState(): TokenHelper {
//   const ctx = getContext<TokenHelper>(owner, SCKCTX);
//   if (!ctx)
//     throw new Error(
//       "TokenHelper context not initialized. Call setTokenHelperState() first.",
//     );
//   return ctx;
// }
