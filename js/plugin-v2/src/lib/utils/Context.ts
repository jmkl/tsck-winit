// Context.ts
type ContextKey = symbol;
const contextMap = new Map<ContextKey, any>(); // Use Map instead of WeakMap

export function setContext<T>(key: ContextKey, value: T): void {
  contextMap.set(key, value);
}

export function getContext<T>(key: ContextKey): T | undefined {
  return contextMap.get(key);
}

export function requireContext<T>(key: ContextKey, errorMsg: string): T {
  const ctx = contextMap.get(key);
  if (!ctx) throw new Error(errorMsg);
  return ctx;
}
// // context.ts
// type ContextKey = string | symbol;

// const contextMap = new WeakMap<object, Map<ContextKey, any>>();

// /**
//  * Set a value in the context of a given owner object
//  */
// export function setContext<T>(owner: object, key: ContextKey, value: T) {
//     let map = contextMap.get(owner);
//     if (!map) {
//         map = new Map();
//         contextMap.set(owner, map);
//     }
//     map.set(key, value);
// }

// /**
//  * Get a value from the context of a given owner object
//  */
// export function getContext<T>(owner: object, key: ContextKey): T | undefined {
//     const map = contextMap.get(owner);
//     return map?.get(key);
// }
