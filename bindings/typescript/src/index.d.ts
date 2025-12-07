/**
 * Template arguments for customizing message variables
 */
export interface GoodbyeTemplateArgs {
  /** Name to use in message (default: "Good Soul") */
  name?: string;
  /** Location to use in message (default: "The World") */
  location?: string;
  /** Date to use in message (default: current date) */
  date?: string;
  /** Time to use in message (default: current time) */
  time?: string;
  /** Additional template variables */
  [key: string]: string | undefined;
}

/**
 * Options for generating goodbye messages
 */
export interface GoodbyeOptions {
  /** ISO 639-1 language code with optional region (default: "en-GB") */
  language_code?: string;
  /** Template variable arguments */
  templateArgs?: GoodbyeTemplateArgs;
  /** Whether to include emojis (default: true) */
  use_emojis?: boolean;
  /** IANA timezone identifier (default: "Europe/London") */
  timezone?: string;
  /** Optional async translator callback for custom language translation */
  translator?: (language_code: string, message: string) => Promise<string>;
}

/**
 * Generate a warm, heartfelt goodbye message
 *
 * @param options - Optional configuration options
 * @returns Promise that resolves to the generated goodbye message
 *
 * @example
 * ```typescript
 * const message = await generateGoodbye({ templateArgs: { name: "Alice" } });
 * console.log(message); // "Wishing you a joyous day, Alice ❤️"
 * ```
 */
export function generateGoodbye(options?: GoodbyeOptions): Promise<string>;

