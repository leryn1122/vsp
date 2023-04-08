#!/usr/bin/env python3

import argparse
import os
import shutil
import tarfile
import zipfile

import release_config as cfg

PACKAGE_NAME = ''
IS_NOT_WINDOWS = True


def main():
    global PACKAGE_NAME
    global IS_NOT_WINDOWS

    args = parse_args()
    config = cfg.ReleaseConfig.new(args)

    PACKAGE_NAME = config.get_package_name()
    IS_NOT_WINDOWS = not config.is_target_windows_platform()

    make_target_directory()
    build_compiler()
    copy_conf()
    # copy_legal()
    compress_package()


def parse_args():
    parser = argparse.ArgumentParser(description='Bootstrap script')
    parser.add_argument('--mode', choices=[cfg.COMPLETE_MODE,cfg.COMPACT_MODE], default=cfg.COMPLETE_MODE)
    parser.add_argument('--arch', type=str, default='x86_64')
    parser.add_argument('--vendor', type=str, default='unknown')
    parser.add_argument('--os', type=str, default='linux')
    return parser.parse_args()


def make_target_directory():
    """
    Create the package file tree according to the `src/distro/file-tree.txt`.
    Empty line or line start with `#` would be ignored.
    :return:
    """
    file = os.path.join('src', 'distro', 'file-tree.txt')
    for line in open(file):
        if len(line) == 0 or line.startswith("#"):
            continue
        os.makedirs(
            os.path.join('target', PACKAGE_NAME, line.replace('\n', '')), exist_ok=True)


def build_compiler():
    """
    Run `make build` to build the language compiler.
    :return:
    """
    # subprocess.call(
    #     args=['make build'],
    #     cwd=os.path.join('src', 'compiler'),
    #     shell=True,
    # )
    shutil.copy(
        os.path.join('src', 'compiler', 'target', 'debug', 'vsp'),
        os.path.join('target', PACKAGE_NAME, 'bin')
    )


def copy_conf():
    """
    Copy `src/distro/conf` to `conf`
    :return:
    """
    conf_path = os.path.join('target', PACKAGE_NAME, 'conf')
    shutil.copytree(os.path.join('src', 'distro', 'conf'), conf_path, dirs_exist_ok=True)
    if IS_NOT_WINDOWS:
        pwd = os.getcwd()
        os.chdir(os.path.join('target', PACKAGE_NAME))
        if not os.path.exists('etc'):
            os.symlink('conf', 'etc', target_is_directory=True)
        os.chdir(pwd)


def copy_legal():
    """
    Copy `legal` to `legal`
    :return:
    """
    legal_path = os.path.join('target', PACKAGE_NAME, 'legal')
    shutil.copytree('legal', legal_path, dirs_exist_ok=True)


# noinspection SpellCheckingInspection
def compress_package():
    """
    Compress the final release package using `gz`, i.e. gnu zip, while using `zip` instead on Windows.
    :return:
    """
    if IS_NOT_WINDOWS:
        with tarfile.TarFile.gzopen(os.path.join('target', PACKAGE_NAME + '.tar.gz'),
                                    mode='w',
                                    compresslevel=9
                                    ) as tar_file:
            source_dir = os.path.join('target',  PACKAGE_NAME)
            tar_file.add(source_dir, arcname=os.path.basename(source_dir))
    else:
        with zipfile.ZipFile(os.path.join('target', PACKAGE_NAME + '.zip'),
                             mode='w',
                             compression=zipfile.ZIP_DEFLATED,
                             compresslevel=9
                             ) as zip_file:
            pre_len = len(os.path.dirname(os.path.join('target', PACKAGE_NAME)))
            for basedir, dirnames, filenames in os.walk(os.path.join('target', PACKAGE_NAME)):
                for filename in filenames:
                    fullpath = os.path.join(basedir, filename)
                    arcname = fullpath[pre_len:].strip(os.path.sep)
                    zip_file.write(fullpath, arcname)


if __name__ == '__main__':
    main()
