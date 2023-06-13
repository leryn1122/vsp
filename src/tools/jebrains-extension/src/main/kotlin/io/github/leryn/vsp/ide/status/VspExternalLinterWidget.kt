package io.github.leryn.vsp.ide.status

import com.intellij.openapi.wm.CustomStatusBarWidget
import com.intellij.openapi.wm.StatusBar
import com.intellij.openapi.wm.impl.status.TextPanel
import com.intellij.ui.ClickListener
import com.intellij.util.ui.JBUI
import com.intellij.util.ui.UIUtil
import java.awt.event.MouseEvent
import javax.swing.JComponent

class VspExternalLinterWidget : TextPanel.WithIconAndArrows(), CustomStatusBarWidget {

  companion object {
    const val ID: String = "vspExternalLinterWidget"
  }

  private var statusBar: StatusBar? = null

  private var enabled: Boolean = false

  init {
    setTextAlignment(CENTER_ALIGNMENT)
    border = JBUI.CurrentTheme.StatusBar.Widget.border()
  }

  override fun ID(): String = "vspExternalLinterWidget"

  override fun dispose() {
    this.statusBar = null
    UIUtil.dispose(this)
  }

  override fun install(statusBar: StatusBar) {
    this.statusBar = statusBar

    object : ClickListener() {
      override fun onClick(event: MouseEvent, clickCount: Int): Boolean {
        return true
      }
    }.installOn(this, true)
  }

  override fun getComponent(): JComponent = this

}
