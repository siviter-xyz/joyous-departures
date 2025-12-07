import { generateGoodbye } from "@siviter-xyz/joyous-departures";

async function main() {
  console.log("🎉 Joyous Departures - TypeScript Example\n");

  // Basic usage
  console.log("1. Basic message:");
  const basic = await generateGoodbye();
  console.log(`   ${basic}\n`);

  // With custom name
  console.log("2. With custom name:");
  const withName = await generateGoodbye({
    templateArgs: { name: "Alice" },
  });
  console.log(`   ${withName}\n`);

  // With name and location
  console.log("3. With name and location:");
  const withLocation = await generateGoodbye({
    templateArgs: { name: "Bob", location: "London" },
  });
  console.log(`   ${withLocation}\n`);

  // Without emojis
  console.log("4. Without emojis:");
  const noEmojis = await generateGoodbye({
    templateArgs: { name: "Charlie" },
    use_emojis: false,
  });
  console.log(`   ${noEmojis}\n`);

  // With timezone
  console.log("5. With timezone (America/New_York):");
  const withTimezone = await generateGoodbye({
    templateArgs: { name: "Diana" },
    timezone: "America/New_York",
  });
  console.log(`   ${withTimezone}\n`);

  // Multiple messages (showing randomness)
  console.log("6. Multiple random messages:");
  for (let i = 1; i <= 3; i++) {
    const message = await generateGoodbye({
      templateArgs: { name: "Friend" },
    });
    console.log(`   ${i}. ${message}`);
  }
}

main().catch(console.error);

