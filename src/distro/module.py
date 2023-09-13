#!/usr/bin/env python3

import logging as log
import os
import shutil
from typing import AnyStr

import compress
import support
from support import cd, Path

PACKAGE_NAME: AnyStr = ''


def build_compiler_package(args) -> int:
    __make_target_dir(__compiler_package_name())
    __compile_compiler(args.suppress_bundler_log)
    __make_project_file_entries()
    __compress_project_package()
    return 0


def build_vscode_plugin() -> int:
    return 0


def build_jetbrains_plugin() -> int:
    return 0


def __make_target_dir(directory: Path) -> int:
    with cd(support.project_root()):
        os.makedirs(os.path.join('target', directory), exist_ok=True)
    return 0


def __compiler_package_name() -> AnyStr:
    """
    :return: Compiler package name.
    """
    global PACKAGE_NAME
    if len(PACKAGE_NAME) != 0:
        return PACKAGE_NAME
    PACKAGE_NAME = '-'.join(['vsp', support.host_triplet()])
    return PACKAGE_NAME


def __make_project_file_entries() -> int:
    """
    Make the project file-trees depending on `file-tree.csv`.
    """
    log.info("Make project file entries.")
    entries = __read_project_file_entries()
    with cd(os.path.join(support.project_root(), 'target', __compiler_package_name())):
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
                support.symlink(src=arr[2], dst=arr[1])
            else:
                raise RuntimeError('Unimplemented file type ' + arr[0])
    return 0


def __read_project_file_entries() -> list[AnyStr]:
    """
    Read the project structure from `file-tree.csv`.
    """
    lines = []
    with open('resources/file-tree.csv', 'r') as f:
        for line in f.readlines():
            if line.startswith('#') or line.isspace():
                continue
            lines.append(line.replace('\n', '').replace('/', os.path.sep))
    return lines


def __compress_project_package() -> int:
    _package_name = __compiler_package_name()
    log.info('Compress the project package: ' + _package_name + '.tar.gz')
    wd = os.path.join(support.project_root(), "target")
    with cd(wd):
        return compress.compress(
            name=_package_name,
            src=_package_name,
            wd=wd,
        )


def __compile_compiler(suppressed: bool) -> int:
    cmd = ['make install && make build']
    cwd = os.path.join(support.project_root(), 'src', 'compiler')
    return support.run_command(cmd, name='cargo', cwd=cwd, suppressed=suppressed)
