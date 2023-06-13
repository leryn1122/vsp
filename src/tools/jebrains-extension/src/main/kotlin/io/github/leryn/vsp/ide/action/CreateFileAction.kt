package io.github.leryn.vsp.ide.action

import com.intellij.ide.actions.CreateFileFromTemplateAction
import com.intellij.ide.actions.CreateFileFromTemplateDialog
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.actionSystem.DataContext
import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.project.Project
import com.intellij.psi.PsiDirectory
import io.github.leryn.vsp.ide.icons.Icons

class CreateFileAction
  : CreateFileFromTemplateAction(CAPTION, "", Icons.VSP_FILE)
, DumbAware {

  private companion object {
    private const val CAPTION = "Vespera File"
  }

  override fun isAvailable(dataContext: DataContext): Boolean {
    if (!super.isAvailable(dataContext)) {
      return false
    }
    val project = CommonDataKeys.PROJECT.getData(dataContext) ?: return false
    val virtualFile = CommonDataKeys.VIRTUAL_FILE.getData(dataContext) ?: return false

    // TODO
    return true
  }

  override fun buildDialog(project: Project, directory: PsiDirectory, builder: CreateFileFromTemplateDialog.Builder) {
    builder.setTitle(CAPTION)
      .addKind("Empty File", Icons.VSP_FILE, "Vespera File")
  }

  override fun getActionName(directory: PsiDirectory?, newName: String, templateName: String?): String = CAPTION

}
