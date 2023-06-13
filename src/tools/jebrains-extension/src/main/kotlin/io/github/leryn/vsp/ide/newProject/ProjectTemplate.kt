package io.github.leryn.vsp.ide.newProject

import com.intellij.openapi.util.NlsContexts.ListItem
import javax.swing.Icon

sealed class ProjectTemplate(@ListItem val name: String, val isBinary: Boolean, val icon: Icon) {
  fun validateProjectName(projectName: String): String? = VspProjectNameValidator.validate(projectName, isBinary)
}
