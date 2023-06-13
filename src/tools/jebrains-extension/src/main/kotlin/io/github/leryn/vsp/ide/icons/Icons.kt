package io.github.leryn.vsp.ide.icons

import com.intellij.openapi.util.IconLoader
import javax.swing.Icon

object Icons {

  val LOGO = load("/icons/logo.svg")

  val VSP_FILE = load("/icons/vsp-file.svg")
  private fun load(path: String): Icon = IconLoader.getIcon(path, Icons::class.java)
}
