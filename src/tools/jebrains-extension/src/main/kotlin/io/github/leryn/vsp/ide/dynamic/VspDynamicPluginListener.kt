package io.github.leryn.vsp.ide.dynamic

import com.intellij.ide.plugins.CannotUnloadPluginException
import com.intellij.ide.plugins.DynamicPluginListener
import com.intellij.ide.plugins.IdeaPluginDescriptor
import com.intellij.openapi.extensions.PluginId
import io.github.leryn.vsp.VspConstants

class VspDynamicPluginListener : DynamicPluginListener {
  override fun checkUnloadPlugin(pluginDescriptor: IdeaPluginDescriptor) {
    if (pluginDescriptor.pluginId == PluginId.findId(VspConstants.PLUGIN_ID)) {
      throw CannotUnloadPluginException("Vespera plugin cannot be dynamically unloaded for now")
    }
  }
}
