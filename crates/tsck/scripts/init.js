(function () {
	// document.addEventListener("mousedown", (e) => {
	// 	if (e.target.hasAttribute("data-tsck-drag-region") && e.button === 0) {
	// 		// e.detail === 2
	// 		// 	? window.tsck.send("[IpcEvent]", { type: "Fuck" }) :
	// 		window.tsck.send("[IpcEvent]", { type: "DragWindow" });
	// 	} else {
	// 		// window.tsck.send("[IpcEvent]", {
	// 		// 	type: "MouseDown",
	// 		// 	value: [e.clientX, e.clientY],
	// 		// });
	// 	}
	// 	// const attr = e.target.getAttribute("data-tsck-drag-region");
	// 	// if (attr != null && e.button === 0) {
	// 	// 	e.detail === 2
	// 	// 		? window.tsck.send("[IpcEvent]", { type: "Maximize" })
	// 	// 		: window.tsck.send("[IpcEvent]", { type: "DragWindow" });
	// 	// } else {
	// 	// 	window.tsck.send("[IpcEvent]", {
	// 	// 		type: "MouseDown",
	// 	// 		value: [e.clientX, e.clientY],
	// 	// 	});
	// 	// }
	// });
	// document.addEventListener("mousemove", (e) => {
	// 	window.tsck.send("[IpcEvent]", {
	// 		type: "MouseMove",
	// 		value: [e.clientX, e.clientY],
	// 	});
	// });
	const pendingCallbacks = new Map();
	let callIdCounter = 1;

	const tsckIpc = {
		invoke: function (event, dataOrCallback, callbackOrUndefined) {
			let data = null;
			let callback = null;

			if (typeof dataOrCallback === "function") {
				callback = dataOrCallback;
			} else {
				data = dataOrCallback;
				callback = callbackOrUndefined;
			}

			const id = callIdCounter++;

			if (callback && typeof callback === "function") {
				pendingCallbacks.set(id, callback);
			}

			const message = {
				id: id,
				event: event,
				data: data,
			};

			if (window.ipc && window.ipc.postMessage) {
				window.ipc.postMessage(JSON.stringify(message));
			}

			return id;
		},

		call: function (event, data) {
			return new Promise((resolve, reject) => {
				this.invoke(event, data, (error, result) => {
					if (error) {
						reject(error);
					} else {
						resolve(result);
					}
				});
			});
		},

		send: function (event, data) {
			const message = {
				id: 0,
				event: event,
				data: data,
			};

			if (window.ipc && window.ipc.postMessage) {
				window.ipc.postMessage(JSON.stringify(message));
			}
		},

		respond: function (id, success, data, error) {
			const response = {
				id: id,
				success: success,
				data: data,
				error: error,
			};

			if (window.ipc && window.ipc.postMessage) {
				window.ipc.postMessage(JSON.stringify(response));
			}
		},

		// Keep reference to original postMessage
		postMessage: window.ipc ? window.ipc.postMessage : null,
	};

	// Expose as window.tsck
	window.tsck = tsckIpc;

	// Also add helper for listening to Rust calls
	window.__ipc_handler = function (request) {
		const event = new CustomEvent("" + request.event, {
			detail: {
				id: request.id,
				data: request.data,
				respond: function (data) {
					tsckIpc.respond(request.id, true, data, null);
				},
				error: function (error) {
					tsckIpc.respond(request.id, false, null, error);
				},
			},
		});
		window.dispatchEvent(event);
	};

	window.__ipc_response_handler = function (response) {
		const callback = pendingCallbacks.get(response.id);
		if (callback) {
			if (response.success) {
				callback(null, response.data);
			} else {
				callback(response.error || "Unknown error", null);
			}
			pendingCallbacks.delete(response.id);
		}
	};

	window.onIpc = function (event, handler) {
		console.log("registering event: ", event);
		window.addEventListener("" + event, (e) => {
			handler(e.detail);
		});
	};

	console.log("tsck IPC system initialized");
	console.log("Use window.tsck.invoke() for callbacks");
})();
