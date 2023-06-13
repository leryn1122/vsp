package io.github.leryn.vsp.project.configurable

import com.intellij.openapi.options.BoundConfigurable
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.NlsContexts.ConfigurableName

@Suppress("UnstableApiUsage")
abstract class VspConfigurableBase(
  protected val project: Project,
  @ConfigurableName displayName: String
) : BoundConfigurable(displayName) {
}
