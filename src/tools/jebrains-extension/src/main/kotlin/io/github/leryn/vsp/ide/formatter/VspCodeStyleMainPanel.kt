package io.github.leryn.vsp.ide.formatter

import com.intellij.application.options.TabbedLanguageCodeStylePanel
import com.intellij.psi.codeStyle.CodeStyleSettings
import io.github.leryn.vsp.lang.VspLanguage

class VspCodeStyleMainPanel(currentSettings: CodeStyleSettings, settings: CodeStyleSettings) : TabbedLanguageCodeStylePanel(VspLanguage, currentSettings, settings) {

  override fun initTabs(settings: CodeStyleSettings) {
    addIndentOptionsTab(settings)
//    addSpacesTab(settings)
//    addWrappingAndBracesTab(settings)
//    addBlankLinesTab(settings)
//    addTab(GenerationCodeStylePanel(settings, VspLanguage))
  }

}
