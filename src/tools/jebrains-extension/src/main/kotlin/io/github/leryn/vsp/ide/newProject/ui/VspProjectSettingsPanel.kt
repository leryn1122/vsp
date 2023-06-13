package io.github.leryn.vsp.ide.newProject.ui

import com.intellij.openapi.Disposable
import com.intellij.openapi.options.ConfigurationException
import com.intellij.openapi.util.Disposer
import com.intellij.ui.dsl.builder.Panel
import com.intellij.ui.dsl.gridLayout.HorizontalAlign
import io.github.leryn.vsp.VspBundle
import io.github.leryn.vsp.ide.newProject.ProjectSettingsData
import java.nio.file.Path
import java.nio.file.Paths
import javax.swing.JLabel

class VspProjectSettingsPanel(
  private val projectDir: Path = Paths.get("."),
  private val updateListener: (() -> Unit)? = null
) : Disposable {

  override fun dispose() {
    Disposer.dispose(toolchainPathComboBox)
  }

  // UI Components

  private val toolchainPathComboBox = ToolchainPathComboBox { doUpdate() }

  private val toolchainVersion = JLabel()

  // Data

  var data: ProjectSettingsData
    get() {
      return ProjectSettingsData(
        toolchain = "~",
        explicitStdlibPath = "~"
      )
    }
    set(value) {

    }


  fun attachTo(panel: Panel) = with(panel) {
    data = ProjectSettingsData(
      toolchain = "~",
      explicitStdlibPath = "~",
    )

    row(VspBundle.message("settings.vsp.toolchain.location")) {
      cell(toolchainPathComboBox).horizontalAlign(HorizontalAlign.FILL)
    }
    row(VspBundle.message("settings.vsp.toolchain.version")) {
      cell()
    }
  }

  @Throws(ConfigurationException::class)
  fun validateSettings() {

  }

  private fun doUpdate() {

  }
}
