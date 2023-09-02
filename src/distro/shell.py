#!/usr/bin/env python3

from __future__ import annotations
import os

from type import Path


class cd:
    def __init__(self, path: Path):
        self.old_cwd = os.getcwd()
        self.path = path

    def __enter__(self):
        os.chdir(self.path)

    def __exit__(self, exc_type, exc_val, exc_tb):
        os.chdir(self.old_cwd)


def symlink(src: Path, dst: Path) -> None:
    if os.path.exists(dst):
        return
    else:
        return os.symlink(src=src, dst=dst)
