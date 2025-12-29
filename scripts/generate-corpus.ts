#!/usr/bin/env npx tsx
/**
 * Corpus Code Generator
 *
 * Reads corpus text files and generates native language constants for both
 * TypeScript and Python. This eliminates runtime file I/O and ensures
 * maximum portability across all environments.
 *
 * Usage: npx tsx scripts/generate-corpus.ts
 */

import { readFileSync, writeFileSync, mkdirSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, "..");

interface CorpusStats {
  totalLines: number;
  comments: number;
  emptyLines: number;
  duplicates: number;
  uniqueMessages: number;
}

function parseCorpus(content: string): { messages: string[]; stats: CorpusStats } {
  const lines = content.split("\n");
  const stats: CorpusStats = {
    totalLines: lines.length,
    comments: 0,
    emptyLines: 0,
    duplicates: 0,
    uniqueMessages: 0,
  };

  const seen = new Set<string>();
  const messages: string[] = [];

  for (const line of lines) {
    const trimmed = line.trim();

    if (!trimmed) {
      stats.emptyLines++;
      continue;
    }

    if (trimmed.startsWith("#")) {
      stats.comments++;
      continue;
    }

    if (seen.has(trimmed)) {
      stats.duplicates++;
      continue;
    }

    seen.add(trimmed);
    messages.push(trimmed);
  }

  stats.uniqueMessages = messages.length;
  return { messages, stats };
}

function generateTypeScript(messages: string[]): string {
  const timestamp = new Date().toISOString();
  const escaped = messages.map((m) => JSON.stringify(m));

  return `// Auto-generated from corpus/en-GB.txt - DO NOT EDIT
// Generated: ${timestamp}
// Message count: ${messages.length}

/**
 * Immutable corpus of goodbye messages.
 * Each message may contain template variables: {name}, {location}, {date}, {time}
 */
export const CORPUS = [
${escaped.map((m) => `  ${m},`).join("\n")}
] as const;

/**
 * Type representing any valid message from the corpus.
 */
export type Message = (typeof CORPUS)[number];

/**
 * Number of unique messages in the corpus.
 */
export const CORPUS_SIZE = ${messages.length} as const;
`;
}

function generatePython(messages: string[]): string {
  const timestamp = new Date().toISOString();
  const escaped = messages.map((m) => JSON.stringify(m));

  return `# Auto-generated from corpus/en-GB.txt - DO NOT EDIT
# Generated: ${timestamp}
# Message count: ${messages.length}

"""
Immutable corpus of goodbye messages.
Each message may contain template variables: {name}, {location}, {date}, {time}
"""

CORPUS: tuple[str, ...] = (
${escaped.map((m) => `    ${m},`).join("\n")}
)

CORPUS_SIZE: int = ${messages.length}
`;
}

function ensureDir(filePath: string): void {
  const dir = dirname(filePath);
  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
}

// Main execution
function main(): void {
  console.log("🎯 Corpus Code Generator\n");

  // Read corpus
  const corpusPath = join(ROOT, "corpus", "en-GB.txt");
  console.log(`📖 Reading: ${corpusPath}`);

  if (!existsSync(corpusPath)) {
    console.error(`❌ Corpus file not found: ${corpusPath}`);
    process.exit(1);
  }

  const content = readFileSync(corpusPath, "utf-8");
  const { messages, stats } = parseCorpus(content);

  console.log(`\n📊 Corpus Statistics:`);
  console.log(`   Total lines: ${stats.totalLines}`);
  console.log(`   Comments: ${stats.comments}`);
  console.log(`   Empty lines: ${stats.emptyLines}`);
  console.log(`   Duplicates removed: ${stats.duplicates}`);
  console.log(`   Unique messages: ${stats.uniqueMessages}`);

  if (messages.length === 0) {
    console.error("❌ No messages found in corpus!");
    process.exit(1);
  }

  // Generate TypeScript
  const tsOutput = generateTypeScript(messages);
  const tsPath = join(ROOT, "packages", "typescript", "src", "corpus.generated.ts");
  ensureDir(tsPath);
  writeFileSync(tsPath, tsOutput);
  console.log(`\n✅ Generated: ${tsPath}`);

  // Generate Python
  const pyOutput = generatePython(messages);
  const pyPath = join(ROOT, "packages", "python", "src", "joyous_departures", "corpus.py");
  ensureDir(pyPath);
  writeFileSync(pyPath, pyOutput);
  console.log(`✅ Generated: ${pyPath}`);

  console.log(`\n🎉 Done! Generated ${messages.length} messages for both TypeScript and Python.`);
}

main();

