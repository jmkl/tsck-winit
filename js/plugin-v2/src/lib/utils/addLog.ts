// ============================================
// Logger Utility - Type-Safe Version
// ============================================

/**
 * CSS class names for log components
 */
const LOG_CLASSES = {
  CONTAINER: "log-entry",
  TAG: "log-tag",
  CONTENT: "log-content",
  TIMESTAMP: "log-timestamp",
} as const;

/**
 * Log levels with associated styling
 */
const LOG_LEVELS = {
  INFO: "info",
  WARN: "warn",
  ERROR: "error",
  DEBUG: "debug",
  SUCCESS: "success",
} as const;

// Type definitions
type LogLevel = (typeof LOG_LEVELS)[keyof typeof LOG_LEVELS];
type LogContent = string | number | boolean | object | null | undefined;

interface LogEntry {
  tag: string;
  content: LogContent;
  level: LogLevel;
  timestamp: string;
}

interface LoggerConfig {
  maxEntries?: number;
  autoScroll?: boolean;
  showTimestamp?: boolean;
  panelId?: string;
}

/**
 * Create a DOM element with class and content
 */
function createElement(
  tag: keyof HTMLElementTagNameMap,
  className: string,
  textContent: string = "",
): HTMLElement {
  const element = document.createElement(tag);
  element.className = className;
  if (textContent) {
    element.textContent = textContent;
  }
  return element;
}

/**
 * Format content for display
 */
function formatContent(content: LogContent): string {
  if (content === null) return "null";
  if (content === undefined) return "undefined";

  if (typeof content === "object") {
    try {
      return JSON.stringify(content, null, 2);
    } catch (error) {
      return String(content);
    }
  }

  return String(content);
}

/**
 * Get timestamp string
 */
function getTimestamp(): string {
  const now = new Date();
  return now.toLocaleTimeString("en-US", {
    hour12: false,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

/**
 * Logger class with configurable behavior
 */
class Logger {
  private config: Required<LoggerConfig>;
  enable: boolean = false;
  constructor(config: LoggerConfig = {}) {
    this.config = {
      maxEntries: config.maxEntries ?? 3,
      autoScroll: config.autoScroll ?? true,
      showTimestamp: config.showTimestamp ?? true,
      panelId: config.panelId ?? "debug-panel",
    };
  }

  /**
   * Add a log entry to the debug panel
   */
  log(
    tag: string,
    content: LogContent,
    level: LogLevel = LOG_LEVELS.INFO,
  ): void {
    const debugPanel = this.getDebugPanel();
    if (!debugPanel) {
      return;
    }
    if (!this.enable) {
      this.clear();
      return;
    }

    const logEntry = this.createLogEntry(tag, content, level);
    debugPanel.appendChild(logEntry);

    if (this.config.autoScroll) {
      this.scrollToBottom(debugPanel);
    }

    this.limitLogEntries(debugPanel);
  }

  /**
   * Create a complete log entry element
   */
  private createLogEntry(
    tag: string,
    content: LogContent,
    level: LogLevel,
  ): HTMLElement {
    const container = createElement("div", `${LOG_CLASSES.CONTAINER} ${level}`);
    const tagElement = createElement("div", LOG_CLASSES.TAG, `[${tag}]`);
    const contentElement = createElement(
      "div",
      LOG_CLASSES.CONTENT,
      formatContent(content),
    );

    if (this.config.showTimestamp) {
      const timestamp = createElement(
        "span",
        LOG_CLASSES.TIMESTAMP,
        getTimestamp(),
      );
      tagElement.appendChild(timestamp);
    }

    container.appendChild(tagElement);
    container.appendChild(contentElement);

    return container;
  }

  /**
   * Get or create debug panel
   */
  private getDebugPanel(): HTMLElement | null {
    let panel = document.getElementById(this.config.panelId);

    if (!panel) {
      panel = document.createElement("div");
      panel.id = this.config.panelId;
      document.body.appendChild(panel);
    }

    return panel;
  }

  /**
   * Scroll debug panel to bottom
   */
  private scrollToBottom(panel: HTMLElement): void {
    panel.scrollTop = panel.scrollHeight;
  }

  /**
   * Limit number of log entries (prevent memory issues)
   */
  private limitLogEntries(panel: HTMLElement): void {
    const entries = panel.querySelectorAll(`.${LOG_CLASSES.CONTAINER}`);

    if (entries.length > this.config.maxEntries) {
      const excess = entries.length - this.config.maxEntries;
      for (let i = 0; i < excess; i++) {
        entries[i].remove();
      }
    }
  }

  /**
   * Clear all log entries
   */
  clear(): void {
    const panel = this.getDebugPanel();
    if (panel) {
      panel.innerHTML = "";
    }
  }
  enableLog(enable: boolean) {
    this.enable = enable;
    this.clear();
  }

  /**
   * Convenience methods for different log levels
   */
  info(tag: string, content: LogContent): void {
    this.log(tag, content, LOG_LEVELS.INFO);
  }

  warn(tag: string, content: LogContent): void {
    this.log(tag, content, LOG_LEVELS.WARN);
  }

  error(tag: string, content: LogContent): void {
    this.log(tag, content, LOG_LEVELS.ERROR);
  }

  debug(tag: string, content: LogContent): void {
    this.log(tag, content, LOG_LEVELS.DEBUG);
  }

  success(tag: string, content: LogContent): void {
    this.log(tag, content, LOG_LEVELS.SUCCESS);
  }

  /**
   * Update logger configuration
   */
  updateConfig(config: Partial<LoggerConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * Get current configuration
   */
  getConfig(): Readonly<Required<LoggerConfig>> {
    return { ...this.config };
  }
}

// ============================================
// Singleton Logger Instance
// ============================================

class LoggerSingleton {
  private static instance: Logger;

  private constructor() {} // Prevent instantiation

  /**
   * Get the singleton logger instance
   */
  static getInstance(): Logger {
    if (!LoggerSingleton.instance) {
      LoggerSingleton.instance = new Logger();
    }
    return LoggerSingleton.instance;
  }

  /**
   * Initialize/reconfigure the logger
   */
  static initialize(config: LoggerConfig): Logger {
    LoggerSingleton.instance = new Logger(config);
    return LoggerSingleton.instance;
  }
}

/**
 * Get the global logger instance
 */
function getLogger(): Logger {
  return LoggerSingleton.getInstance();
}

/**
 * Add a log entry (uses singleton logger)
 */
function addLog(
  tag: string,
  content: LogContent,
  level: LogLevel = LOG_LEVELS.INFO,
): void {
  getLogger().log(tag, content, level);
}

/**
 * Clear all logs (uses singleton logger)
 */
function clearLogs(): void {
  getLogger().clear();
}

/**
 * Convenience object for logging (always uses singleton)
 */
const logger = {
  info: (tag: string, content: LogContent) => getLogger().info(tag, content),
  warn: (tag: string, content: LogContent) => getLogger().warn(tag, content),
  error: (tag: string, content: LogContent) => getLogger().error(tag, content),
  debug: (tag: string, content: LogContent) => getLogger().debug(tag, content),
  success: (tag: string, content: LogContent) =>
    getLogger().success(tag, content),
  clear: () => getLogger().clear(),

  // Get the singleton instance directly
  getInstance: () => getLogger(),

  // Initialize with custom config
  initialize: (config: LoggerConfig) => LoggerSingleton.initialize(config),

  // Create a separate logger instance (not singleton)
  create: (config?: LoggerConfig) => new Logger(config),
};

// ============================================
// Export
// ============================================

export {
  addLog,
  clearLogs,
  getLogger,
  LOG_CLASSES,
  LOG_LEVELS,
  logger,
  Logger,
};

export type { LogContent, LogEntry, LoggerConfig, LogLevel };

// ============================================
// Usage Examples
// ============================================

/*
// ============================================
// SINGLETON USAGE - Same instance everywhere!
// ============================================

// In file A (component.ts)
import { logger } from './logger';
logger.info('Component', 'Initialized');

// In file B (api.ts)
import { logger } from './logger';
logger.error('API', 'Request failed'); // Same logger instance!

// In file C (main.ts)
import { logger } from './logger';
logger.success('App', 'Started'); // Same logger instance!


// ============================================
// CONFIGURE ONCE, USE EVERYWHERE
// ============================================

// In your app initialization (main.ts)
import { logger } from './logger';

logger.initialize({
  maxEntries: 200,
  autoScroll: true,
  showTimestamp: true,
  panelId: 'my-debug-panel'
});

// Everywhere else - just import and use
import { logger } from './logger';
logger.info('Module', 'Data'); // Uses your config!


// ============================================
// GET INSTANCE DIRECTLY
// ============================================

import { getLogger } from './logger';

const loggerInstance = getLogger();
loggerInstance.updateConfig({ maxEntries: 500 });
loggerInstance.info('Direct', 'Access');


// ============================================
// CONVENIENCE METHODS (Recommended)
// ============================================

import { logger } from './logger';

logger.info('WebSocket', 'Connected');
logger.warn('Cache', 'Low memory');
logger.error('DB', 'Connection lost');
logger.debug('State', { user: 'John' });
logger.success('Upload', 'Complete');
logger.clear();


// ============================================
// STANDALONE FUNCTIONS
// ============================================

import { addLog, clearLogs, LOG_LEVELS } from './logger';

addLog('API', 'Request sent');
addLog('Auth', 'Failed', LOG_LEVELS.ERROR);
clearLogs();


// ============================================
// CREATE SEPARATE INSTANCE (Non-singleton)
// ============================================

import { logger } from './logger';

// This creates a NEW instance, not the singleton
const customLogger = logger.create({
  panelId: 'custom-panel',
  maxEntries: 50
});

customLogger.info('Custom', 'Separate instance');
*/

// ============================================
// Suggested CSS
// ============================================

/*
.log-entry {
  display: flex;
  gap: 8px;
  padding: 8px;
  border-bottom: 1px solid #e0e0e0;
  font-family: monospace;
  font-size: 12px;
}

.log-entry.info { background: #f0f8ff; }
.log-entry.warn { background: #fff3cd; }
.log-entry.error { background: #f8d7da; }
.log-entry.debug { background: #e7f3ff; }
.log-entry.success { background: #d4edda; }

.log-tag {
  font-weight: bold;
  color: #333;
  min-width: 100px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.log-timestamp {
  font-size: 10px;
  color: #666;
  font-weight: normal;
}

.log-content {
  flex: 1;
  color: #555;
  white-space: pre-wrap;
  word-break: break-word;
}

#debug-panel {
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid #ccc;
  background: white;
}
*/
