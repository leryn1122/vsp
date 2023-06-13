package io.github.leryn.vsp.ide.newProject.ui

import com.intellij.openapi.Disposable
import com.intellij.openapi.options.ConfigurationException
import com.intellij.openapi.util.Disposer
import com.intellij.ui.ColoredListCellRenderer
import com.intellij.ui.components.JBList
import com.intellij.ui.dsl.builder.Panel
import io.github.leryn.vsp.ide.newProject.ConfigurationData
import io.github.leryn.vsp.ide.newProject.ProjectTemplate
import io.github.leryn.vsp.openapiext.UiDebouncer
import java.nio.file.Path
import java.nio.file.Paths
import javax.swing.DefaultListModel
import javax.swing.JList
import javax.swing.ListSelectionModel

class VspNewProjectPanel(
  projectDir: Path = Paths.get("."),
  private val updateListener: (() -> Unit)? = null,
) : Disposable {

  private val projectSettingsPanel = VspProjectSettingsPanel(projectDir, updateListener = null)

  override fun dispose() {
    Disposer.dispose(projectSettingsPanel)
  }

  // UI Components

  private val selectedTemplate: ProjectTemplate
    get() = templateList.selectedValue

  private val defaultTemplates: List<ProjectTemplate> = listOf()

  private val templateListModel: DefaultListModel<ProjectTemplate> = JBList.createDefaultListModel(defaultTemplates)

  private val templateList: JBList<ProjectTemplate> = JBList<ProjectTemplate>().apply {
    selectionMode = ListSelectionModel.SINGLE_SELECTION
    selectedIndex = 0
    addListSelectionListener { doUpdate() }
    cellRenderer = object : ColoredListCellRenderer<ProjectTemplate>() {
      override fun customizeCellRenderer(list: JList<out ProjectTemplate>,
                                         value: ProjectTemplate,
                                         index: Int,
                                         selected: Boolean,
                                         hasFocus: Boolean) {
        icon = value.icon
        append(value.name)
      }
    }
  }

  private val debouncer = UiDebouncer(this)

  // Data

  val data: ConfigurationData get() = ConfigurationData(selectedTemplate, projectSettingsPanel.data)

  fun attachTo(panel: Panel) = with(panel) {
    projectSettingsPanel.attachTo(this)
  }

  private fun doUpdate() {
    debouncer.run(
      onPooledThread = {
      },
      onUiThread = {
      }
    )
  }

  private fun updateTemplatesList() {
    val index = templateList.selectedIndex
    with(templateListModel) {
      removeAllElements()
      defaultTemplates.forEach(::addElement)
    }
    templateList.selectedIndex = kotlin.math.min(index, templateList.itemsCount - 1)
  }

  @Throws(ConfigurationException::class)
  fun validateSettings() {
  }


}
