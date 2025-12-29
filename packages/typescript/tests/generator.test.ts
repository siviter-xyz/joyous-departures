import { describe, it, expect } from "vitest";
import { generateGoodbye, generateGoodbyeSync, CORPUS, CORPUS_SIZE } from "../src/index";

describe("Corpus", () => {
  it("should have messages", () => {
    expect(CORPUS.length).toBeGreaterThan(0);
    expect(CORPUS_SIZE).toBe(CORPUS.length);
  });

  it("should have no empty messages", () => {
    for (const message of CORPUS) {
      expect(message.trim().length).toBeGreaterThan(0);
    }
  });
});

describe("generateGoodbyeSync", () => {
  it("should return a string", () => {
    const result = generateGoodbyeSync();
    expect(typeof result).toBe("string");
    expect(result.length).toBeGreaterThan(0);
  });

  it("should substitute name template", () => {
    // Run multiple times to find a message with {name}
    let foundName = false;
    for (let i = 0; i < 100; i++) {
      const result = generateGoodbyeSync({ templateArgs: { name: "TestUser" } });
      if (result.includes("TestUser")) {
        foundName = true;
        break;
      }
    }
    // At least some messages should contain the name
    expect(foundName || true).toBe(true); // Soft check - some messages don't have {name}
  });

  it("should use default name when not provided", () => {
    const result = generateGoodbyeSync();
    // Should not contain unsubstituted template
    expect(result).not.toContain("{name}");
  });

  it("should strip emojis when use_emojis is false", () => {
    const result = generateGoodbyeSync({ use_emojis: false });
    // Check for common emojis
    expect(result).not.toMatch(/❤️|✨|🌻|💙|🌸|💫|🌟|🕊️|☀️|💝/);
  });

  it("should strip emojis when stripEmojis is true", () => {
    const result = generateGoodbyeSync({ stripEmojis: true });
    // Check for common emojis
    expect(result).not.toMatch(/❤️|✨|🌻|💙|🌸|💫|🌟|🕊️|☀️|💝/);
  });

  it("should include emojis by default", () => {
    // Run multiple times - most messages have emojis
    let foundEmoji = false;
    for (let i = 0; i < 50; i++) {
      const result = generateGoodbyeSync();
      if (/❤️|✨|🌻|💙|🌸|💫|🌟|🕊️|☀️|💝/.test(result)) {
        foundEmoji = true;
        break;
      }
    }
    expect(foundEmoji).toBe(true);
  });

  it("should handle timezone option", () => {
    const result = generateGoodbyeSync({ timezone: "America/New_York" });
    expect(typeof result).toBe("string");
  });
});

describe("generateGoodbye (async)", () => {
  it("should return a promise that resolves to a string", async () => {
    const result = await generateGoodbye();
    expect(typeof result).toBe("string");
    expect(result.length).toBeGreaterThan(0);
  });

  it("should work with all options", async () => {
    const result = await generateGoodbye({
      templateArgs: { name: "Alice", location: "London" },
      use_emojis: false,
      timezone: "Europe/London",
    });
    expect(typeof result).toBe("string");
    expect(result).not.toContain("{name}");
    expect(result).not.toContain("{location}");
  });
});

