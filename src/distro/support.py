#!/usr/bin/env python3

from __future__ import annotations

import logging as log
import os
import platform
import subprocess
import sys
import sysconfig
from typing import AnyStr, Union


Path = Union[str, bytes, os.PathLike[str], os.PathLike[bytes]]

"""
Global variables
"""
PROJECT_ROOT: AnyStr = ''


class cd:
    """
    A `cd` statement
    ```python
    with cd(path):
        // do something...
    ```
    """
    def __init__(self, path: Path):
        self.old_cwd = os.getcwd()
        self.path = path

    def __enter__(self):
        os.chdir(self.path)

    def __exit__(self, exc_type, exc_val, exc_tb):
        os.chdir(self.old_cwd)


def init_logger():
    log.basicConfig(level=log.INFO,
                    stream=sys.stdout,
                    format='%(asctime)s - %(levelname)s - %(message)s')


def project_root() -> AnyStr:
    """
    The function keeping constant, while the directory of source codes changing. The tricky way is
    to go to the parent directory until the working directory includes a marker file, for example,
    `LICENSE-MIT`. It is not recommended to find `.git`, cuz the project might have several git
    submodules.

    :return: Project root.
    """
    global PROJECT_ROOT
    if len(PROJECT_ROOT) != 0:
        return PROJECT_ROOT
    path = os.path.dirname(__file__)
    while not os.path.exists(os.path.join(path, 'LICENSE-MIT')):
        path = os.path.dirname(path)

    PROJECT_ROOT = path
    return PROJECT_ROOT


def get_host_triplet():
    """
    See also
    - https://wiki.osdev.org/Target_Triplet
    - https://llvm.org/docs/doxygen/html/classllvm_1_1Triple.html
    :return:
    """
    if 'HOST_GNU_TYPE' in sysconfig.get_config_vars().keys():
        return sysconfig.get_config_var('HOST_GNU_TYPE')
    else:
        log.critical('Found non standard host triplet')
        uname = platform.uname()
        return '-'.join([uname.machine, 'unknown', uname.system.lower()])


def host_triplet():
    os_type = platform.system()
    cpu_type = platform.machine()
    clib_type = 'gnu'

    if os_type == 'Darwin':
        if cpu_type == 'i386':
            cpu_type = 'x86_64'
        elif cpu_type == 'x86_64':
            cpu_type = 'arm64'

    if os_type == 'Linux':
        ""

    if os_type == 'Android':
        os_type = 'linux-android'
    elif os_type == 'Linux':
        os_type = 'unknown-linux-' + clib_type
    elif os_type == 'Darwin':
        os_type = 'apple-darwin'
    elif os_type == 'FreeBSD':
        os_type = 'unknown-freebsd'
    elif os_type == 'NetBSD':
        os_type = 'unknown-netbsd'
    else:
        raise OSError('Unrecognized OS type: ', os_type)

    if cpu_type in ['i386', 'i486', 'i686', 'i786', 'x86']:
        cpu_type = 'i686'
    elif cpu_type == 'arm':
        if os_type == 'linux-android':
            os_type = 'linux-androideabi'
        else:
            os_type = os_type + 'eabihf'
    elif cpu_type in ['aarch64', 'arm64']:
        cpu_type = 'aarch64'
    elif cpu_type in ['x86_64', 'x86-64', 'x64', 'amd64']:
        cpu_type = 'x86_64'
    else:
        raise OSError('Unrecognized CPU type: ', cpu_type)

    return '-'.join([cpu_type, os_type])


class switch:
    """
    A syntax sugar for switch statement implementation.
    """
    __pattern = None
    __invoked = False
    __result = None

    def __init__(self, pattern):
        self.__pattern = pattern

    @classmethod
    def __eval(cls, handler, *args):
        if callable(handler):
            if args == ((),):
                return handler()
            else:
                print(*args)
                return handler(*args)
        else:
            return handler

    def case(self, case, handler, *args):
        if self.__pattern == case and not self.__invoked:
            self.__invoked = True
            self.__result = switch.__eval(handler, *args)
        return self

    def default(self, handler, *args):
        if not self.__invoked:
            self.__invoked = True
            self.__result = switch.__eval(handler, *args)
        return self

    def result(self):
        if self.__invoked:
            return self.__result
        else:
            raise RuntimeError('switch statement is not exhausted')


def symlink(src: Path, dst: Path) -> None:
    if os.path.exists(dst):
        return
    else:
        return os.symlink(src=src, dst=dst)


def run_command(cmd, cwd=os.getcwd(), suppress=False) -> int:
    p = subprocess.Popen(cmd, shell=True, text=True,
                         stdout=None,
                         stderr=None,
                         cwd=cwd)
    while p.poll() is None:
        ""
    if p.returncode == 0:
        log.info('Subprogram success')
        return 0
    else:
        log.error('Subprogram failed')
        return 1
