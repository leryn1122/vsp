package io.github.leryn.vsp.lang

import com.intellij.openapi.fileTypes.LanguageFileType
import com.intellij.openapi.vfs.VirtualFile
import io.github.leryn.vsp.ide.icons.Icons
import java.nio.charset.StandardCharsets
import javax.swing.Icon

object VspFileType : LanguageFileType(VspLanguage) {

  override fun getName(): String = "Vespera"

  override fun getDescription(): String = "Vespera files"

  override fun getDefaultExtension(): String = "vsp"

  override fun getIcon(): Icon = Icons.VSP_FILE

  override fun getCharset(file: VirtualFile, content: ByteArray): String = StandardCharsets.UTF_8.name()
}
