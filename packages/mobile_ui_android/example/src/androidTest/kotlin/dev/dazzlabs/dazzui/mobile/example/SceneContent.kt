package dev.dazzlabs.dazzui.mobile.example

import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsProperties
import org.json.JSONObject

/** Compare declared content independently of the chosen native view hierarchy. */
internal fun declaredText(state: SceneState): List<String> = buildList {
    fun visit(raw: Any) {
        if (raw is String) { if (raw.isNotEmpty()) add(raw); return }
        val node = raw as JSONObject
        val props = state.props(node)
        for (key in listOf("title", "description", "subtitle", "label", "hint", "meta", "shortcut")) {
            (props[key] as? String)?.takeIf { it.isNotEmpty() }?.let { add(it) }
        }
        when (node.getString("component")) {
            "Value" -> sceneText(state.evaluate(node.get("value"))).takeIf { it.isNotEmpty() }?.let { add(it) }
            "TextField" -> {
                val value = state.bound(node, "value", "") as String
                if (value.isNotEmpty()) add(value)
                else (props["placeholder"] as? String)?.takeIf { it.isNotEmpty() }?.let { add(it) }
            }
        }
        node.optJSONArray("children")?.items()?.forEach { visit(it) }
        node.optJSONObject("slots")?.entries()?.values?.forEach { visit(it) }
    }
    visit(state.scene.get("content"))
}

internal fun renderedText(node: SemanticsNode): List<String> = buildList {
    node.config.getOrElse(SemanticsProperties.Text) { emptyList() }.forEach { if (it.text.isNotEmpty()) add(it.text) }
    if (node.config.contains(SemanticsProperties.EditableText)) {
        node.config[SemanticsProperties.EditableText].text.takeIf { it.isNotEmpty() }?.let { add(it) }
    }
    node.children.forEach { addAll(renderedText(it)) }
}
