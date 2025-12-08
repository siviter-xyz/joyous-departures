"""Tests for the Python example usage."""
import pytest
import asyncio
from joyous_departures import generate_goodbye


@pytest.mark.asyncio
async def test_basic_message():
    """Test basic message generation."""
    message = await generate_goodbye()
    assert isinstance(message, str)
    assert len(message) > 0
    # Should contain some text
    assert len(message.strip()) > 0


@pytest.mark.asyncio
async def test_custom_name():
    """Test message with custom name."""
    message = await generate_goodbye(template_args={"name": "Alice"})
    assert isinstance(message, str)
    assert len(message) > 0
    # Name should appear in message (case-insensitive)
    assert "alice" in message.lower()


@pytest.mark.asyncio
async def test_custom_name_and_location():
    """Test message with custom name and location."""
    message = await generate_goodbye(
        template_args={"name": "Bob", "location": "London"}
    )
    assert isinstance(message, str)
    assert len(message) > 0
    # Name should appear in message
    assert "bob" in message.lower()


@pytest.mark.asyncio
async def test_no_emojis():
    """Test message without emojis."""
    message = await generate_goodbye(
        template_args={"name": "Charlie"}, use_emojis=False
    )
    assert isinstance(message, str)
    assert len(message) > 0
    # Should not contain common emojis (basic check)
    # Note: This is a simple check - some unicode might still appear
    common_emojis = ["❤️", "🔥", "💫", "🕯️", "💙", "🌺", "💖", "🌙", "💝", "🌸"]
    for emoji in common_emojis:
        assert emoji not in message, f"Found emoji {emoji} in message when use_emojis=False"


@pytest.mark.asyncio
async def test_timezone():
    """Test message with custom timezone."""
    message = await generate_goodbye(
        template_args={"name": "Diana"}, timezone="America/New_York"
    )
    assert isinstance(message, str)
    assert len(message) > 0
    # Should generate successfully with timezone


@pytest.mark.asyncio
async def test_randomness():
    """Test that multiple calls produce varied output."""
    messages = []
    for _ in range(10):
        message = await generate_goodbye(template_args={"name": "Friend"})
        messages.append(message)
    
    # Should have at least some variation (not all identical)
    unique_messages = set(messages)
    assert len(unique_messages) > 1, "All messages were identical - randomness may not be working"
    
    # All messages should be valid strings
    for message in messages:
        assert isinstance(message, str)
        assert len(message) > 0


@pytest.mark.asyncio
async def test_all_example_scenarios():
    """Test all scenarios from the example main() function."""
    # Basic usage
    basic = await generate_goodbye()
    assert isinstance(basic, str)
    assert len(basic) > 0
    
    # With custom name
    with_name = await generate_goodbye(template_args={"name": "Alice"})
    assert isinstance(with_name, str)
    assert "alice" in with_name.lower()
    
    # With name and location
    with_location = await generate_goodbye(
        template_args={"name": "Bob", "location": "London"}
    )
    assert isinstance(with_location, str)
    
    # Without emojis
    no_emojis = await generate_goodbye(
        template_args={"name": "Charlie"}, use_emojis=False
    )
    assert isinstance(no_emojis, str)
    
    # With timezone
    with_timezone = await generate_goodbye(
        template_args={"name": "Diana"}, timezone="America/New_York"
    )
    assert isinstance(with_timezone, str)
    
    # Multiple random messages
    for i in range(3):
        message = await generate_goodbye(template_args={"name": "Friend"})
        assert isinstance(message, str)
        assert len(message) > 0

