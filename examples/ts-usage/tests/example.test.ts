import { describe, it, expect } from "vitest";
import { generateGoodbye } from "@siviter-xyz/joyous-departures";

describe("TypeScript Example Tests", () => {
  it("should generate a basic message", async () => {
    const message = await generateGoodbye();
    expect(message).toBeTypeOf("string");
    expect(message.length).toBeGreaterThan(0);
    expect(message.trim().length).toBeGreaterThan(0);
  });

  it("should generate message with custom name", async () => {
    const message = await generateGoodbye({
      templateArgs: { name: "Alice" },
    });
    expect(message).toBeTypeOf("string");
    expect(message.length).toBeGreaterThan(0);
    // Name should appear in message (case-insensitive)
    expect(message.toLowerCase()).toContain("alice");
  });

  it("should generate message with custom name and location", async () => {
    const message = await generateGoodbye({
      templateArgs: { name: "Bob", location: "London" },
    });
    expect(message).toBeTypeOf("string");
    expect(message.length).toBeGreaterThan(0);
    expect(message.toLowerCase()).toContain("bob");
  });

  it("should generate message without emojis", async () => {
    const message = await generateGoodbye({
      templateArgs: { name: "Charlie" },
      use_emojis: false,
    });
    expect(message).toBeTypeOf("string");
    expect(message.length).toBeGreaterThan(0);
    // Should not contain common emojis
    const commonEmojis = ["❤️", "🔥", "💫", "🕯️", "💙", "🌺", "💖", "🌙", "💝", "🌸"];
    for (const emoji of commonEmojis) {
      expect(message).not.toContain(emoji);
    }
  });

  it("should generate message with custom timezone", async () => {
    const message = await generateGoodbye({
      templateArgs: { name: "Diana" },
      timezone: "America/New_York",
    });
    expect(message).toBeTypeOf("string");
    expect(message.length).toBeGreaterThan(0);
  });

  it("should produce varied output (randomness)", async () => {
    const messages: string[] = [];
    for (let i = 0; i < 10; i++) {
      const message = await generateGoodbye({
        templateArgs: { name: "Friend" },
      });
      messages.push(message);
    }

    // Should have at least some variation (not all identical)
    const uniqueMessages = new Set(messages);
    expect(uniqueMessages.size).toBeGreaterThan(
      1,
      "All messages were identical - randomness may not be working"
    );

    // All messages should be valid strings
    for (const message of messages) {
      expect(message).toBeTypeOf("string");
      expect(message.length).toBeGreaterThan(0);
    }
  });

  it("should handle all example scenarios", async () => {
    // Basic usage
    const basic = await generateGoodbye();
    expect(basic).toBeTypeOf("string");
    expect(basic.length).toBeGreaterThan(0);

    // With custom name
    const withName = await generateGoodbye({
      templateArgs: { name: "Alice" },
    });
    expect(withName).toBeTypeOf("string");
    expect(withName.toLowerCase()).toContain("alice");

    // With name and location
    const withLocation = await generateGoodbye({
      templateArgs: { name: "Bob", location: "London" },
    });
    expect(withLocation).toBeTypeOf("string");

    // Without emojis
    const noEmojis = await generateGoodbye({
      templateArgs: { name: "Charlie" },
      use_emojis: false,
    });
    expect(noEmojis).toBeTypeOf("string");

    // With timezone
    const withTimezone = await generateGoodbye({
      templateArgs: { name: "Diana" },
      timezone: "America/New_York",
    });
    expect(withTimezone).toBeTypeOf("string");

    // Multiple random messages
    for (let i = 0; i < 3; i++) {
      const message = await generateGoodbye({
        templateArgs: { name: "Friend" },
      });
      expect(message).toBeTypeOf("string");
      expect(message.length).toBeGreaterThan(0);
    }
  });
});

