package io.github.leryn.vsp.project.icons

import com.intellij.ide.FileIconProvider
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import javax.swing.Icon

class SourceIconProvider : FileIconProvider {
  override fun getIcon(file: VirtualFile, flags: Int, project: Project?): Icon? = when (file.name) {
//      ProjectConstants.MANIFEST_FILE -> Icons.MANIFEST_FILE
//      ProjectConstants.LOCK_FILE -> Icons.LOCK_FILE
    else -> null
  }
}
