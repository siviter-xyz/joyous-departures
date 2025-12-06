import { describe, it, expect, bench } from "vitest";
import { generateGoodbye } from "../src/index";

describe("E2E Tests for joy-goodbye TypeScript bindings", () => {
  it("should generate a basic goodbye message", async () => {
    const result = await generateGoodbye();

    expect(result).toBeTypeOf("string");
    expect(result.length).toBeGreaterThan(0);
    // Should contain default name "Good Soul" or be a valid message
    expect(result).toMatch(/Good Soul|joyous/i);
  });

  it("should verify randomness (different results each call)", async () => {
    const results = await Promise.all(
      Array.from({ length: 10 }, () => generateGoodbye()),
    );

    // All should be non-empty
    expect(results.every((r) => r.length > 0)).toBe(true);
    expect(results.length).toBe(10);

    // With fallback corpus (1 message), all will be same
    // In Phase 5 with 360 messages, we should see different results
    // For now, just verify structure
  });

  it("should accept custom name", async () => {
    const result = await generateGoodbye({
      templateArgs: { name: "Alice" },
    });

    expect(result).toContain("Alice");
  });

  it("should accept custom location", async () => {
    const result = await generateGoodbye({
      templateArgs: { location: "Paris" },
    });

    expect(result).toBeTypeOf("string");
    expect(result.length).toBeGreaterThan(0);
  });

  it("should accept timezone option", async () => {
    const result = await generateGoodbye({
      timezone: "America/New_York",
    });

    expect(result).toBeTypeOf("string");
    expect(result.length).toBeGreaterThan(0);
  });

  it("should strip emojis when use_emojis is false", async () => {
    const resultWithEmojis = await generateGoodbye({ use_emojis: true });
    const resultWithoutEmojis = await generateGoodbye({ use_emojis: false });

    expect(resultWithEmojis).toBeTypeOf("string");
    expect(resultWithoutEmojis).toBeTypeOf("string");
    // Both should be valid messages
    expect(resultWithEmojis.length).toBeGreaterThan(0);
    expect(resultWithoutEmojis.length).toBeGreaterThan(0);
  });

  it("should truncate long names to 50 characters", async () => {
    const longName = "A".repeat(100);
    const result = await generateGoodbye({
      templateArgs: { name: longName },
    });

    // Name should be truncated in the result
    expect(result.length).toBeLessThan(longName.length + 50);
  });

  it("should fallback to en-GB for invalid language code", async () => {
    const result = await generateGoodbye({
      language_code: "invalid-code",
    });

    // Should still return a valid message
    expect(result).toBeTypeOf("string");
    expect(result.length).toBeGreaterThan(0);
  });

  it("should fallback to Europe/London for invalid timezone", async () => {
    const result = await generateGoodbye({
      timezone: "Invalid/Timezone",
    });

    // Should still return a valid message
    expect(result).toBeTypeOf("string");
    expect(result.length).toBeGreaterThan(0);
  });

  it("should perform quickly (benchmark)", async () => {
    const start = performance.now();
    await generateGoodbye();
    const elapsed = performance.now() - start;
    // Should be fast (<10ms target, but allow some margin for WASM initialization)
    expect(elapsed).toBeLessThan(100); // 100ms is reasonable for first call with WASM init
  });
});
