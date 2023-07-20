#!/usr/bin/env python3

import logging as log
import platform
import sys
import sysconfig


def init_logger():
    log.basicConfig(level=log.INFO,
                    stream=sys.stdout,
                    format='%(asctime)s - %(levelname)s - %(message)s')


def host_triplet():
    """
    See also
    - https://wiki.osdev.org/Target_Triplet
    - https://llvm.org/docs/doxygen/html/classllvm_1_1Triple.html
    :return:
    """
    if 'BUILD_GNU_TYPE' in sysconfig.get_config_vars().keys():
        return sysconfig.get_config_var('BUILD_GNU_TYPE')
    else:
        log.critical('Found non standard host triplet')
        uname = platform.uname()
        return '-'.join([uname.machine, 'unknown', uname.system.lower()])
