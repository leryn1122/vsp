package io.github.leryn.vsp.ide.formatter

import com.intellij.application.options.CodeStyleAbstractConfigurable
import com.intellij.application.options.CodeStyleAbstractPanel
import com.intellij.application.options.IndentOptionsEditor
import com.intellij.lang.Language
import com.intellij.psi.codeStyle.*
import io.github.leryn.vsp.lang.VspLanguage

class VspLanguageCodeStyleSettingsProvider : LanguageCodeStyleSettingsProvider() {

  override fun getLanguage(): Language = VspLanguage

  override fun createCustomSettings(settings: CodeStyleSettings): CustomCodeStyleSettings =
    VspCodeStyleSettings(settings)

  override fun createConfigurable(baseSettings: CodeStyleSettings, modelSettings: CodeStyleSettings): CodeStyleConfigurable {
    return object : CodeStyleAbstractConfigurable(baseSettings, modelSettings, configurableDisplayName) {
      override fun createPanel(settings: CodeStyleSettings): CodeStyleAbstractPanel =
        VspCodeStyleMainPanel(currentSettings, settings)
    }
  }

  override fun getCodeSample(settingsType: SettingsType): String =
    when (settingsType) {
      SettingsType.INDENT_SETTINGS -> INDENT_SAMPLE
//      SettingsType.SPACING_SETTINGS -> SPACING_SAMPLE
//      SettingsType.WRAPPING_AND_BRACES_SETTINGS -> WRAPPING_AND_BRACES_SAMPLE
//      SettingsType.BLANK_LINES_SETTINGS -> BLANK_LINES_SAMPLE
      else -> ""
    }

  override fun customizeSettings(consumer: CodeStyleSettingsCustomizable, settingsType: SettingsType) {
    super.customizeSettings(consumer, settingsType)
  }

  override fun getIndentOptionsEditor(): IndentOptionsEditor? {
    return super.getIndentOptionsEditor()
  }

  override fun customizeDefaults(commonSettings: CommonCodeStyleSettings, indentOptions: CommonCodeStyleSettings.IndentOptions) {
    super.customizeDefaults(commonSettings, indentOptions)
  }

}

private fun sample(@org.intellij.lang.annotations.Language("Vespera") code: String) = code.trim()

private val INDENT_SAMPLE = sample("""
pub enum Either<L, R> {
}
""")
