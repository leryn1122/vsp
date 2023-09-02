#!/usr/bin/env python3

import logging as log
import os
import shutil
from typing import AnyStr

import compress
import shell
import support

"""
Global variables
"""
PROJECT_ROOT: AnyStr = ''
PACKAGE_NAME: AnyStr = ''


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


def package_name() -> AnyStr:
    """
    :return: Package name.
    """
    global PACKAGE_NAME
    if len(PACKAGE_NAME) != 0:
        return PACKAGE_NAME
    PACKAGE_NAME = '-'.join(['vsp', support.host_triplet()])
    return PACKAGE_NAME


def make_target_dir() -> int:
    with shell.cd(project_root()):
        os.makedirs(os.path.join('target', package_name()), exist_ok=True)
    return 0


def package_project() -> int:
    make_target_dir()
    make_project_file_entries()
    compress_project_package()
    return 0


def make_project_file_entries() -> int:
    """
    Make the project file-trees depending on `file-tree.csv`.
    """
    log.info("Make project file-trees.")
    entries = _read_project_file_entries()
    with shell.cd(os.path.join(project_root(), 'target', package_name())):
        for e in entries:
            arr = e.split(',')
            if arr[0] == 'd':
                os.makedirs(name=arr[1], exist_ok=True)
            elif arr[0] == 'f':
                shutil.copy2(src=arr[2], dst=arr[1])
            elif arr[0] == 'fx':
                # TODO:
                # shutil.copy2(src=arr[2], dst=os.path.join(arr[1]))
                pass
            elif arr[0] == 's':
                shell.symlink(src=arr[2], dst=arr[1])
            else:
                raise RuntimeError('Unimplemented file type ' + arr[0])
    return 0


def _read_project_file_entries() -> list[AnyStr]:
    """
    Read the project structure from `file-tree.csv`.
    """
    lines = []
    with open('file-tree.csv', 'r') as f:
        for line in f.readlines():
            if line.startswith('#') or line.isspace():
                continue
            lines.append(line.replace('\n', '').replace('/', os.path.sep))
    return lines


def compress_project_package() -> int:
    _package_name = package_name()
    log.info('Compress the project package: ' + _package_name + '.tar.gz')
    wd = os.path.join(project_root(), "target")
    with shell.cd(wd):
        return compress.compress(
            name=_package_name,
            src=_package_name,
            wd=wd,
        )
