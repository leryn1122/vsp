package io.github.leryn.vsp.ide.newProject

import com.intellij.ide.wizard.AbstractNewProjectWizardStep
import com.intellij.ide.wizard.LanguageNewProjectWizard
import com.intellij.ide.wizard.NewProjectWizardLanguageStep
import com.intellij.ide.wizard.NewProjectWizardStep
import com.intellij.openapi.project.Project
import com.intellij.ui.dsl.builder.Panel
import com.intellij.ui.dsl.gridLayout.HorizontalAlign
import io.github.leryn.vsp.support.toPathOrNull
import java.nio.file.Paths

/**
 * Vespera new project wizard.
 */
class VspNewProjectWizard : LanguageNewProjectWizard {

  override val name: String = "Vespera"

  override fun createStep(parent: NewProjectWizardLanguageStep): NewProjectWizardStep = Step(parent)

  private class Step(parent: NewProjectWizardLanguageStep) : AbstractNewProjectWizardStep(parent) {

    private val peer: VspProjectGeneratorPeer = VspProjectGeneratorPeer(parent.path.toPathOrNull() ?: Paths.get("."));

    override fun setupProject(project: Project) {
    }

    override fun setupUI(builder: Panel) {
      with(builder) {
        row {
          cell(peer.component)
            .horizontalAlign(HorizontalAlign.FILL)
        }
      }
    }
  }

}
