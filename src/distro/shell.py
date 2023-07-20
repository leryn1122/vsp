#!/usr/bin/env python3

from __future__ import annotations

import os


class cd:
    def __init__(self, path: str | bytes | os.PathLike[str] | os.PathLike[bytes]):
        self.old_cwd = os.getcwd()
        self.path = path

    def __enter__(self):
        os.chdir(self.path)

    def __exit__(self, exc_type, exc_val, exc_tb):
        os.chdir(self.old_cwd)
