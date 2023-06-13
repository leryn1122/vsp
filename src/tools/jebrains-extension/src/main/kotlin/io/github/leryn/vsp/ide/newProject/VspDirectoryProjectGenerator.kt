package io.github.leryn.vsp.ide.newProject

import com.intellij.facet.ui.ValidationResult
import com.intellij.ide.util.projectWizard.AbstractNewProjectStep
import com.intellij.ide.util.projectWizard.CustomStepProjectGenerator
import com.intellij.openapi.module.Module
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.openapi.wm.impl.welcomeScreen.AbstractActionWithPanel
import com.intellij.platform.DirectoryProjectGenerator
import com.intellij.platform.DirectoryProjectGeneratorBase
import io.github.leryn.vsp.ide.icons.Icons
import java.nio.file.Paths
import javax.swing.Icon

class VspDirectoryProjectGenerator
  : DirectoryProjectGeneratorBase<ConfigurationData>(), CustomStepProjectGenerator<ConfigurationData> {

  private var peer: VspProjectGeneratorPeer? = null
  override fun getName(): String = "Vespera"

  override fun getLogo(): Icon = Icons.LOGO

  override fun validate(baseDirPath: String): ValidationResult {
    val moduleName = Paths.get(baseDirPath).fileName
//    val message = peer?.settings?.template
    val message = null
    return ValidationResult(message)
  }

  override fun generateProject(project: Project, baseDir: VirtualFile, data: ConfigurationData, module: Module) {
    val (_, settings) = data
//    val cargo = settings.toolchain?.cargo() ?: return

    val name = VspProjectNameValidator.normalize(project.name)
  }

  override fun createStep(projectGenerator: DirectoryProjectGenerator<ConfigurationData>,
                          callback: AbstractNewProjectStep.AbstractCallback<ConfigurationData>?
  ): AbstractActionWithPanel = VspProjectSettingsStep(projectGenerator)

}
