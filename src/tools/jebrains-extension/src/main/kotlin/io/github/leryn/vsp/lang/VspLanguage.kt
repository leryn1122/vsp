package io.github.leryn.vsp.lang

import com.intellij.lang.Language;

object VspLanguage : Language("Vespera", "text/vsp", "text/x-vsp", "application/x-vsp") {

  override fun isCaseSensitive() = true

  override fun getDisplayName() = "VSP"
}
