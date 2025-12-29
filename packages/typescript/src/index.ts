/**
 * Joyous Departures - Generate warm, heartfelt sign-off messages
 *
 * This is a pure TypeScript implementation with no WASM dependencies.
 * Works in all JavaScript/TypeScript environments including Cloudflare Workers.
 *
 * @packageDocumentation
 */

import { CORPUS } from "./corpus.generated.js";

// Cache corpus length for faster random selection
const CORPUS_LENGTH = CORPUS.length;

/**
 * Template variable values for message customization.
 */
export interface TemplateArgs {
  /** Recipient's name (max 50 chars). Default: "Good Soul" */
  name?: string;
  /** Location context (max 100 chars). Default: "The World" */
  location?: string;
  /** Date string, format: YYYY-MM-DD. Default: current date */
  date?: string;
  /** Time string, format: HH:MM. Default: current time */
  time?: string;
}

/**
 * Configuration options for message generation.
 */
export interface GoodbyeOptions {
  /** Language code (reserved for future use) */
  language_code?: string;
  /** Template variable values */
  templateArgs?: TemplateArgs;
  /** Include emojis in output. Default: true (v1.x compatibility) */
  use_emojis?: boolean;
  /** Remove emoji characters from output. Default: false */
  stripEmojis?: boolean;
  /** IANA timezone for date/time formatting. Default: "Europe/London" */
  timezone?: string;
  /** Optional async translator callback for custom translation */
  translator?: (language_code: string, message: string) => Promise<string>;
}

// Default values for template variables
const DEFAULTS = {
  name: "Good Soul",
  location: "The World",
} as const;

// Validation limits
const LIMITS = {
  name: 50,
  location: 100,
  date: 20,
  time: 10,
} as const;

/**
 * Emoji regex pattern for stripping
 * Covers most common emoji ranges
 */
const EMOJI_REGEX =
  /[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]|[\u{FE00}-\u{FE0F}]|[\u{1F900}-\u{1F9FF}]|[\u{1FA00}-\u{1FA6F}]|[\u{1FA70}-\u{1FAFF}]|[\u{231A}-\u{231B}]|[\u{23E9}-\u{23F3}]|[\u{23F8}-\u{23FA}]|[\u{25AA}-\u{25AB}]|[\u{25B6}]|[\u{25C0}]|[\u{25FB}-\u{25FE}]|[\u{2614}-\u{2615}]|[\u{2648}-\u{2653}]|[\u{267F}]|[\u{2693}]|[\u{26A1}]|[\u{26AA}-\u{26AB}]|[\u{26BD}-\u{26BE}]|[\u{26C4}-\u{26C5}]|[\u{26CE}]|[\u{26D4}]|[\u{26EA}]|[\u{26F2}-\u{26F3}]|[\u{26F5}]|[\u{26FA}]|[\u{26FD}]|[\u{2702}]|[\u{2705}]|[\u{2708}-\u{270D}]|[\u{270F}]|[\u{2712}]|[\u{2714}]|[\u{2716}]|[\u{271D}]|[\u{2721}]|[\u{2728}]|[\u{2733}-\u{2734}]|[\u{2744}]|[\u{2747}]|[\u{274C}]|[\u{274E}]|[\u{2753}-\u{2755}]|[\u{2757}]|[\u{2763}-\u{2764}]|[\u{2795}-\u{2797}]|[\u{27A1}]|[\u{27B0}]|[\u{27BF}]|[\u{2934}-\u{2935}]|[\u{2B05}-\u{2B07}]|[\u{2B1B}-\u{2B1C}]|[\u{2B50}]|[\u{2B55}]|[\u{3030}]|[\u{303D}]|[\u{3297}]|[\u{3299}]|[\u{200D}]|[\u{20E3}]|[\u{FE0F}]/gu;

/**
 * Truncate string to max length
 */
function truncate(value: string | undefined, maxLength: number): string | undefined {
  if (!value) return value;
  return value.length > maxLength ? value.substring(0, maxLength) : value;
}

/**
 * Get current date in YYYY-MM-DD format for the specified timezone
 * Reuses a single Date object for efficiency
 */
function getCurrentDate(timezone: string, now?: Date): string {
  const date = now ?? new Date();
  try {
    return date.toLocaleDateString("en-CA", { timeZone: timezone }); // en-CA uses YYYY-MM-DD
  } catch {
    // Fallback if timezone is invalid
    return date.toISOString().split("T")[0];
  }
}

/**
 * Get current time in HH:MM format for the specified timezone
 * Reuses a single Date object for efficiency
 */
function getCurrentTime(timezone: string, now?: Date): string {
  const date = now ?? new Date();
  try {
    return date.toLocaleTimeString("en-GB", {
      timeZone: timezone,
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    });
  } catch {
    // Fallback if timezone is invalid
    return `${date.getHours().toString().padStart(2, "0")}:${date.getMinutes().toString().padStart(2, "0")}`;
  }
}

/**
 * Substitute template variables in a message
 * Optimized single-pass replacement for better performance
 */
function substituteTemplates(
  message: string,
  templateArgs: TemplateArgs | undefined,
  timezone: string
): string {
  // Check if date/time are needed before computing
  const needsDate = message.includes("{date}");
  const needsTime = message.includes("{time}");
  
  // Reuse single Date object if both date and time are needed
  const now = (needsDate || needsTime) ? new Date() : undefined;

  // Get values with defaults and validation (lazy evaluation)
  const name = truncate(templateArgs?.name, LIMITS.name) ?? DEFAULTS.name;
  const location = truncate(templateArgs?.location, LIMITS.location) ?? DEFAULTS.location;
  const date = truncate(templateArgs?.date, LIMITS.date) ?? (needsDate ? getCurrentDate(timezone, now) : "");
  const time = truncate(templateArgs?.time, LIMITS.time) ?? (needsTime ? getCurrentTime(timezone, now) : "");

  // Single-pass replacement using a map for better performance
  return message
    .replace(/\{name\}/g, name)
    .replace(/\{location\}/g, location)
    .replace(/\{date\}/g, date)
    .replace(/\{time\}/g, time);
}

/**
 * Strip emojis from a string
 */
function stripEmojisFromString(text: string): string {
  return text.replace(EMOJI_REGEX, "").replace(/\s+/g, " ").trim();
}

/**
 * Generate a warm, heartfelt goodbye message (synchronous).
 *
 * This is the core function that performs message generation without any async operations.
 * Use this when you don't need the translator callback.
 *
 * @param options - Optional configuration options
 * @returns The generated message
 *
 * @example
 * ```typescript
 * import { generateGoodbyeSync } from '@siviter-xyz/joyous-departures';
 *
 * const message = generateGoodbyeSync({ templateArgs: { name: 'Alice' } });
 * console.log(message); // "Wishing you a joyous day, Alice ❤️"
 * ```
 */
export function generateGoodbyeSync(options: GoodbyeOptions = {}): string {
  // Random message selection (using cached length)
  const message = CORPUS[Math.floor(Math.random() * CORPUS_LENGTH)];

  // Get timezone
  const timezone = options.timezone ?? "Europe/London";

  // Template substitution
  let result = substituteTemplates(message, options.templateArgs, timezone);

  // Emoji handling: support both use_emojis (v1.x) and stripEmojis (v2.x)
  // use_emojis=false OR stripEmojis=true → strip emojis
  const shouldStripEmojis = options.stripEmojis === true || options.use_emojis === false;
  if (shouldStripEmojis) {
    result = stripEmojisFromString(result);
  }

  return result;
}

/**
 * Generate a warm, heartfelt goodbye message (async).
 *
 * This async wrapper maintains backward compatibility with v1.x API.
 * The core generation is synchronous; async is only used for the optional translator.
 *
 * @param options - Optional configuration options
 * @returns Promise resolving to the generated message
 *
 * @example
 * ```typescript
 * import { generateGoodbye } from '@siviter-xyz/joyous-departures';
 *
 * // Basic usage (async for compatibility)
 * const message = await generateGoodbye({ templateArgs: { name: 'Alice' } });
 * console.log(message); // "Wishing you a joyous day, Alice ❤️"
 *
 * // With translator
 * const translated = await generateGoodbye({
 *   templateArgs: { name: 'Alice' },
 *   translator: async (lang, msg) => translateToFrench(msg),
 * });
 * ```
 */
export async function generateGoodbye(options: GoodbyeOptions = {}): Promise<string> {
  // Generate message synchronously
  let result = generateGoodbyeSync(options);

  // Apply translator if provided
  if (options.translator) {
    const languageCode = options.language_code ?? "en-GB";
    if (languageCode !== "en-GB") {
      try {
        result = await options.translator(languageCode, result);
      } catch {
        // Fallback to original message if translation fails
        console.warn("Translation failed, using original message");
      }
    }
  }

  return result;
}

// Re-export corpus for advanced usage
export { CORPUS, CORPUS_SIZE } from "./corpus.generated.js";

// Default export for convenience
export default generateGoodbye;

