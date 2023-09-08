#!/usr/bin/env python3

import logging as log
import os
import sys

import project
import support

if __name__ == '__main__':
    """
    Wrapper of main entrypoint.
    """
    if sys.version_info.major < 3:
        log.critical("""
        There are no guarantees that it is compatible using python of version 2.x or lower.
        Try `python3` or `venv`, virtual environment, if any exception on compatibility occurred.
        """)
        try:
            os.execvp('py', ['py', '-3'] + sys.argv)
        except OSError:
            try:
                os.execvp('python3', ['python3'] + sys.argv)
            except OSError:
                pass
    support.init_logger()
    project.entrypoint()
