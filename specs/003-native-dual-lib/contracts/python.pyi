"""
API Contract: joyous-departures v2.0

This file defines the public Python API for the library.
Implementation must conform to these type signatures.
"""

from typing import TypedDict, Callable, Awaitable

class TemplateArgs(TypedDict, total=False):
    """Template variable values for message customization."""
    
    name: str
    """Recipient's name (max 50 chars). Default: "Good Soul" """
    
    location: str
    """Location context (max 100 chars). Default: "The World" """
    
    date: str
    """Date string, format: YYYY-MM-DD. Default: current date"""
    
    time: str
    """Time string, format: HH:MM. Default: current time"""


def generate_goodbye_sync(
    *,
    template_args: dict[str, str] | None = None,
    strip_emojis: bool = False,
    timezone: str = "Europe/London",
) -> str:
    """
    Generate a warm, heartfelt goodbye message (synchronous).
    
    Args:
        template_args: Template variable values
        strip_emojis: Remove emoji characters from output. Default: False
        timezone: IANA timezone for date/time formatting. Default: "Europe/London"
    
    Returns:
        The generated message
    
    Example:
        >>> from joyous_departures import generate_goodbye_sync
        >>> message = generate_goodbye_sync(template_args={"name": "Alice"})
        >>> print(message)
        "Wishing you a joyous day, Alice ❤️"
    """
    ...


async def generate_goodbye(
    *,
    template_args: dict[str, str] | None = None,
    strip_emojis: bool = False,
    timezone: str = "Europe/London",
    translator: Callable[[str], Awaitable[str]] | None = None,
) -> str:
    """
    Generate a warm, heartfelt goodbye message (async).
    
    Args:
        template_args: Template variable values
        strip_emojis: Remove emoji characters from output. Default: False
        timezone: IANA timezone for date/time formatting. Default: "Europe/London"
        translator: Optional async translator callback for custom translation
    
    Returns:
        The generated message (optionally translated)
    
    Example:
        >>> import asyncio
        >>> from joyous_departures import generate_goodbye
        >>> 
        >>> async def main():
        ...     message = await generate_goodbye(template_args={"name": "Alice"})
        ...     print(message)
        >>> 
        >>> asyncio.run(main())
        "Wishing you a joyous day, Alice ❤️"
    """
    ...


# The embedded message corpus (immutable tuple).
# Code-generated from corpus/en-GB.txt at build time.
CORPUS: tuple[str, ...]

__version__: str
__all__: list[str]

