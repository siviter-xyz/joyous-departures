"""Example usage of joyous-departures Python package."""
import asyncio
from joyous_departures import generate_goodbye


async def main():
    print("🎉 Joyous Departures - Python Example\n")

    # Basic usage
    print("1. Basic message:")
    basic = await generate_goodbye()
    print(f"   {basic}\n")

    # With custom name
    print("2. With custom name:")
    with_name = await generate_goodbye(template_args={"name": "Alice"})
    print(f"   {with_name}\n")

    # With name and location
    print("3. With name and location:")
    with_location = await generate_goodbye(
        template_args={"name": "Bob", "location": "London"}
    )
    print(f"   {with_location}\n")

    # Without emojis
    print("4. Without emojis:")
    no_emojis = await generate_goodbye(
        template_args={"name": "Charlie"}, use_emojis=False
    )
    print(f"   {no_emojis}\n")

    # With timezone
    print("5. With timezone (America/New_York):")
    with_timezone = await generate_goodbye(
        template_args={"name": "Diana"}, timezone="America/New_York"
    )
    print(f"   {with_timezone}\n")

    # Multiple messages (showing randomness)
    print("6. Multiple random messages:")
    for i in range(1, 4):
        message = await generate_goodbye(template_args={"name": "Friend"})
        print(f"   {i}. {message}")


if __name__ == "__main__":
    asyncio.run(main())

