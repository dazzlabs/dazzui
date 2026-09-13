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
        val component = node.getString("component")
        val glyph = node.optJSONArray("children")?.items()?.any { it is JSONObject } == true
        // A group's or a slider's label names it for accessibility, and a glyph
        // member's label is its content description; neither draws text.
        val keys = if (component in setOf("SegmentedControl", "RadioGroup", "PillTabs", "ToggleGroup", "Slider") ||
            (component == "ToggleItem" && glyph)) listOf("title") else
            listOf("title", "description", "subtitle", "label", "hint", "meta", "shortcut")
        for (key in keys) {
            (props[key] as? String)?.takeIf { it.isNotEmpty() }?.let { add(it) }
        }
        fun field(placeholder: String) {
            val value = state.bound(node, "value", props["value"] ?: "") as String
            value.ifEmpty { placeholder }.takeIf { it.isNotEmpty() }?.let { add(it) }
        }
        when (component) {
            "PillTab" -> props["count"]?.let { add(it.toString()) }
            "Slider" -> if (props["showValue"] == true) {
                val first = state.bound(node, "value", props["value"] ?: 0)
                val end = state.bound(node, "end", props["end"] ?: "")
                add((props["unit"] as? String)?.let { sceneText(first) + it }
                    ?: listOf(first, end).filter { it is Number }.joinToString(" – ") { sceneText((it as Number).toDouble()) })
            }
            "NumberField" -> {
                field(props["placeholder"] as? String ?: "")
                (props["unit"] as? String)?.let { add(it) }
            }
            "OtpField" -> field("•".repeat((props["length"] as? Number)?.toInt() ?: 6))
            "SearchField" -> {
                val value = state.bound(node, "value", "") as String
                add(value.ifEmpty { props["placeholder"] as? String ?: "Search" })
                if (value.isEmpty()) node.optJSONObject("slots")?.opt("hint")?.let { visit(it) }
                return
            }
            "Show" -> if (state.evaluate(node.get("value")) != true) return
            "Meter" -> if (props["showValue"] == true) add(meterPercent(props))
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
