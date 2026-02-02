// ============================================
// Sandbox Code Execution Utility
// ============================================

/**
 * Result of sandbox execution
 */
interface SandboxResult<T = any> {
  success: boolean;
  result?: T;
  error?: {
    message: string;
    name: string;
    stack?: string;
  };
  executionTime: number;
}

/**
 * Sandbox execution options
 */
interface SandboxOptions {
  timeout?: number; // milliseconds
  allowAsync?: boolean;
  context?: Record<string, any>; // Variables to inject into sandbox
}

/**
 * Execute code in a sandbox and catch errors
 * @param code - Code to execute (string or function)
 * @param options - Sandbox options
 * @returns SandboxResult
 */
function sandbox<T = any>(
  code: string | (() => T) | (() => Promise<T>),
  options: SandboxOptions = {},
): Promise<SandboxResult<T>> {
  const { timeout = 5000, allowAsync = false, context = {} } = options;

  return new Promise((resolve) => {
    const startTime = performance.now();

    // Handle timeout
    const timeoutId = setTimeout(() => {
      resolve({
        success: false,
        error: {
          name: "TimeoutError",
          message: `Execution exceeded ${timeout}ms timeout`,
        },
        executionTime: performance.now() - startTime,
      });
    }, timeout);

    // Execute code
    (async () => {
      try {
        let result: T;

        if (typeof code === "function") {
          // Execute function
          result = await code();
        } else {
          // Execute string code
          result = executeStringCode<T>(code, context, allowAsync);
        }

        clearTimeout(timeoutId);
        resolve({
          success: true,
          result,
          executionTime: performance.now() - startTime,
        });
      } catch (error) {
        clearTimeout(timeoutId);
        resolve({
          success: false,
          error: {
            name: error instanceof Error ? error.name : "Error",
            message: error instanceof Error ? error.message : String(error),
            stack: error instanceof Error ? error.stack : undefined,
          },
          executionTime: performance.now() - startTime,
        });
      }
    })();
  });
}

/**
 * Execute string code with context
 */
function executeStringCode<T>(
  code: string,
  context: Record<string, any>,
  allowAsync: boolean,
): T {
  // Create context variables
  const contextKeys = Object.keys(context);
  const contextValues = Object.values(context);

  // Create function with context
  const func = allowAsync
    ? new Function(...contextKeys, `return (async () => { ${code} })()`)
    : new Function(...contextKeys, `return (() => { ${code} })()`);

  // Execute with context
  return func(...contextValues);
}

/**
 * Synchronous version - execute and catch immediately
 */
function sandboxSync<T = any>(
  code: string | (() => T),
  options: Omit<SandboxOptions, "allowAsync"> = {},
): SandboxResult<T> {
  const { timeout = 5000, context = {} } = options;
  const startTime = performance.now();

  try {
    let result: T;

    if (typeof code === "function") {
      result = code();
    } else {
      result = executeStringCode<T>(code, context, false);
    }

    const executionTime = performance.now() - startTime;

    if (executionTime > timeout) {
      return {
        success: false,
        error: {
          name: "TimeoutError",
          message: `Execution exceeded ${timeout}ms timeout`,
        },
        executionTime,
      };
    }

    return {
      success: true,
      result,
      executionTime,
    };
  } catch (error) {
    return {
      success: false,
      error: {
        name: error instanceof Error ? error.name : "Error",
        message: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
      },
      executionTime: performance.now() - startTime,
    };
  }
}

/**
 * Safe evaluation - returns default value on error
 */
function safeEval<T>(
  code: string | (() => T),
  defaultValue: T,
  context?: Record<string, any>,
): T {
  const result = sandboxSync<T>(code, { context });
  return result.success ? result.result! : defaultValue;
}

/**
 * Try-catch wrapper with typed error handling
 */
function tryCatch<T, E = Error>(
  fn: () => T,
  onError?: (error: E) => T,
): T | undefined {
  try {
    return fn();
  } catch (error) {
    if (onError) {
      return onError(error as E);
    }
    return undefined;
  }
}

/**
 * Async try-catch wrapper
 */
async function tryCatchAsync<T, E = Error>(
  fn: () => Promise<T>,
  onError?: (error: E) => Promise<T> | T,
): Promise<T | undefined> {
  try {
    return await fn();
  } catch (error) {
    if (onError) {
      return await onError(error as E);
    }
    return undefined;
  }
}

// ============================================
// Export
// ============================================

export { safeEval, sandbox, sandboxSync, tryCatch, tryCatchAsync };

export type { SandboxOptions, SandboxResult };

// ============================================
// Usage Examples
// ============================================

/*
// ============================================
// Example 1: Execute string code
// ============================================

const result1 = await sandbox('return 1 + 1');
if (result1.success) {
  console.log('Result:', result1.result); // 2
  console.log('Execution time:', result1.executionTime, 'ms');
} else {
  console.error('Error:', result1.error?.message);
}

// ============================================
// Example 2: Execute with context
// ============================================

const result2 = await sandbox(
  'return x + y',
  { context: { x: 10, y: 20 } }
);
console.log(result2.result); // 30

// ============================================
// Example 3: Execute function
// ============================================

const result3 = await sandbox(() => {
  const sum = 5 + 5;
  if (sum !== 10) throw new Error('Math is broken!');
  return sum;
});

if (result3.success) {
  console.log('Sum:', result3.result);
} else {
  console.error('Error:', result3.error?.message);
}

// ============================================
// Example 4: Catch errors
// ============================================

const result4 = await sandbox(() => {
  throw new Error('Something went wrong!');
});

console.log(result4.success); // false
console.log(result4.error?.message); // 'Something went wrong!'
console.log(result4.error?.stack); // Stack trace

// ============================================
// Example 5: Async code
// ============================================

const result5 = await sandbox(
  async () => {
    await new Promise(resolve => setTimeout(resolve, 100));
    return 'Async result!';
  },
  { allowAsync: true }
);

console.log(result5.result); // 'Async result!'

// ============================================
// Example 6: Timeout
// ============================================

const result6 = await sandbox(
  () => {
    while (true) {} // Infinite loop
    return 'Never reached';
  },
  { timeout: 1000 }
);

console.log(result6.success); // false
console.log(result6.error?.name); // 'TimeoutError'

// ============================================
// Example 7: Synchronous execution
// ============================================

const result7 = sandboxSync(() => {
  return Math.random() * 100;
});

if (result7.success) {
  console.log('Random number:', result7.result);
}

// ============================================
// Example 8: Safe eval with default
// ============================================

const result8 = safeEval(
  'return JSON.parse(invalidJSON)',
  { fallback: true }
);
console.log(result8); // { fallback: true }

// ============================================
// Example 9: Try-catch wrapper
// ============================================

const result9 = tryCatch(
  () => JSON.parse('invalid json'),
  (error) => {
    console.error('Parse failed:', error);
    return { error: true };
  }
);

console.log(result9); // { error: true }

// ============================================
// Example 10: Async try-catch
// ============================================

const result10 = await tryCatchAsync(
  async () => {
    const response = await fetch('/api/data');
    return response.json();
  },
  async (error) => {
    console.error('Fetch failed:', error);
    return { error: 'Failed to fetch' };
  }
);

console.log(result10);

// ============================================
// Example 11: Dynamic code execution
// ============================================

const userCode = `
  const numbers = [1, 2, 3, 4, 5];
  const sum = numbers.reduce((a, b) => a + b, 0);
  return sum;
`;

const result11 = await sandbox(userCode);
console.log(result11.result); // 15

// ============================================
// Example 12: With complex context
// ============================================

interface User {
  name: string;
  age: number;
}

const user: User = { name: 'John', age: 30 };

const result12 = await sandbox(
  'return `${user.name} is ${user.age} years old`',
  { context: { user } }
);

console.log(result12.result); // 'John is 30 years old'
*/
