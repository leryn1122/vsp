#!/usr/bin/env python3

COMPLETE_MODE = 'complete'
COMPACT_MODE = 'compact'


class ReleaseConfig(object):
    def __init__(self):
        self.mode = None
        self.arch = None
        self.vendor = None
        self.os = None
        self.triplet = None
        self.package_name = None

    @classmethod
    def new(cls, args):
        """
        See also
        - https://wiki.osdev.org/Target_Triplet
        - https://llvm.org/docs/doxygen/html/classllvm_1_1Triple.html
        :return:
        """
        config = ReleaseConfig()
        config.mode = args.mode
        config.arch = args.arch
        config.vendor = args.vendor
        config.os = args.os
        config.triplet = '-'.join([config.arch, config.vendor, config.os])
        config.package_name = '-'.join(['vsp', config.mode, config.get_triplet()])
        return config

    def get_mode(self):
        return self.mode

    def get_arch(self):
        return self.arch

    def get_vendor(self):
        return self.vendor

    def get_os(self):
        return self.os

    def get_triplet(self):
        """
        See also
        - https://wiki.osdev.org/Target_Triplet
        - https://llvm.org/docs/doxygen/html/classllvm_1_1Triple.html
        :return:
        """
        return self.triplet

    def get_package_name(self):
        return self.package_name

    def is_target_windows_platform(self):
        return self.get_os() == 'windows'
