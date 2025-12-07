// @ts-ignore - WASM module types
import init, {
  generate_goodbye as wasm_generate_goodbye,
} from "../pkg/joyous_departures_wasm.js";
import { fileURLToPath } from "url";
import { dirname, join } from "path";
import { readFileSync } from "fs";

// Get __dirname equivalent for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Initialize WASM module (call once)
let wasmInitialized = false;

async function ensureWasmInitialized() {
  if (!wasmInitialized) {
    // For Node.js, load WASM file directly
    if (typeof process !== "undefined" && process.versions?.node) {
      const wasmPath = join(__dirname, "../pkg/joyous_departures_wasm_bg.wasm");
      const wasmBuffer = readFileSync(wasmPath);
      await init({ module_or_path: wasmBuffer });
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

  // Truncate name to 50 characters if provided
  if (options.templateArgs?.name && options.templateArgs.name.length > 50) {
    options.templateArgs.name = options.templateArgs.name.substring(0, 50);
  }

  try {
    // Call WASM function
    let result = wasm_generate_goodbye(
      options.language_code || "en-GB",
      JSON.stringify(options.templateArgs || {}),
      options.use_emojis !== false, // default true
      options.timezone || "Europe/London",
    ) as string;

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
