"""Docker/Podman auto-detection utilities.

Copied from celestia-devtools pattern.
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

_cached_cmd: list[str] | None = None


def docker_cmd() -> list[str]:
    """Return ['docker'] or ['podman'], preferring docker if daemon is alive."""
    global _cached_cmd
    if _cached_cmd is not None:
        return _cached_cmd
    if _daemon_alive("docker"):
        _cached_cmd = ["docker"]
    elif _daemon_alive("podman"):
        _cached_cmd = ["podman"]
    else:
        _cached_cmd = ["docker"]
    return _cached_cmd


def compose_file() -> Path:
    return Path(__file__).resolve().parent.parent.parent / "docker" / "docker-compose.yml"


def _daemon_alive(cmd: str) -> bool:
    try:
        r = subprocess.run([cmd, "info"], capture_output=True, timeout=5)
        return r.returncode == 0
    except Exception:
        return False
