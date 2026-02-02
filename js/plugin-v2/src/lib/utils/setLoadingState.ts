//setloading.ts
import { getSocketServerState } from "../socket/socketContext";
import { logger } from "./addLog";
export function setLoadingState(state: boolean) {
  getSocketServerState().toggleLoading(state);
  logger.warn("Loading State", state);
}
