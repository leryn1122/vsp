import os
import shutil
import subprocess
import sys
import tarfile
import zipfile


def main():
    make_target_directory()
    build_compiler()
    copy_conf()
    copy_legal()
    compress_package()


def is_target_windows_platform():
    """
    Determine if the target platform is Windows.
    :return: True if the target platform is Windows.
    """
    return False


def get_triple():
    # TODO
    return 'linux-amd64-unknown'


def make_target_directory():
    """
    Create the package file tree according to the `src/bootstrap/file-tree.txt`.
    Empty line or line start with `#` would be ignored.
    :return:
    """
    file = os.path.join('src', 'bootstrap', 'file-tree.txt')
    for line in open(file):
        if len(line) == 0 or line.startswith("#"):
            continue
        os.makedirs(
            os.path.join('target', 'vsp-' + get_triple(), line.replace('\n', '')),
            exist_ok=True)


def build_compiler():
    """
    Run `cargo build` to build the language compiler.
    :return:
    """
    subprocess.call(
        args=['cargo build'],
        cwd=os.path.join('src', 'compiler'),
        shell=True,
    )
    shutil.copy(
        os.path.join('src', 'compiler', 'target', 'debug', 'vsp'),
        os.path.join('target', 'vsp-' + get_triple(), 'bin')
    )


def copy_conf():
    """
    Copy `src/bootstrap/conf` to `conf`
    :return:
    """
    conf_path = os.path.join('target', 'vsp-' + get_triple(), 'conf')
    shutil.copytree(os.path.join('src', 'bootstrap', 'conf'), conf_path, dirs_exist_ok=True)
    if not is_target_windows_platform():
        pwd = os.getcwd()
        os.chdir(os.path.join('target', 'vsp-' + get_triple()))
        if not os.path.exists('etc'):
            os.symlink('conf', 'etc', target_is_directory=True)
        os.chdir(pwd)


def copy_legal():
    """
    Copy `legal` to `legal`
    :return:
    """
    legal_path = os.path.join('target', 'vsp-' + get_triple(), 'legal')
    shutil.copytree('legal', legal_path, dirs_exist_ok=True)


# noinspection SpellCheckingInspection
def compress_package():
    """
    Compress the final release package using `gz`, i.e. gnu zip, while using `zip` instead on Windows.
    :return:
    """
    if is_target_windows_platform():
        with zipfile.ZipFile(os.path.join('target', 'vsp-' + get_triple() + '.zip'),
                             mode='w',
                             compression=zipfile.ZIP_DEFLATED,
                             compresslevel=9
                             ) as zip_file:
            pre_len = len(os.path.dirname(os.path.join('target', 'vsp-' + get_triple())))
            for basedir, dirnames, filenames in os.walk(os.path.join('target', 'vsp-' + get_triple())):
                for filename in filenames:
                    fullpath = os.path.join(basedir, filename)
                    arcname = fullpath[pre_len:].strip(os.path.sep)
                    zip_file.write(fullpath, arcname)
    else:
        with tarfile.TarFile.gzopen(os.path.join('target', 'vsp-' + get_triple() + '.tar.gz'),
                                    mode='w',
                                    compresslevel=9
                                    ) as tar_file:
            source_dir = os.path.join('target', 'vsp-' + get_triple())
            tar_file.add(source_dir, arcname=os.path.basename(source_dir))


if __name__ == '__main__':
    main()
