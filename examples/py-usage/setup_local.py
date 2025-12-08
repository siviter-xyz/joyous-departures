#!/usr/bin/env python3
"""Setup script to install local package for testing."""
import os
import sys
import subprocess
from pathlib import Path

# Get repo root (assuming this script is in examples/py-usage/)
REPO_ROOT = Path(__file__).parent.parent.parent

# Check for local package path from environment
env_file = REPO_ROOT / "examples" / ".env.local"
local_wheel_path = None

if env_file.exists():
    with open(env_file) as f:
        for line in f:
            if line.startswith("PYTHON_PACKAGE_PATH="):
                path = line.split("=", 1)[1].strip().strip('"').strip("'")
                if path:
                    local_wheel_path = Path(path)
                    if not local_wheel_path.is_absolute():
                        local_wheel_path = REPO_ROOT / local_wheel_path
                    break

# Also check environment variable
if not local_wheel_path:
    env_path = os.getenv("PYTHON_PACKAGE_PATH")
    if env_path:
        local_wheel_path = Path(env_path)
        if not local_wheel_path.is_absolute():
            local_wheel_path = REPO_ROOT / local_wheel_path

if local_wheel_path and local_wheel_path.exists():
    print(f"📦 Installing local package from: {local_wheel_path}")
    subprocess.run(
        [sys.executable, "-m", "pip", "install", "--force-reinstall", str(local_wheel_path)],
        check=True,
    )
    print("✅ Local package installed")
else:
    print("ℹ️  No local package path found, using published package")
    print("   Set PYTHON_PACKAGE_PATH in examples/.env.local to use local builds")

