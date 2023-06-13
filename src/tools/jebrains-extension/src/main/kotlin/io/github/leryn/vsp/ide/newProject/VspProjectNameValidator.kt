package io.github.leryn.vsp.ide.newProject

object VspProjectNameValidator {


  fun validate(name: String, isBinary: Boolean = false): String? = when {
    name.isEmpty() -> "Package name can not be empty"
    else -> null
  }

  fun normalize(name: String): String = name.replace(' ', '_')

}
