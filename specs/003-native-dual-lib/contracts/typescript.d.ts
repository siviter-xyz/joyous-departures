/**
 * API Contract: @siviter-xyz/joyous-departures v2.0
 * 
 * This file defines the public TypeScript API for the library.
 * Implementation must conform to these type signatures.
 */

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
  /** Template variable values */
  templateArgs?: TemplateArgs;
  /** Remove emoji characters from output. Default: false */
  stripEmojis?: boolean;
  /** IANA timezone for date/time formatting. Default: "Europe/London" */
  timezone?: string;
  /** Optional async translator callback for custom translation */
  translator?: (message: string) => Promise<string>;
}

/**
 * Generate a warm, heartfelt goodbye message.
 * 
 * @param options - Optional configuration options
 * @returns The generated message (synchronous)
 * 
 * @example
 * ```typescript
 * import { generateGoodbye } from '@siviter-xyz/joyous-departures';
 * 
 * const message = generateGoodbye({ templateArgs: { name: 'Alice' } });
 * console.log(message); // "Wishing you a joyous day, Alice ❤️"
 * ```
 * 
 * @example
 * ```typescript
 * // With emoji stripping
 * const message = generateGoodbye({ stripEmojis: true });
 * console.log(message); // "Wishing you a joyous day, Good Soul"
 * ```
 * 
 * @example
 * ```typescript
 * // With translator
 * const message = await generateGoodbyeAsync({
 *   templateArgs: { name: 'Alice' },
 *   translator: async (msg) => translateToFrench(msg),
 * });
 * ```
 */
export function generateGoodbye(options?: GoodbyeOptions): string;

/**
 * Generate a warm, heartfelt goodbye message (async version).
 * 
 * Use this when you need to use the translator callback.
 * 
 * @param options - Optional configuration options
 * @returns Promise resolving to the generated message
 */
export function generateGoodbyeAsync(options?: GoodbyeOptions): Promise<string>;

/**
 * The embedded message corpus (readonly).
 * 
 * This is code-generated from corpus/en-GB.txt at build time.
 * Do not modify directly.
 */
export declare const CORPUS: readonly string[];

/**
 * Type representing any valid message from the corpus.
 */
export type Message = typeof CORPUS[number];

