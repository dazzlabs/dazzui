package dev.dazzlabs.dazzui.mobile.example

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.foundation.background
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.automirrored.filled.OpenInNew
import androidx.compose.material.icons.filled.Close
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.unit.Dp
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Download
import androidx.compose.material.icons.filled.Circle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material.icons.filled.Search
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.key
import androidx.compose.ui.input.key.onPreviewKeyEvent
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
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
        // Emits its children straight into the parent layout while it holds.
        "Show" -> if (state.evaluate(node.get("value")) as Boolean) children.forEach { SceneNode(it, state) }
        "Tag" -> {
            val label: @Composable () -> Unit = { Text(children.filterIsInstance<String>().joinToString("")) }
            val removable = node.optJSONObject("on")?.has("remove") == true
            val remove: @Composable () -> Unit = {
                IconButton({ state.fire(node, "remove") }, Modifier.size(18.dp).semantics { contentDescription = p["removeLabel"] as? String ?: "Remove" }, enabled) {
                    Icon(Icons.Default.Close, contentDescription = null, modifier = Modifier.size(14.dp))
                }
            }
            val selectable = node.optJSONObject("bind")?.has("selected") == true || node.optJSONObject("on")?.has("select") == true
            if (selectable) {
                val selected = state.bound(node, "selected", p["selected"] ?: false) as Boolean
                FilterChip(selected, { state.change(node, "selected", !selected); state.fire(node, "select") }, label, mod, enabled,
                    trailingIcon = if (removable) remove else null)
            } else if (removable) InputChip(false, {}, label, mod, enabled, trailingIcon = remove)
            else SuggestionChip({}, label, mod, enabled)
        }
        "Link" -> TextButton({ state.fire(node, "press") }, mod, enabled) {
            Text(children.filterIsInstance<String>().joinToString(""),
                textDecoration = if (p["underline"] == "always") TextDecoration.Underline else null)
            if (p["external"] == true) Icon(Icons.AutoMirrored.Filled.OpenInNew, contentDescription = null, modifier = Modifier.size(14.dp))
        }
        "Meter" -> {
            val value = (p.getValue("value") as Number).toFloat()
            val min = (p["min"] as? Number)?.toFloat() ?: 0f
            val max = (p["max"] as? Number)?.toFloat() ?: 100f
            val fraction = ((value - min) / (max - min)).coerceIn(0f, 1f)
            Column(mod, verticalArrangement = gap) {
                if (p["label"] != null || p["showValue"] == true) Row(Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
                    Text(p["label"] as? String ?: "", style = MaterialTheme.typography.labelMedium)
                    if (p["showValue"] == true) Text(meterPercent(p), style = MaterialTheme.typography.labelMedium.copy(fontFamily = FontFamily.Monospace))
                }
                LinearProgressIndicator(progress = { fraction }, modifier = Modifier.fillMaxWidth())
            }
        }
        "Skeleton" -> {
            fun length(value: Any?): Dp? = when (value) {
                is Number -> value.toFloat().dp
                is String -> sceneSpacing[value]?.dp
                else -> null
            }
            val width = p["width"]
            val shape = when (p["shape"]) { "circle" -> CircleShape; "rect" -> MaterialTheme.shapes.small; else -> MaterialTheme.shapes.extraSmall }
            val side = if (p["shape"] == "circle") length(width) ?: length(p["height"]) ?: 40.dp else null
            val sized = when {
                side != null -> mod.size(side)
                width is String && width.endsWith("%") -> mod.fillMaxWidth(width.removeSuffix("%").toFloat() / 100f)
                else -> (length(width)?.let { mod.width(it) } ?: mod.fillMaxWidth()).height(length(p["height"]) ?: 16.dp)
            }
            Box(sized.background(MaterialTheme.colorScheme.surfaceVariant, shape))
        }
        "SkeletonText" -> Column(mod.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            val lines = (p["lines"] as? Number)?.toInt() ?: 3
            repeat(lines) { line ->
                Box(Modifier.fillMaxWidth(if (line + 1 == lines) 0.6f else 1f).height(16.dp)
                    .background(MaterialTheme.colorScheme.surfaceVariant, MaterialTheme.shapes.extraSmall))
            }
        }
        "IconButton" -> {
            val labelled = mod.semantics { contentDescription = p.getValue("label") as String }
            val press = { state.press(node) }
            when (p["variant"]) {
                "filled" -> FilledIconButton(press, labelled, enabled) { contents() }
                "tinted" -> FilledTonalIconButton(press, labelled, enabled) { contents() }
                "outlined" -> OutlinedIconButton(press, labelled, enabled) { contents() }
                else -> IconButton(press, labelled, enabled) { contents() }
            }
        }
        "Toggle" -> {
            val pressed = state.bound(node, "pressed", p["pressed"] ?: false) as Boolean
            val labelled = (p["label"] as? String)?.let { label -> mod.semantics { contentDescription = label } } ?: mod
            if (children.size == 1 && children.single() is JSONObject)
                IconToggleButton(pressed, { state.change(node, "pressed", it) }, labelled, enabled) { contents() }
            else FilterChip(pressed, { state.change(node, "pressed", !pressed) }, { Row { contents() } }, labelled, enabled)
        }
        "SegmentedControl" -> {
            val items = children.map { (it as JSONObject).getJSONObject("props") }
            val selected = state.bound(node, "value", p["value"] ?: "") as String
            SingleChoiceSegmentedButtonRow(mod.semantics { (p["label"] as? String)?.let { contentDescription = it } }
                .let { if (p["stretch"] == true) it.fillMaxWidth() else it }) {
                items.forEachIndexed { index, item ->
                    SegmentedButton(
                        selected = item.getString("value") == selected,
                        onClick = { state.change(node, "value", item.getString("value")) },
                        shape = SegmentedButtonDefaults.itemShape(index, items.size),
                        enabled = !item.optBoolean("disabled"),
                    ) { Text(item.getString("label")) }
                }
            }
        }
        "SegmentedItem", "RadioOption", "PillTab", "ToggleItem" -> error("$component is read by the group that holds it")
        "RadioGroup" -> {
            val selected = state.bound(node, "value", p["value"] ?: "") as String
            val options: @Composable () -> Unit = {
                children.map { (it as JSONObject).getJSONObject("props") }.forEach { option ->
                    val value = option.getString("value")
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        RadioButton(value == selected, { state.change(node, "value", value) }, enabled = !option.optBoolean("disabled"))
                        Text(option.getString("label"))
                    }
                }
            }
            val labelled = mod.selectableGroup().semantics { (p["label"] as? String)?.let { contentDescription = it } }
            if (p["direction"] == "row") FlowRow(labelled, horizontalArrangement = gap) { options() }
            else Column(labelled) { options() }
        }
        "PillTabs" -> {
            val selected = state.bound(node, "value", p["value"] ?: "") as String
            FlowRow(mod.semantics { (p["label"] as? String)?.let { contentDescription = it } }, horizontalArrangement = gap) {
                children.map { (it as JSONObject).getJSONObject("props") }.forEach { item ->
                    val value = item.getString("value")
                    FilterChip(
                        selected = value == selected, onClick = { state.change(node, "value", value) },
                        label = { Text(item.getString("label")); if (item.has("count")) Text(item.get("count").toString(), Modifier.padding(start = 6.dp), color = MaterialTheme.colorScheme.onSurfaceVariant) },
                        enabled = !item.optBoolean("disabled"),
                    )
                }
            }
        }
        "ToggleGroup" -> {
            val items = children.map { it as JSONObject }
            val single = p["multiple"] == false
            val bound = state.bound(node, "value", p["value"] ?: "") as String
            @Composable fun label(item: JSONObject) {
                val glyph = item.optJSONArray("children")?.items()?.firstOrNull { it is JSONObject } as? JSONObject
                val props = item.getJSONObject("props")
                if (glyph != null) Icon(sceneGlyphs.getValue(glyph.getJSONObject("props").getString("name")), contentDescription = props.getString("label"))
                else Text(props.getString("label"))
            }
            val labelled = mod.semantics { (p["label"] as? String)?.let { contentDescription = it } }
            if (single) SingleChoiceSegmentedButtonRow(labelled) {
                items.forEachIndexed { index, item ->
                    val value = item.getJSONObject("props").getString("value")
                    SegmentedButton(
                        selected = value == bound, onClick = { state.change(node, "value", if (value == bound) "" else value) },
                        shape = SegmentedButtonDefaults.itemShape(index, items.size),
                        enabled = !item.getJSONObject("props").optBoolean("disabled"),
                    ) { label(item) }
                }
            } else MultiChoiceSegmentedButtonRow(labelled) {
                items.forEachIndexed { index, item ->
                    val props = state.props(item)
                    SegmentedButton(
                        checked = state.bound(item, "pressed", props["pressed"] ?: false) as Boolean,
                        onCheckedChange = { state.change(item, "pressed", it) },
                        shape = SegmentedButtonDefaults.itemShape(index, items.size),
                        enabled = props["disabled"] != true,
                    ) { label(item) }
                }
            }
        }
        "OptionCard" -> {
            val selected = p["selected"] == true
            OutlinedCard(
                onClick = { state.fire(node, "press") }, enabled = enabled,
                modifier = mod.semantics { this.selected = selected },
                border = if (selected) CardDefaults.outlinedCardBorder().copy(brush = androidx.compose.ui.graphics.SolidColor(MaterialTheme.colorScheme.primary)) else CardDefaults.outlinedCardBorder(enabled),
            ) {
                Row(Modifier.padding(12.dp), horizontalArrangement = gap) {
                    RadioButton(selected, null, enabled = enabled)
                    Column {
                        ProvideTextStyle(MaterialTheme.typography.titleSmall) { if (slots?.has("title") == true) slot("title") else title() }
                        (p["description"] as? String)?.let { Text(it, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant) }
                    }
                }
            }
        }
        "Slider" -> {
            val min = (p["min"] as? Number)?.toFloat() ?: 0f
            val max = (p["max"] as? Number)?.toFloat() ?: 100f
            val step = (p["step"] as? Number)?.toFloat() ?: 1f
            val steps = maxOf(0, Math.round((max - min) / step) - 1)
            val first = (state.bound(node, "value", p["value"] ?: 0) as Number).toFloat()
            val end = (state.bound(node, "end", p["end"] ?: Double.NaN) as Number).toFloat().takeUnless { it.isNaN() }
            Row(mod, verticalAlignment = Alignment.CenterVertically, horizontalArrangement = gap) {
                val labelled = Modifier.weight(1f).semantics { (p["label"] as? String)?.let { contentDescription = it } }
                if (end == null) Slider(first, { state.change(node, "value", it.toDouble()) }, labelled, enabled, min..max, steps)
                else RangeSlider(first..end, { state.change(node, "value", it.start.toDouble()); state.change(node, "end", it.endInclusive.toDouble()) }, labelled, enabled, min..max, steps)
                // React prints the reading: a range as "a – b", a unit after one value.
                if (p["showValue"] == true) Text(
                    (p["unit"] as? String)?.let { sceneText(first.toDouble()) + it }
                        ?: listOfNotNull(first, end).joinToString(" – ") { sceneText(it.toDouble()) },
                )
            }
        }
        "NumberField" -> {
            val text = state.bound(node, "value", p["value"] ?: "") as String
            val min = (p["min"] as? Number)?.toDouble() ?: Double.NEGATIVE_INFINITY
            val max = (p["max"] as? Number)?.toDouble() ?: Double.POSITIVE_INFINITY
            val step = (p["step"] as? Number)?.toDouble() ?: 1.0
            val current = text.toDoubleOrNull()
            fun nudge(delta: Double) = state.change(node, "value", sceneText(((current ?: min.takeIf { it.isFinite() } ?: 0.0) + delta).coerceIn(min, max)))
            OutlinedTextField(
                value = text, onValueChange = { state.change(node, "value", it) }, modifier = mod,
                enabled = enabled, singleLine = true, isError = p["state"] == "error",
                placeholder = { Text(p["placeholder"] as? String ?: "") },
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                trailingIcon = {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                    (p["unit"] as? String)?.let { Text(it, color = MaterialTheme.colorScheme.onSurfaceVariant) }
                    Column {
                        IconButton({ nudge(step) }, Modifier.size(24.dp), enabled && (current ?: min) < max) { Icon(Icons.Filled.KeyboardArrowUp, "Increase") }
                        IconButton({ nudge(-step) }, Modifier.size(24.dp), enabled && (current ?: max) > min) { Icon(Icons.Filled.KeyboardArrowDown, "Decrease") }
                    }
                    }
                },
            )
        }
        "SearchField" -> {
            val text = state.bound(node, "value", "") as String
            OutlinedTextField(
                value = text, onValueChange = { state.change(node, "value", it) },
                // Escape clears first and dismisses only once nothing is left.
                modifier = mod.onPreviewKeyEvent { event ->
                    if (event.key != Key.Escape) return@onPreviewKeyEvent false
                    if (text.isEmpty()) state.fire(node, "dismiss") else state.change(node, "value", "")
                    true
                },
                singleLine = true, placeholder = { Text(p["placeholder"] as? String ?: "Search") },
                leadingIcon = { Icon(Icons.Filled.Search, null) },
                trailingIcon = if (text.isEmpty()) slots?.opt("hint")?.let { { slot("hint") } }
                    else { { IconButton({ state.change(node, "value", "") }) { Icon(Icons.Filled.Close, p["clearLabel"] as? String ?: "Clear search") } } },
            )
        }
        "OtpField" -> {
            val length = (p["length"] as? Number)?.toInt() ?: 6
            OutlinedTextField(
                value = state.bound(node, "value", p["value"] ?: "") as String,
                onValueChange = { next -> state.change(node, "value", next.filter { it.isDigit() }.take(length)) },
                modifier = mod, enabled = enabled, singleLine = true, isError = p["state"] == "error",
                placeholder = { Text("•".repeat(length)) },
                textStyle = MaterialTheme.typography.titleLarge.copy(fontFamily = FontFamily.Monospace),
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.NumberPassword),
                visualTransformation = if (p["mask"] == true) PasswordVisualTransformation() else VisualTransformation.None,
            )
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
        "Icon" -> Icon(
            sceneGlyphs.getValue(p.getValue("name") as String), contentDescription = null,
            modifier = when (val size = p["size"]) {
                // Mobile sizes a spacing step on the 4dp grid.
                is String -> mod.size((size.substringAfter('.').toInt() * 4).dp)
                is Number -> mod.size(size.toFloat().dp)
                else -> mod
            },
        )
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
        }.let { if (p["mono"] == true) it.copy(fontFamily = FontFamily.Monospace) else it }) {
            // Muted and subtle ink map to Material's two quieter content roles.
            val tone = when (p["tone"]) {
                "muted" -> MaterialTheme.colorScheme.onSurfaceVariant
                "subtle" -> MaterialTheme.colorScheme.outline
                else -> LocalContentColor.current
            }
            CompositionLocalProvider(LocalContentColor provides tone) { Row(mod) { contents() } }
        }
        "TableRow" -> Row(mod.fillMaxWidth(), horizontalArrangement = gap) { children.forEach { SceneNode(it, state, Modifier.weight(1f)) } }
        "TableCell" -> ProvideTextStyle(if (p["head"] == true) MaterialTheme.typography.titleSmall else MaterialTheme.typography.bodyMedium) {
            Row(mod, horizontalArrangement = when (p["align"]) { "end" -> Arrangement.End; "center" -> Arrangement.Center; else -> Arrangement.Start }) { contents() }
        }
        "Box" -> if (p["display"] != null && p["wrap"] == true) FlowRow(mod, horizontalArrangement = gap, verticalArrangement = gap) { contents() }
            else if (p["display"] != null) Row(if (hasVerticalDivider) mod.height(IntrinsicSize.Min) else mod, horizontalArrangement = if (p["justify"] == "between") Arrangement.SpaceBetween else gap) { contents() }
            else Column(if (p["padding"] != null) mod.padding(16.dp) else mod, verticalArrangement = gap) { contents() }
        "Col", "Stack", "StepList", "DialogBody", "DialogFooter", "Table", "TableHead" -> Column(mod, verticalArrangement = gap) { contents() }
        else -> error("Unsupported scene component: $component")
    }
}

/** The printed reading React shows: a whole percent of the meter's range. */
internal fun meterPercent(props: Map<String, Any>): String {
    val value = (props.getValue("value") as Number).toDouble()
    val min = (props["min"] as? Number)?.toDouble() ?: 0.0
    val max = (props["max"] as? Number)?.toDouble() ?: 100.0
    return "${Math.round((value - min) / (max - min) * 100)}%"
}
