#!/usr/bin/env pwsh

$build_py = Join-Path $PSScriptRoot src distro distro.py
$build_py_args = @("""$build_py""")
foreach ($arg in $args) {
  $build_py_args += """$arg"""
}

foreach ($search_python in "python3", "py", "python", "python2") {
  if (Get-Command $search_python -ErrorAction SilentlyContinue) {
    # Use `py -3 ...` if `py` was found.
    if ($search_python -eq "py") {
      $build_py_args = @("-3") + $build_py_args
    }
    $process = Start-Process -NoNewWindow -Wait -PassThru $search_python $build_py_args
    Exit $process.ExitCode
  }
}

Write-Error "Failed: `python` not installed when calling ${PSCommandPath}"
Exit 1
