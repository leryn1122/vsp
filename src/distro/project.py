#!/usr/bin/env python3

import argparse
from argparse import Namespace
from enum import Enum
import logging as log

import support
from support import switch
import module


def parse_args() -> Namespace:
    parser = argparse.ArgumentParser(
        prog='Distro',
        description='Commandline tools to build package')
    parser.add_argument('--module',
                        type=TargetModule,
                        default=TargetModule.COMPILER,
                        nargs='+',
                        help='name of the modules to build')
    parser.add_argument('--target-triplet',
                        type=str,
                        default=support.host_triplet(),
                        help='target triplet')
    parser.add_argument('--suppress-bundler-log',
                        action='store_true',
                        help='suppress delegated bundler log')
    args = parser.parse_args()

    # preprocess arguments
    def _tolist(_args) -> list:
        return _args if type(_args) is list else [_args]

    args.module = _tolist(args.module)
    if TargetModule.ALL in args.module:
        _modules = list(TargetModule)
        _modules.remove(TargetModule.ALL)
        args.module = _modules
    return args


class TargetModule(Enum):
    """
    Target module. If `all` is chosen, all the modules is about to build.
    """
    ALL = 'all'
    COMPILER = 'compiler'
    VSCODE_PLUGIN = 'vscode'
    JETBRAINS_PLUGIN = 'jetbrains'

    def __repr__(self) -> str:
        return self.value


def entrypoint() -> int:
    args = parse_args()
    log.info('Ready to build module: %s', args.module)
    log.info('Target triplet is: %s', args.target_triplet)
    for _module in args.module:
        switch(_module).\
            case(TargetModule.COMPILER, module.build_compiler_package, args).\
            case(TargetModule.VSCODE_PLUGIN, module.build_vscode_plugin).\
            case(TargetModule.JETBRAINS_PLUGIN, module.build_jetbrains_plugin).\
            result()
    return 0
