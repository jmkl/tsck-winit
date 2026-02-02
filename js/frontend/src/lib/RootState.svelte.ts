import { getContext, setContext } from "svelte";

let contextInstance: Root | null = null;
const MAINCTX = Symbol("RootContext");

function getInstance() {
  if (!contextInstance) contextInstance = new Root();
  return contextInstance;
}

export function SetRootState() {
  setContext(MAINCTX, getInstance());
}

export function GetRootState() {
  return getContext<Root>(MAINCTX);
}

class Root {
  IsConnected = $state(false);
  constructor() {}
}
