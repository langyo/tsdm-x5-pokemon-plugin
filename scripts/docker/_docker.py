"""Docker/Podman auto-detection utilities.

Copied from celestia-devtools pattern.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

_cached_cmd: list[str] | None = None
_cached_compose_cmd: list[str] | None = None


def docker_cmd() -> list[str]:
    """Return ['docker'] or ['podman'], preferring podman if daemon is alive.

    TSDM_DOCKER_RUNTIME=docker|podman forces a choice and skips probing:
    the probe result can drift within one CI job (rootless podman on a
    fresh runner initializes slowly, so dev.py may pick docker while a
    later process finds podman alive - and then execs a runtime that runs
    no containers).
    """
    global _cached_cmd
    if _cached_cmd is not None:
        return _cached_cmd
    forced = os.environ.get("TSDM_DOCKER_RUNTIME", "").strip().lower()
    if forced in ("docker", "podman"):
        _cached_cmd = [forced]
        return _cached_cmd
    if _daemon_alive("podman"):
        _cached_cmd = ["podman"]
    elif _daemon_alive("docker"):
        _cached_cmd = ["docker"]
    else:
        _cached_cmd = ["docker"]
    return _cached_cmd


def compose_cmd() -> list[str]:
    """Return ['docker', 'compose'] or ['podman-compose']."""
    global _cached_compose_cmd
    if _cached_compose_cmd is not None:
        return _cached_compose_cmd
    if docker_cmd() == ["docker"]:
        _cached_compose_cmd = ["docker", "compose"]
    else:
        _cached_compose_cmd = ["podman-compose"]
    return _cached_compose_cmd


def compose_file() -> Path:
    return Path(__file__).resolve().parent.parent.parent / "docker" / "docker-compose.yml"


def _daemon_alive(cmd: str) -> bool:
    try:
        r = subprocess.run([cmd, "info"], capture_output=True, timeout=5)
        return r.returncode == 0
    except Exception:
        return False
