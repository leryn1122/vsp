package io.github.leryn.vsp.ide.template

import com.intellij.codeInsight.template.TemplateActionContext
import com.intellij.codeInsight.template.TemplateContextType
import kotlin.reflect.KClass

sealed class VspContextType(id: String, presentableName: String, baseContextType: KClass<out TemplateContextType>)
  : TemplateContextType(id, presentableName, baseContextType.java) {

  final override fun isInContext(templateActionContext: TemplateActionContext): Boolean {
    return super.isInContext(templateActionContext)
  }
}
