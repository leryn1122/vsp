package io.github.leryn.vsp.ide.newProject

import com.intellij.ide.util.projectWizard.AbstractNewProjectStep
import com.intellij.ide.util.projectWizard.ProjectSettingsStepBase
import com.intellij.platform.DirectoryProjectGenerator

open class VspProjectSettingsStep(generator: DirectoryProjectGenerator<ConfigurationData>)
  : ProjectSettingsStepBase<ConfigurationData>(generator, AbstractNewProjectStep.AbstractCallback()) {
}
