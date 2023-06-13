package io.github.leryn.vsp.support

import java.nio.file.InvalidPathException
import java.nio.file.Path
import java.nio.file.Paths

fun String.toPathOrNull(): Path? = pathOrNull(this::toPath)

fun String.toPath(): Path = Paths.get(this)

private fun pathOrNull(block: () -> Path): Path? {
  return try {
    block()
  } catch (e: InvalidPathException) {
    null
  }
}
