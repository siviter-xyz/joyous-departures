// Import WASM module with proper types
// wasm-pack automatically generates TypeScript definitions (.d.ts files)
// The types are available at build time from the generated package
import type { InitInput } from "../pkg/joy_generator_wasm.js";
import init, {
  generate_goodbye as wasm_generate_goodbye,
} from "../pkg/joy_generator_wasm.js";
import { fileURLToPath } from "url";
import { dirname, join } from "path";
import { readFileSync } from "fs";

// Get __dirname equivalent for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Initialize WASM module (call once)
let wasmInitialized = false;

/**
 * Initialize WASM module (call once)
 * Handles both Node.js (direct file read) and browser (fetch) environments
 */
async function ensureWasmInitialized(): Promise<void> {
  if (!wasmInitialized) {
    // For Node.js, load WASM file directly
    if (typeof process !== "undefined" && process.versions?.node) {
      const wasmPath = join(__dirname, "../pkg/joy_generator_wasm_bg.wasm");
      const wasmBuffer = readFileSync(wasmPath);
      await init({ module_or_path: wasmBuffer } as InitInput);
    } else {
      // For browser, use default init (with fetch)
      await init();
    }
    wasmInitialized = true;
  }
}

export interface GoodbyeTemplateArgs {
  name?: string;
  location?: string;
  date?: string;
  time?: string;
  [key: string]: string | undefined;
}

export interface GoodbyeOptions {
  language_code?: string;
  templateArgs?: GoodbyeTemplateArgs;
  use_emojis?: boolean;
  timezone?: string;
  translator?: (language_code: string, message: string) => Promise<string>;
}

/**
 * Generate a warm, heartfelt goodbye message
 *
 * @param options - Optional configuration options
 * @returns Promise<string> - Generated goodbye message
 *
 * @example
 * ```typescript
 * const message = await generateGoodbye({ templateArgs: { name: "Alice" } });
 * console.log(message); // "Wishing you a joyous day, Alice ❤️"
 * ```
 */
export async function generateGoodbye(
  options: GoodbyeOptions = {},
): Promise<string> {
  await ensureWasmInitialized();

  // Validate and prepare options
  const languageCode = options.language_code || "en-GB";

  // Validate language code format (ISO 639-1 with optional region)
  if (!isValidLanguageCode(languageCode)) {
    console.warn(
      `Invalid language code: ${languageCode}, falling back to en-GB`,
    );
    options.language_code = "en-GB";
  }

  // Validate timezone (IANA format)
  const timezone = options.timezone || "Europe/London";
  if (!isValidTimezone(timezone)) {
    console.warn(
      `Invalid timezone: ${timezone}, falling back to Europe/London`,
    );
    options.timezone = "Europe/London";
  }

  // Validate and truncate template args to prevent excessive input
  if (options.templateArgs) {
    if (options.templateArgs.name && options.templateArgs.name.length > 50) {
      options.templateArgs.name = options.templateArgs.name.substring(0, 50);
    }
    if (options.templateArgs.location && options.templateArgs.location.length > 100) {
      options.templateArgs.location = options.templateArgs.location.substring(0, 100);
    }
    if (options.templateArgs.date && options.templateArgs.date.length > 20) {
      options.templateArgs.date = options.templateArgs.date.substring(0, 20);
    }
    if (options.templateArgs.time && options.templateArgs.time.length > 10) {
      options.templateArgs.time = options.templateArgs.time.substring(0, 10);
    }
  }

  try {
    // Call WASM function (returns string per wasm-bindgen signature)
    let result: string = wasm_generate_goodbye(
      options.language_code || "en-GB",
      JSON.stringify(options.templateArgs || {}),
      options.use_emojis !== false, // default true
      options.timezone || "Europe/London",
    );

    // If translator callback provided and language is not en-GB, translate
    if (options.translator && languageCode !== "en-GB") {
      try {
        result = await options.translator(languageCode, result);
      } catch (error) {
        // Fallback to original message if translation fails
        console.warn("Translation failed, using original message:", error);
      }
    }

    return result;
  } catch (error) {
    // Handle errors with fallbacks
    if (error instanceof Error) {
      if (error.message.includes("CorpusLoadError")) {
        // Return fallback message
        const name = options.templateArgs?.name || "Good Soul";
        return `Wishing you a joyous day, ${name} ❤️`;
      }
      if (error.message.includes("TemplateVariableError")) {
        return "Wishing you a joyous day❤️";
      }
    }
    throw error;
  }
}

/**
 * Validate ISO 639-1 language code format
 */
function isValidLanguageCode(code: string): boolean {
  const parts = code.split("-");
  if (parts.length === 0 || parts.length > 2) return false;
  return (
    parts[0].length === 2 &&
    parts[0].split("").every((c) => /[a-z]/.test(c)) &&
    (parts.length === 1 ||
      (parts[1].length === 2 &&
        parts[1].split("").every((c) => /[A-Z]/.test(c))))
  );
}

/**
 * Validate IANA timezone identifier
 */
function isValidTimezone(tz: string): boolean {
  // Basic validation: should contain a slash
  // More thorough validation would be done in Rust core
  return tz.includes("/") && tz.length > 0;
}
