package dev.dazzlabs.dazzui.mobile.example

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Download
import androidx.compose.material.icons.filled.Circle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.isActive
import org.json.JSONObject

@Composable
fun GeneratedScene(state: SceneState, running: Boolean = true) {
    if (running && (state.scene.optJSONArray("timers")?.length() ?: 0) > 0) {
        LaunchedEffect(state) {
            var previous = withFrameNanos { it }
            var remainder = 0L
            while (isActive) {
                val now = withFrameNanos { it }
                val delta = now - previous + remainder
                state.advance(delta / 1_000_000)
                remainder = delta % 1_000_000
                previous = now
            }
        }
    }
    SceneNode(state.scene.get("content"), state)
}

/** Only native Material controls and Compose layout primitives live here. */
@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun SceneNode(raw: Any, state: SceneState, modifier: Modifier = Modifier) {
    if (raw is String) { Text(raw, modifier); return }
    val node = raw as JSONObject
    val component = node.getString("component")
    val p = state.props(node)
    val children = node.optJSONArray("children")?.items() ?: emptyList()
    val slots = node.optJSONObject("slots")
    val tag = node.optString("id")
    val mod = if (tag.isEmpty()) modifier else modifier.testTag(tag)
    val enabled = p["disabled"] != true
    val gap = Arrangement.spacedBy(8.dp)
    val hasVerticalDivider = children.any { (it as? JSONObject)?.let { child -> child.optString("component") == "Divider" && child.optJSONObject("props")?.optString("orientation") == "vertical" } == true }
    @Composable fun contents() { children.forEach { SceneNode(it, state) } }
    @Composable fun slot(name: String) { slots?.opt(name)?.let { SceneNode(it, state) } }
    @Composable fun title(key: String = "title") { (p[key] as? String)?.let { Text(it, style = MaterialTheme.typography.titleMedium) } }
    when (component) {
        "Button" -> {
            val label: @Composable RowScope.() -> Unit = {
                contents()
                (p["shortcut"] as? String)?.let { Text(it, style = MaterialTheme.typography.labelSmall) }
            }
            when (p["variant"]) {
                "filled" -> Button(onClick = { state.press(node) }, enabled = enabled, modifier = mod, content = label)
                "outlined" -> OutlinedButton(onClick = { state.press(node) }, enabled = enabled, modifier = mod, content = label)
                "plain", "normal" -> TextButton(onClick = { state.press(node) }, enabled = enabled, modifier = mod, content = label)
                else -> FilledTonalButton(onClick = { state.press(node) }, enabled = enabled, modifier = mod, content = label)
            }
        }
        "Switch", "Checkbox" -> {
            val checked = state.bound(node, "checked", p["checked"] ?: false) as Boolean
            Row(verticalAlignment = Alignment.CenterVertically) {
                if (component == "Switch") Switch(checked, { state.change(node, "checked", it) }, mod, enabled = enabled)
                else Checkbox(checked, { state.change(node, "checked", it) }, mod, enabled = enabled)
                (p["label"] as? String)?.let { Text(it) }
                contents()
            }
        }
        "TextField" -> OutlinedTextField(
            value = state.bound(node, "value", "") as String,
            onValueChange = { state.change(node, "value", it) }, modifier = mod,
            enabled = enabled, readOnly = p["readOnly"] == true, singleLine = true,
            isError = p["state"] == "error", placeholder = { Text(p["placeholder"] as? String ?: "") },
            textStyle = MaterialTheme.typography.bodyLarge.copy(fontFamily = if (p["mono"] == true) FontFamily.Monospace else FontFamily.Default),
        )
        "Value" -> Text(sceneText(state.evaluate(node.get("value"))), mod)
        "Text" -> Text(children.single() as String, mod)
        "Icon" -> Icon(when (p["name"]) {
            "ArrowDownload20Regular" -> Icons.Default.Download
            "Circle20Filled" -> Icons.Default.Circle
            else -> error("Unknown icon")
        }, contentDescription = null, modifier = mod)
        "Spinner" -> CircularProgressIndicator(mod, color = if (p["onAccent"] == true) LocalContentColor.current else MaterialTheme.colorScheme.primary)
        "Progress" -> if (p["value"] is Number) LinearProgressIndicator(progress = { (p.getValue("value") as Number).toFloat() / 100f }, modifier = mod)
            else LinearProgressIndicator(mod)
        "Divider" -> if (p["orientation"] == "vertical") VerticalDivider(mod.fillMaxHeight()) else HorizontalDivider(mod)
        "Badge" -> Badge(mod) { contents() }
        "KeyCap" -> ProvideTextStyle(MaterialTheme.typography.labelSmall.copy(fontFamily = FontFamily.Monospace)) { Row(mod) { contents() } }
        "SectionLabel" -> ProvideTextStyle(MaterialTheme.typography.titleSmall) { Row(mod) { contents() } }
        "Card" -> Card(mod) { Column(Modifier.padding(16.dp), verticalArrangement = gap) { contents() } }
        "Callout" -> Card(mod) { Column(Modifier.padding(16.dp), verticalArrangement = gap) { slot("icon"); title(); contents(); slot("action") } }
        "EmptyState" -> Column(mod, verticalArrangement = gap) { title(); slot("action") }
        "Step" -> ListItem(
            modifier = mod,
            headlineContent = { Text(p.getValue("label") as String) },
            supportingContent = { Text(p["meta"] as? String ?: "") },
            leadingContent = { if (p["status"] == "done") Checkbox(true, null) else RadioButton(selected = p["status"] == "active", onClick = null) },
        )
        "Page", "Section" -> Column(mod, verticalArrangement = gap) { title(); (p["description"] as? String)?.let { Text(it) }; contents() }
        "Specimen" -> Column(mod, verticalArrangement = gap) { Text(p["label"] as? String ?: "", style = MaterialTheme.typography.labelMedium); if (hasVerticalDivider) Row(Modifier.height(IntrinsicSize.Min)) { contents() } else contents() }
        "PreferenceSection" -> Column(mod, verticalArrangement = gap) { title("label"); contents() }
        "PreferenceRow" -> Column(mod, verticalArrangement = gap) { title(); (p["subtitle"] as? String)?.let { Text(it) }; contents() }
        "FormField" -> Column(mod, verticalArrangement = gap) {
            title("label"); contents()
            Text(p["hint"] as? String ?: "", color = if (p["state"] == "error") MaterialTheme.colorScheme.error else MaterialTheme.colorScheme.onSurfaceVariant)
        }
        "Dialog" -> if (state.bound(node, "open", false) as Boolean) {
            fun part(name: String) = children.filter { (it as? JSONObject)?.optString("component") == name }
            AlertDialog(
                onDismissRequest = { state.change(node, "open", false) }, modifier = mod.testTag("scene-dialog"),
                title = { part("DialogHeader").forEach { SceneNode(it, state) } },
                text = { part("DialogBody").forEach { SceneNode(it, state) } },
                confirmButton = { part("DialogFooter").forEach { SceneNode(it, state) } },
            )
        }
        "DialogHeader" -> Column(mod) { title(); (p["subtitle"] as? String)?.let { Text(it) } }
        "ActionBar" -> Row((if (hasVerticalDivider) mod.height(IntrinsicSize.Min) else mod).fillMaxWidth(), horizontalArrangement = when (p["align"]) {
            "end" -> Arrangement.End; "center" -> Arrangement.Center; "between" -> Arrangement.SpaceBetween; else -> gap
        }, verticalAlignment = Alignment.CenterVertically) { contents() }
        "Grid" -> BoxWithConstraints(mod.fillMaxWidth()) {
            val width = maxWidth
            val columns = maxOf(1, ((width.value + 8) / 168).toInt())
            FlowRow(horizontalArrangement = gap, verticalArrangement = gap) {
                children.forEach { SceneNode(it, state, Modifier.width((width - 8.dp * (columns - 1)) / columns)) }
            }
        }
        "Row" -> if (hasVerticalDivider) Row(mod.height(IntrinsicSize.Min), horizontalArrangement = gap) { contents() } else FlowRow(mod, horizontalArrangement = gap, verticalArrangement = gap) { contents() }
        "Inline" -> ProvideTextStyle(when (p["fontSize"]) {
            "title.large" -> MaterialTheme.typography.headlineLarge
            "body.small" -> MaterialTheme.typography.bodySmall
            else -> LocalTextStyle.current
        }.let { if (p["mono"] == true) it.copy(fontFamily = FontFamily.Monospace) else it }) { Row(mod) { contents() } }
        "TableRow" -> Row(mod.fillMaxWidth(), horizontalArrangement = gap) { children.forEach { SceneNode(it, state, Modifier.weight(1f)) } }
        "TableCell" -> ProvideTextStyle(if (p["head"] == true) MaterialTheme.typography.titleSmall else MaterialTheme.typography.bodyMedium) {
            Row(mod, horizontalArrangement = when (p["align"]) { "end" -> Arrangement.End; "center" -> Arrangement.Center; else -> Arrangement.Start }) { contents() }
        }
        "Box" -> if (p["display"] != null) Row(if (hasVerticalDivider) mod.height(IntrinsicSize.Min) else mod, horizontalArrangement = if (p["justify"] == "between") Arrangement.SpaceBetween else gap) { contents() }
            else Column(if (p["padding"] != null) mod.padding(16.dp) else mod, verticalArrangement = gap) { contents() }
        "Col", "Stack", "StepList", "DialogBody", "DialogFooter", "Table", "TableHead" -> Column(mod, verticalArrangement = gap) { contents() }
        else -> error("Unsupported scene component: $component")
    }
}
