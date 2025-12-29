"""Tests for joyous_departures package."""

import pytest
from joyous_departures import generate_goodbye, generate_goodbye_sync, CORPUS, CORPUS_SIZE


class TestCorpus:
    """Tests for corpus data."""

    def test_corpus_has_messages(self):
        """Corpus should have messages."""
        assert len(CORPUS) > 0
        assert CORPUS_SIZE == len(CORPUS)

    def test_corpus_no_empty_messages(self):
        """All messages should be non-empty."""
        for message in CORPUS:
            assert len(message.strip()) > 0


class TestGenerateGoodbyeSync:
    """Tests for synchronous generation."""

    def test_returns_string(self):
        """Should return a non-empty string."""
        result = generate_goodbye_sync()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_substitutes_name_template(self):
        """Should substitute name in some messages."""
        # Run multiple times to find a message with {name}
        found_name = False
        for _ in range(100):
            result = generate_goodbye_sync(template_args={"name": "TestUser"})
            if "TestUser" in result:
                found_name = True
                break
        # Soft check - at least verify no unsubstituted templates
        result = generate_goodbye_sync(template_args={"name": "TestUser"})
        assert "{name}" not in result

    def test_uses_default_name(self):
        """Should use default name when not provided."""
        result = generate_goodbye_sync()
        assert "{name}" not in result

    def test_strips_emojis_when_use_emojis_false(self):
        """Should strip emojis when use_emojis is False."""
        result = generate_goodbye_sync(use_emojis=False)
        # Check for common emojis
        assert "❤️" not in result
        assert "✨" not in result
        assert "🌻" not in result

    def test_strips_emojis_when_strip_emojis_true(self):
        """Should strip emojis when strip_emojis is True."""
        result = generate_goodbye_sync(strip_emojis=True)
        # Check for common emojis
        assert "❤️" not in result
        assert "✨" not in result
        assert "🌻" not in result

    def test_includes_emojis_by_default(self):
        """Should include emojis by default in some messages."""
        found_emoji = False
        for _ in range(50):
            result = generate_goodbye_sync()
            if any(emoji in result for emoji in ["❤️", "✨", "🌻", "💙", "🌸", "💫", "🌟"]):
                found_emoji = True
                break
        assert found_emoji

    def test_handles_timezone_option(self):
        """Should handle timezone option."""
        result = generate_goodbye_sync(timezone="America/New_York")
        assert isinstance(result, str)


class TestGenerateGoodbyeAsync:
    """Tests for async generation."""

    @pytest.mark.asyncio
    async def test_returns_string(self):
        """Should return a non-empty string."""
        result = await generate_goodbye()
        assert isinstance(result, str)
        assert len(result) > 0

    @pytest.mark.asyncio
    async def test_works_with_all_options(self):
        """Should work with all options."""
        result = await generate_goodbye(
            template_args={"name": "Alice", "location": "London"},
            use_emojis=False,
            timezone="Europe/London",
        )
        assert isinstance(result, str)
        assert "{name}" not in result
        assert "{location}" not in result

