#!/usr/bin/env python3

import os
import sys

if sys.version_info.major < 3:
    try:
        os.execvp("py", ["py", "-3"] + sys.argv)
    except OSError:
        try:
            os.execvp("python3", ["python3"] + sys.argv)
        except OSError:
            pass

project_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.join(project_dir, "src", "distro"))

import distro

distro.main()
