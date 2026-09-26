#!/usr/bin/env python3
"""Docker/Podman orchestration for TSDM Pokemon Plugin development.

Usage:
    python scripts/docker/dev.py up         # start all services
    python scripts/docker/dev.py down       # stop and remove
    python scripts/docker/dev.py restart    # restart app container
    python scripts/docker/dev.py logs       # follow logs
    python scripts/docker/dev.py status     # show container status
    python scripts/docker/dev.py clean      # full teardown including volumes
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent

sys.path.insert(0, str(ROOT / "scripts" / "docker"))
from _docker import docker_cmd, compose_cmd, compose_file


def up() -> None:
    env = os.environ.copy()
    env["DOCKER_BUILDKIT"] = "0"
    cmd = [*compose_cmd(), "-f", str(compose_file()), "up", "-d", "--build"]
    try:
        subprocess.run(cmd, check=True, env=env)
    except subprocess.CalledProcessError:
        if docker_cmd() == ["podman"]:
            print(
                "\n[tsdm] podman 起栈失败。若上方日志含 netavark/nftables 错误\n"
                '      ("nft did not return successfully")，是 WSL 内核不支持\n'
                "      netavark 的 nft 规则；把防火墙后端切到 iptables 可解：\n"
                "\n"
                "        podman machine ssh \"sudo mkdir -p /etc/containers && "
                "printf '[network]\\nfirewall_driver = \\\"iptables\\\"\\n' "
                "| sudo tee -a /etc/containers/containers.conf\"\n"
                "        podman machine stop && podman machine start\n"
                "\n"
                "      然后重试 python scripts/docker/dev.py up"
            )
        raise
    print("[tsdm] services starting...")
    _wait_healthy("tsdm-db", timeout=30)
    if _container_exists("tsdm-setup"):
        print("[tsdm] waiting for setup to complete...")
        _wait_exit("tsdm-setup", timeout=60)
    print("[tsdm] ready at http://localhost")


def down(volumes: bool = False) -> None:
    cmd = [*compose_cmd(), "-f", str(compose_file()), "down"]
    if volumes:
        cmd.append("-v")
    subprocess.run(cmd, check=True)
    print("[tsdm] services stopped.")


def restart(service: str = "tsdm-app") -> None:
    subprocess.run([*docker_cmd(), "restart", service], check=True)
    print(f"[tsdm] {service} restarted.")


def logs(service: str = "") -> None:
    args = [*compose_cmd(), "-f", str(compose_file()), "logs", "-f"]
    if service:
        args.append(service)
    subprocess.run(args, check=False)


def status() -> None:
    subprocess.run(
        [*compose_cmd(), "-f", str(compose_file()), "ps"],
        check=False,
    )


def clean() -> None:
    down(volumes=True)
    print("[tsdm] full clean complete.")


def _wait_healthy(container: str, timeout: int = 30) -> None:
    for _ in range(timeout):
        r = subprocess.run(
            [*docker_cmd(), "inspect", "--format={{.State.Health.Status}}", container],
            capture_output=True, text=True,
        )
        if r.stdout.strip() == "healthy":
            return
        time.sleep(1)
    print(f"[tsdm] warning: {container} did not become healthy within {timeout}s")


def _wait_exit(container: str, timeout: int = 60) -> None:
    for _ in range(timeout):
        r = subprocess.run(
            [*docker_cmd(), "inspect", "--format={{.State.Status}}", container],
            capture_output=True, text=True,
        )
        if r.stdout.strip() == "exited":
            return
        time.sleep(1)


def _container_exists(name: str) -> bool:
    r = subprocess.run(
        [*docker_cmd(), "ps", "-a", "--filter", f"name={name}", "--format={{.Names}}"],
        capture_output=True, text=True,
    )
    return name in r.stdout


def main() -> None:
    p = argparse.ArgumentParser(description="TSDM Docker dev orchestration")
    sp = p.add_subparsers(dest="cmd")

    sp.add_parser("up", help="start all services")
    sp.add_parser("down", help="stop services")
    sp.add_parser("restart", help="restart app container")
    sp.add_parser("logs", help="follow logs")
    sp.add_parser("status", help="show container status")
    sp.add_parser("clean", help="full teardown including volumes")

    args = p.parse_args()
    if args.cmd == "up":
        up()
    elif args.cmd == "down":
        down()
    elif args.cmd == "restart":
        restart()
    elif args.cmd == "logs":
        logs()
    elif args.cmd == "status":
        status()
    elif args.cmd == "clean":
        clean()
    else:
        p.print_help()


if __name__ == "__main__":
    main()
