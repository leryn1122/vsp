package io.github.leryn.vsp.project

/**
 * Constants for project.
 */
object ProjectConstants {

  const val MANIFEST_FILE = "manifest.toml"
  const val LOCK_FILE = "manifest.lock"

  object ProjectLayout {
    val sources = listOf("src", "examples")
    val tests = listOf("tests", "benches")
    const val target = "target"
  }
}
