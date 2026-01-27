type InvokeCallback<T> = (error: string | null, result: T | null) => void;

interface ListenHandler<T> {
	(payload: T): void;
}

interface UnlistenFn {
	(): void;
}

interface TsckWindow extends Window {
	tsck: {
		invoke: (
			event: string,
			data?: any,
			callback?: (error: string | null, result: any) => void,
		) => number;
		call: (event: string, data?: any) => Promise<any>;
		send: (event: string, data?: any) => void;
		respond: (
			id: number,
			success: boolean,
			data: any,
			error: string | null,
		) => void;
		postMessage: ((message: string) => void) | null;
	};
	onIpc: (event: string, handler: (detail: any) => void) => void;
	__ipc_handler: (request: any) => void;
	__ipc_response_handler: (response: any) => void;
}

declare const window: TsckWindow;

// ============================================================================
// Helper Functions
// ============================================================================

function checkTsck(): boolean {
	if (typeof window === "undefined") {
		return false;
	}

	if (!window.tsck) {
		console.error(
			"tsck not initialized. Make sure the tsck IPC script is loaded.",
		);
		return false;
	}

	return true;
}

// ============================================================================
// Invoke - Call Rust from Frontend
// ============================================================================

/**
 * Call Rust with callback
 * @example
 * invoke<User>('get-user', { id: 123 }, (error, result) => {
 *   if (error) console.error(error);
 *   else console.log(result);
 * });
 */
export function invoke<T = any>(
	cmd: string,
	args?: any,
	callback?: InvokeCallback<T>,
): number {
	if (!checkTsck()) {
		callback?.("Tsck not available", null);
		return -1;
	}

	if (callback) {
		return window.tsck.invoke(
			cmd,
			args,
			(error: string | null, result: any) => {
				callback(error, result as T);
			},
		);
	} else {
		return window.tsck.invoke(cmd, args);
	}
}

/**
 * Call Rust and return Promise (async/await)
 * @example
 * const user = await invokeAsync<User>('get-user', { id: 123 });
 */
export async function invokeAsync<T = any>(
	cmd: string,
	args?: any,
): Promise<T> {
	if (!checkTsck()) {
		throw new Error("Tsck not available");
	}

	return window.tsck.call(cmd, args) as Promise<T>;
}

/**
 * Send payload to Rust frontend handler (fire and forget with type safety)
 * @example
 * interface FrontEndEvent {
 *   type: "FunctionCall";
 *   value: { func: string; args: any[] };
 * }
 *
 * invokePayload<FrontEndEvent>({
 *   type: "FunctionCall",
 *   value: { func: "layerToSmartObject", args: ["layer1"] }
 * });
 */
export function invokePayload<T = any>(payload: T): void {
	if (!checkTsck()) {
		return;
	}

	// Send to a specific frontend event handler
	window.tsck.send("frontend:payload|event", payload);
}

/**
 * Send payload to Rust frontend handler and wait for response
 * @example
 * const result = await invokePayloadAsync<FrontEndEvent, ResultType>({
 *   type: "FunctionCall",
 *   value: { func: "getLayer", args: ["layer1"] }
 * });
 */
export async function invokePayloadAsync<TPayload = any, TResult = any>(
	payload: TPayload,
): Promise<TResult> {
	if (!checkTsck()) {
		throw new Error("Tsck not available");
	}

	return window.tsck.call(
		"frontend:payload|event",
		payload,
	) as Promise<TResult>;
}

/**
 * Send payload to Rust frontend handler with callback
 * @example
 * invokePayloadWithCallback<FrontEndEvent, ResultType>(
 *   { type: "FunctionCall", value: { func: "test", args: [] } },
 *   (error, result) => {
 *     if (!error) console.log(result);
 *   }
 * );
 */
export function invokePayloadWithCallback<TPayload = any, TResult = any>(
	payload: TPayload,
	callback: InvokeCallback<TResult>,
): number {
	if (!checkTsck()) {
		callback("Tsck not available", null);
		return -1;
	}

	return window.tsck.invoke(
		"frontend:payload|event",
		payload,
		(error: string | null, result: any) => {
			callback(error, result as TResult);
		},
	);
}

// ============================================================================
// Listen - Receive events from Rust
// ============================================================================

/**
 * Listen to events from Rust
 * @example
 * const unlisten = listen<{ count: number }>('counter-updated', (payload) => {
 *   console.log('Count:', payload.count);
 * });
 *
 * // Later: unlisten();
 */
export function listen<E extends string, T = any>(
	event: E,
	handler: ListenHandler<T>,
): UnlistenFn {
	if (typeof window === "undefined") {
		console.warn("Cannot listen in SSR context");
		return () => {};
	}

	// Use Tsck's onIpc if available
	if (window.onIpc) {
		window.onIpc(event, (detail: any) => {
			handler(detail.data as T);
		});
	}

	// Fallback: create event listener reference for cleanup
	const eventListener = (e: Event) => {
		const customEvent = e as CustomEvent;
		if (customEvent.detail && customEvent.detail.data !== undefined) {
			handler(customEvent.detail.data as T);
		} else {
			// Fallback for direct payload
			handler(customEvent.detail as T);
		}
	};

	window.addEventListener(event, eventListener);

	// Return unlisten function
	return () => {
		window.removeEventListener(event, eventListener);
	};
}

/**
 * Listen once, then automatically unlisten
 * @example
 * listenOnce<string>('app-ready', (payload) => {
 *   console.log('App is ready:', payload);
 * });
 */
export function listenOnce<T = any>(
	event: string,
	handler: ListenHandler<T>,
): UnlistenFn {
	let unlisten: UnlistenFn;

	const wrappedHandler = (payload: T) => {
		handler(payload);
		unlisten?.();
	};

	unlisten = listen(event, wrappedHandler);
	return unlisten;
}

// ============================================================================
// Emit - Send events without expecting response
// ============================================================================

/**
 * Send event to Rust without waiting for response
 * @example
 * emit('log-event', { event: 'button-clicked', timestamp: Date.now() });
 */
export function emit<T = any>(event: string, payload?: T): void {
	if (!checkTsck()) {
		return;
	}

	window.tsck.send(event, payload);
}

// ============================================================================
// Respond - Respond to Rust-initiated calls
// ============================================================================

/**
 * Respond to a Rust-initiated IPC call
 * Typically used inside listen handlers when Rust expects a response
 * @example
 * listen('get-form-data', (payload) => {
 *   const formData = collectFormData();
 *   respond(payload.id, formData);
 * });
 */
export function respond<T = any>(id: number, data: T): void {
	if (!checkTsck()) {
		return;
	}

	window.tsck.respond(id, true, data, null);
}

/**
 * Respond with error to a Rust-initiated IPC call
 * @example
 * listen('validate-data', (payload) => {
 *   if (!isValid(payload.data)) {
 *     respondError(payload.id, 'Invalid data');
 *   }
 * });
 */
export function respondError(id: number, error: string): void {
	if (!checkTsck()) {
		return;
	}

	window.tsck.respond(id, false, null, error);
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Check if Tsck is available
 */
export function isAvailable(): boolean {
	return checkTsck();
}

/**
 * Wait for Tsck to be ready
 * @param timeout - Maximum time to wait in milliseconds (default: 5000)
 */
export async function waitForReady(timeout = 5000): Promise<void> {
	const start = Date.now();

	while (!checkTsck()) {
		if (Date.now() - start > timeout) {
			throw new Error("Tsck initialization timeout");
		}
		await new Promise((resolve) => setTimeout(resolve, 100));
	}
}

// ============================================================================
// Export all functions
// ============================================================================

export default {
	invoke,
	invokeAsync,
	invokePayload,
	invokePayloadAsync,
	invokePayloadWithCallback,
	listen,
	listenOnce,
	emit,
	respond,
	respondError,
	isAvailable,
	waitForReady,
};
