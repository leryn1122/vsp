from __future__ import annotations

import os
import sys
import tarfile
import zipfile

from support import Path


def compress(name: Path, src: Path, wd: Path = os.getcwd()) -> int:
    """
    Compress the final release package using `gz`, i.e. gnu zip,
    while using `zip` instead on Windows.
    :return:
    """
    if 'windows' in sys.platform:
        return compress_by_zip(name=name + '.zip', wd=wd, src=src)
    else:
        return compress_by_tar(name=name + '.tar.gz', wd=wd, src=src)


def compress_by_tar(name: Path, src: Path, wd: Path = os.getcwd()) -> int:
    """
    Equivalent to `tar -cf <name> <src>`
    """
    with tarfile.TarFile.gzopen(
        name=os.path.join(wd, name),
        mode='w',
        compresslevel=9
    ) as tar:
        tar.add(src, arcname=os.path.basename(src))
    return 0


def compress_by_zip(name: Path, src: Path, wd: Path = os.getcwd()) -> int:
    archive = os.path.join(wd, name)
    l = len(src)
    with zipfile.ZipFile(
        file=archive,
        mode='w',
        compression=zipfile.ZIP_DEFLATED,
    ) as zip_file:
        for base, dirs, files in os.walk(src):
            for f in files:
                path = os.path.join(base, f)
                entry = path[l:].strip(os.path.sep)
                zip_file.write(path, entry)
    return 0
