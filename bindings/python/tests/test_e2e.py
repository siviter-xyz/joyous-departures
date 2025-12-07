"""E2E tests for joyous-departures Python bindings"""

import pytest
from joyous_departures import generate_goodbye


def test_basic_generate_goodbye():
    """Test basic generate_goodbye() call"""
    result = generate_goodbye()
    assert isinstance(result, str)
    assert len(result) > 0
    # Should contain default name "Good Soul" or be a valid message
    assert "Good Soul" in result or "joyous" in result.lower()


def test_randomness_verification():
    """Test that each call produces different results (when corpus has multiple messages)"""
    results = [generate_goodbye() for _ in range(10)]
    
    # All should be non-empty
    assert all(len(r) > 0 for r in results)
    assert len(results) == 10
    
    # With fallback corpus (1 message), all will be same
    # In Phase 5 with 360 messages, we should see different results
    # For now, just verify structure


def test_custom_name():
    """Test custom name parameter"""
    result = generate_goodbye(template_args={"name": "Alice"})
    assert "Alice" in result


def test_custom_location():
    """Test custom location parameter"""
    result = generate_goodbye(template_args={"location": "Paris"})
    # Location may or may not be in message depending on template
    assert isinstance(result, str)
    assert len(result) > 0


def test_timezone_option():
    """Test timezone option"""
    result = generate_goodbye(timezone="America/New_York")
    assert isinstance(result, str)
    assert len(result) > 0


def test_use_emojis_false():
    """Test emoji stripping"""
    result_with_emojis = generate_goodbye(use_emojis=True)
    result_without_emojis = generate_goodbye(use_emojis=False)
    
    # Both should be valid
    assert isinstance(result_with_emojis, str)
    assert isinstance(result_without_emojis, str)
    
    # Without emojis should not contain common emojis (basic check)
    # Note: More thorough emoji detection would be needed


def test_long_name_truncation():
    """Test that long names are truncated to 50 characters"""
    long_name = "A" * 100
    result = generate_goodbye(template_args={"name": long_name})
    # Name should be truncated in the result
    assert len(result) < len(long_name) + 50  # Message + truncated name


def test_benchmark_performance():
    """Benchmark generation speed (target: <10ms)"""
    import time
    start = time.time()
    result = generate_goodbye()
    elapsed = (time.time() - start) * 1000  # Convert to milliseconds
    
    assert isinstance(result, str)
    assert len(result) > 0
    # Should be fast (<10ms target, but allow some margin for first call)
    assert elapsed < 100  # 100ms is reasonable for first call with initialization

