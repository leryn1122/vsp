package io.github.leryn.vsp.ide.newProject.ui

import com.intellij.openapi.fileChooser.FileChooser
import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.ui.ComboBoxWithWidePopup
import com.intellij.openapi.ui.ComponentWithBrowseButton
import com.intellij.ui.ComboboxSpeedSearch
import com.intellij.ui.components.fields.ExtendableTextField
import io.github.leryn.vsp.openapiext.addTextChangeListener
import io.github.leryn.vsp.support.toPath
import io.github.leryn.vsp.support.toPathOrNull
import java.nio.file.Path
import javax.swing.plaf.basic.BasicComboBoxEditor

class ToolchainPathComboBox(onTextChanged: () -> Unit = {}) : ComponentWithBrowseButton<ComboBoxWithWidePopup<Path>>(ComboBoxWithWidePopup(), null) {

  private val editor: BasicComboBoxEditor = object : BasicComboBoxEditor() {
    override fun createEditorComponent(): ExtendableTextField = ExtendableTextField()
  }

  private val pathTextField: ExtendableTextField
    get() = childComponent.editor.editorComponent as ExtendableTextField

  var selectedPath: Path?
    get() = pathTextField.text?.toPathOrNull()
    set(value) {
      pathTextField.text = value?.toString().orEmpty()
    }

  init {
    ComboboxSpeedSearch(childComponent)
    childComponent.editor = editor
    childComponent.isEditable = true

    addActionListener {
      val descriptor = FileChooserDescriptorFactory.createSingleFolderDescriptor()
      FileChooser.chooseFile(descriptor, null, null) { file ->
        childComponent.selectedItem = file.canonicalPath?.toPath()
      }
    }

    pathTextField.addTextChangeListener { onTextChanged() }
  }

}
