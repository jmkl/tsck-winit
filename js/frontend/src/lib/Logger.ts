type LogLevel = "info" | "warn" | "error" | "debug";

class Logger {
  private static formatTag(tag?: string): string {
    return tag ? `%c[${tag}]` : "";
  }

  private static styleTag(level: LogLevel): string {
    switch (level) {
      case "info":
        return "color: #0ea5e9; font-weight: bold"; // sky-500
      case "warn":
        return "color: #facc15; font-weight: bold"; // yellow-400
      case "error":
        return "color: #ef4444; font-weight: bold"; // red-500
      case "debug":
        return "color: #a855f7; font-weight: bold"; // purple-500
    }
  }

  static log(...args: any[]): void {
    Logger.print("info", ...args);
  }

  static warn(...args: any[]): void {
    Logger.print("warn", ...args);
  }

  static error(...args: any[]): void {
    Logger.print("error", ...args);
  }

  static debug(...args: any[]): void {
    Logger.print("debug", ...args);
  }

  private static print(level: LogLevel, ...args: any[]) {
    const firstArg = args[0];
    let tag: string | undefined;
    let rest = args;

    // If first argument is a string and not an object, use as tag
    if (typeof firstArg === "string" && args.length > 1) {
      tag = firstArg;
      rest = args.slice(1);
    }

    const prefix = Logger.formatTag(tag);
    const style = Logger.styleTag(level);

    if (tag) {
      console[level](`${prefix}`, style, ...rest);
    } else {
      console[level](...rest);
    }
  }

  // ─────────────────────────────────────────────
  // Benchmark Helper
  // ─────────────────────────────────────────────
  static benchmark(label = "Benchmark") {
    let startTime: number | null = null;

    return {
      start() {
        startTime = performance.now();
        Logger.debug(label, "started...");
        return this;
      },
      end() {
        if (startTime === null) {
          Logger.warn(label, "not started yet");
          return 0;
        }
        const duration = performance.now() - startTime;
        Logger.log(label, `took ${duration.toFixed(2)} ms`);
        startTime = null;
        return duration;
      },
    };
  }
}

export default Logger;
