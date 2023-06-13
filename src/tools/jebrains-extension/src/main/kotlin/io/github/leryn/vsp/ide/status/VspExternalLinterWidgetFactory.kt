package io.github.leryn.vsp.ide.status

import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.StatusBar
import com.intellij.openapi.wm.StatusBarWidget
import com.intellij.openapi.wm.StatusBarWidgetFactory

/**
 * {@link CustomStatusBarWidget}
 */
class VspExternalLinterWidgetFactory : StatusBarWidgetFactory {
  override fun getId(): String = VspExternalLinterWidget.ID

  override fun getDisplayName(): String = "Vespera External Linter"

  override fun isAvailable(project: Project): Boolean = true

  override fun createWidget(project: Project): StatusBarWidget = VspExternalLinterWidget()

  override fun disposeWidget(widget: StatusBarWidget) {
  }

  override fun canBeEnabledOn(statusBar: StatusBar): Boolean = true
}
