/**
 * Simple DOM element creator
 */
function $el(t, e = {}) {
	let s = document.createElement(t);
	if (
		(e.text && (s.textContent = e.text),
		e.class && (s.className = e.class),
		e.value && (s.value = e.value),
		e.placeholder && (s.placeholder = e.placeholder),
		e.style && Object.assign(s.style, e.style),
		e.attrs)
	)
		for (let [n, r] of Object.entries(e.attrs)) s.setAttribute(n, r);
	if (e.on) for (let [a, l] of Object.entries(e.on)) s.addEventListener(a, l);
	return (e.parent && e.parent.appendChild(s), s);
}

/**
 * Send message to Rust backend
 */
function sendToBackend(mode, value) {
	globalThis.tsck.send("[browser::IpcEvent]", {
		type: mode,
		value: value,
	});
}

// ============================================================================
// IMAGE DOWNLOAD WATCHER
// ============================================================================

// ============================================================================
// TOAST NOTIFICATIONS
// ============================================================================

class Toast {
	static container = null;

	static init() {
		if (Toast.container) return;

		Toast.container = $el("div", {
			style: {
				position: "fixed",
				bottom: "20px",
				right: "20px",
				zIndex: "999999",
				display: "flex",
				flexDirection: "column",
				gap: "10px",
				pointerEvents: "none",
			},
			parent: document.body,
		});

		// Add animations
		if (!document.getElementById("toast-animations")) {
			const style = document.createElement("style");
			style.id = "toast-animations";
			style.textContent = `
				@keyframes slideIn {
					from { transform: translateX(400px); opacity: 0; }
					to { transform: translateX(0); opacity: 1; }
				}
				@keyframes slideOut {
					from { transform: translateX(0); opacity: 1; }
					to { transform: translateX(400px); opacity: 0; }
				}
			`;
			document.head.appendChild(style);
		}
	}

	static show(type, message, duration = 3000) {
		Toast.init();

		const colors = {
			success: { bg: "#4CAF50", icon: "✓" },
			error: { bg: "#f44336", icon: "✕" },
			warning: { bg: "#ff9800", icon: "⚠" },
			info: { bg: "#2196F3", icon: "ℹ" },
		};

		const config = colors[type] || colors.info;

		const toast = $el("div", {
			style: {
				background: config.bg,
				color: "white",
				padding: "12px 20px",
				borderRadius: "8px",
				boxShadow: "0 4px 12px rgba(0,0,0,0.3)",
				display: "flex",
				alignItems: "center",
				gap: "10px",
				minWidth: "250px",
				maxWidth: "400px",
				fontFamily: "system-ui, -apple-system, sans-serif",
				fontSize: "14px",
				pointerEvents: "auto",
				animation: "slideIn 0.3s ease-out",
				cursor: "pointer",
			},
			parent: Toast.container,
		});

		$el("span", {
			text: config.icon,
			style: { fontSize: "18px", fontWeight: "bold" },
			parent: toast,
		});
		$el("span", {
			text: message,
			style: { flex: "1", wordBreak: "break-word" },
			parent: toast,
		});

		toast.addEventListener("click", () => Toast.dismiss(toast));

		if (duration > 0) {
			setTimeout(() => Toast.dismiss(toast), duration);
		}

		return toast;
	}

	static dismiss(toast) {
		toast.style.animation = "slideOut 0.3s ease-in";
		setTimeout(() => toast.remove(), 300);
	}

	static success(msg, dur) {
		return Toast.show("success", msg, dur);
	}
	static error(msg, dur) {
		return Toast.show("error", msg, dur);
	}
	static warning(msg, dur) {
		return Toast.show("warning", msg, dur);
	}
	static info(msg, dur) {
		return Toast.show("info", msg, dur);
	}
}

class ImageDownloadWatcher {
	constructor(options = {}) {
		this.onDownload = options.onDownload || (() => {});
		this.minWidth = options.minWidth || 100;
		this.minHeight = options.minHeight || 100;
		this.icon = options.icon || "⬇";

		this.processed = new WeakSet();
		this.observer = null;
	}

	start() {
		document.querySelectorAll("img").forEach((img) => this.processImage(img));

		this.observer = new MutationObserver((mutations) => {
			for (const mut of mutations) {
				if (mut.type === "childList") {
					mut.addedNodes.forEach((node) => {
						if (node.nodeType === 1) {
							if (node.tagName === "IMG") this.processImage(node);
							else {
								node
									.querySelectorAll?.("img")
									.forEach((img) => this.processImage(img));
							}
						}
					});
				}
			}
		});

		this.observer.observe(document.body, { childList: true, subtree: true });
	}

	stop() {
		this.observer?.disconnect();
	}

	processImage(img) {
		if (this.processed.has(img)) return;

		if (!img.complete) {
			img.addEventListener("load", () => this.processImage(img), {
				once: true,
			});
			return;
		}

		if (img.naturalWidth < this.minWidth || img.naturalHeight < this.minHeight)
			return;

		this.processed.add(img);
		this.addButton(img);
	}

	addButton(img) {
		const url = img.src || img.currentSrc;
		if (!url) return;

		let wrapper = img.parentElement;
		const pos = getComputedStyle(wrapper).position;

		if (pos === "static") {
			const newWrapper = $el("div", {
				style: { position: "relative", display: "inline-block", lineHeight: 0 },
			});
			wrapper.insertBefore(newWrapper, img);
			newWrapper.appendChild(img);
			wrapper = newWrapper;
		}

		const btn = $el("button", {
			text: this.icon,
			style: {
				position: "absolute",
				top: "8px",
				right: "8px",
				width: "32px",
				height: "32px",
				border: "none",
				borderRadius: "50%",
				background: "rgba(0,0,0,0.7)",
				color: "#fff",
				fontSize: "16px",
				cursor: "pointer",
				display: "flex",
				alignItems: "center",
				justifyContent: "center",
				zIndex: "1000",
				opacity: "0",
				transition: "all 0.2s",
			},
			on: {
				click: (e) => {
					e.preventDefault();
					e.stopPropagation();
					this.onDownload(url, img);
				},
				mouseenter: (e) => {
					e.target.style.background = "rgba(0,102,204,0.9)";
					e.target.style.transform = "scale(1.1)";
				},
				mouseleave: (e) => {
					e.target.style.background = "rgba(0,0,0,0.7)";
					e.target.style.transform = "scale(1)";
				},
			},
			parent: wrapper,
		});

		wrapper.addEventListener("mouseenter", () => (btn.style.opacity = "1"));
		wrapper.addEventListener("mouseleave", () => (btn.style.opacity = "0"));
	}
}

// ============================================================================
// VIDEO SCREENSHOT
// ============================================================================

class VideoScreenshot {
	constructor(options = {}) {
		this.onScreenshot = options.onScreenshot || (() => {});
		this.icon = options.icon || "📸";
		this.format = options.format || "png";
		this.quality = options.quality || 0.95;

		this.processed = new WeakSet();
		this.observer = null;
	}

	start() {
		document.querySelectorAll("video").forEach((v) => this.processVideo(v));

		this.observer = new MutationObserver((mutations) => {
			for (const mut of mutations) {
				if (mut.type === "childList") {
					mut.addedNodes.forEach((node) => {
						if (node.nodeType === 1) {
							if (node.tagName === "VIDEO") this.processVideo(node);
							else {
								node
									.querySelectorAll?.("video")
									.forEach((v) => this.processVideo(v));
							}
						}
					});
				}
			}
		});

		this.observer.observe(document.body, { childList: true, subtree: true });
	}

	stop() {
		this.observer?.disconnect();
	}

	processVideo(video) {
		if (this.processed.has(video)) return;
		this.processed.add(video);
		this.addButton(video);
	}

	capture(video) {
		const canvas = document.createElement("canvas");
		canvas.width = video.videoWidth;
		canvas.height = video.videoHeight;

		const ctx = canvas.getContext("2d");
		ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

		const mime = this.format === "jpeg" ? "image/jpeg" : "image/png";
		return canvas.toDataURL(mime, this.quality);
	}

	addButton(video) {
		const container = video.parentElement;
		if (getComputedStyle(container).position === "static") {
			container.style.position = "relative";
		}

		const btn = $el("button", {
			text: this.icon,
			style: {
				position: "absolute",
				top: "10px",
				right: "10px",
				width: "40px",
				height: "40px",
				border: "none",
				borderRadius: "50%",
				background: "rgba(0,0,0,0.7)",
				color: "#fff",
				fontSize: "20px",
				cursor: "pointer",
				display: "flex",
				alignItems: "center",
				justifyContent: "center",
				zIndex: "9999",
				opacity: "0",
				transition: "all 0.2s",
			},
			on: {
				click: (e) => {
					e.preventDefault();
					e.stopPropagation();

					try {
						const dataUrl = this.capture(video);
						const time = Math.floor(video.currentTime);
						const filename = `screenshot-${time}s.${this.format}`;

						this.onScreenshot(dataUrl, video, filename);

						btn.style.background = "rgba(0,255,0,0.8)";
						setTimeout(() => (btn.style.background = "rgba(0,0,0,0.7)"), 300);
					} catch (err) {
						Toast.error("Screenshot failed");
						btn.style.background = "rgba(255,0,0,0.8)";
						setTimeout(() => (btn.style.background = "rgba(0,0,0,0.7)"), 300);
					}
				},
				mouseenter: (e) => {
					e.target.style.background = "rgba(0,102,204,0.9)";
					e.target.style.transform = "scale(1.1)";
				},
				mouseleave: (e) => {
					e.target.style.background = "rgba(0,0,0,0.7)";
					e.target.style.transform = "scale(1)";
				},
			},
			parent: container,
		});

		container.addEventListener("mouseenter", () => (btn.style.opacity = "1"));
		container.addEventListener("mouseleave", () => (btn.style.opacity = "0"));
	}
}

// ============================================================================
// CHILD REMOVER
// ============================================================================

class ChildRemover {
	constructor(options = {}) {
		this.selector = options.selector || 'div[data-attrid="images universal"]';
		this.childIndex = options.childIndex ?? 2;
		this.observer = null;
	}

	start() {
		document
			.querySelectorAll(this.selector)
			.forEach((div) => this.process(div));

		this.observer = new MutationObserver((mutations) => {
			for (const mut of mutations) {
				if (mut.type === "childList") {
					mut.addedNodes.forEach((node) => {
						if (node.nodeType === 1) {
							if (node.matches?.(this.selector)) this.process(node);
							node
								.querySelectorAll?.(this.selector)
								.forEach((div) => this.process(div));
						}
					});

					if (mut.target.matches?.(this.selector)) {
						this.process(mut.target);
					}
				}
			}
		});

		this.observer.observe(document.body, { childList: true, subtree: true });
	}

	stop() {
		this.observer?.disconnect();
	}

	process(div) {
		const indices = Array.isArray(this.childIndex)
			? this.childIndex
			: [this.childIndex];
		indices.forEach((i) => div.children[i]?.remove());
	}
}

// ============================================================================
// PANEL BAR
// ============================================================================

const $btn = (text, parent, click, style = {}) => {
	$el("div", {
		text,
		style: {
			color: "#fff",
			width: "28px",
			lineHeight: "2.2",
			height: "100%",
			borderRadius: "5px",
			textAlign: "center",
			cursor: "pointer",
			transition: "all 0.2s",
			...style,
		},
		on: {
			click,
			mouseenter: (e) => {
				e.target.style.background = "rgba(0,102,204,0.9)";
				e.target.style.transform = "scale(1.1)";
			},
			mouseleave: (e) => {
				e.target.style.background = "rgba(0,0,0,0.7)";
				e.target.style.transform = "scale(1)";
			},
		},
		parent,
	});
};
let timeOut;
const countdownTimer = (parent) => {
	if (timeOut) clearTimeout(timeOut);
	timeOut = setTimeout(() => {
		parent.style.right = "-310px";
	}, 3000);
};

// ============================================================================
// INITIALIZATION
// ============================================================================

function init() {
	new ImageDownloadWatcher({
		icon: "📥",
		onDownload: (url) => {
			sendToBackend("GoogleDownloadImage", url);
			Toast.success("Downloading image");
		},
	}).start();

	// Video screenshot
	new VideoScreenshot({
		icon: "📥",
		onScreenshot: async (dataUrl, _video, filename) => {
			try {
				const blob = await (await fetch(dataUrl)).blob();
				await navigator.clipboard.write([
					new ClipboardItem({ "image/png": blob }),
				]);
				Toast.success("Screenshot copied!");
			} catch {
				const a = document.createElement("a");
				a.href = dataUrl;
				a.download = filename;
				a.click();
				Toast.success("Screenshot downloaded!");
			}
		},
	}).start();

	// Child remover
	new ChildRemover({
		selector: 'div[data-attrid="images universal"]',
		childIndex: 2,
	}).start();

	sendToBackend("GoogleInfo", "UI initialized");
}

document.addEventListener("DOMContentLoaded", () => setTimeout(init, 300));
