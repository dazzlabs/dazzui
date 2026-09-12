package dev.dazzlabs.dazzui.mobile.example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import dev.dazzlabs.dazzui.mobile.DazzTheme
import dev.dazzlabs.dazzui.mobile.generated.DazzThemeFamily

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent { Playground() }
    }
}

// These are example screen spacing, not replacements for component metrics.
private val ScreenPadding = 16.dp
private val SectionGap = 12.dp

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun Playground() {
    var family by rememberSaveable { mutableStateOf(DazzThemeFamily.Studio) }
    val systemDark = isSystemInDarkTheme()
    var followSystem by rememberSaveable { mutableStateOf(true) }
    var dark by rememberSaveable { mutableStateOf(systemDark) }
    var menuOpen by remember { mutableStateOf(false) }
    var dialogOpen by rememberSaveable { mutableStateOf(false) }
    var checked by rememberSaveable { mutableStateOf(true) }
    var name by rememberSaveable { mutableStateOf("") }
    var progress by rememberSaveable { mutableFloatStateOf(0.5f) }
    var saved by rememberSaveable { mutableStateOf(false) }

    DazzTheme(family, if (followSystem) systemDark else dark) {
        Scaffold(topBar = { TopAppBar(title = { Text("DazzUI Mobile") }) }) { insets ->
            Column(
                Modifier.fillMaxSize().padding(insets).verticalScroll(rememberScrollState())
                    .padding(ScreenPadding),
                verticalArrangement = Arrangement.spacedBy(SectionGap),
            ) {
                Text("Theme", style = MaterialTheme.typography.titleLarge)
                Column {
                    OutlinedButton(onClick = { menuOpen = true }) { Text(family.name) }
                    DropdownMenu(expanded = menuOpen, onDismissRequest = { menuOpen = false }) {
                        DazzThemeFamily.entries.forEach { option ->
                            DropdownMenuItem(
                                text = { Text(option.name) },
                                onClick = { family = option; menuOpen = false },
                            )
                        }
                    }
                }
                ListItem(
                    headlineContent = { Text("Follow system appearance") },
                    trailingContent = { Switch(checked = followSystem, onCheckedChange = { followSystem = it }) },
                )
                ListItem(
                    headlineContent = { Text("Dark theme") },
                    trailingContent = {
                        Switch(checked = if (followSystem) systemDark else dark,
                            enabled = !followSystem, onCheckedChange = { dark = it })
                    },
                )
                HorizontalDivider()
                Text("Components", style = MaterialTheme.typography.titleLarge)
                Card(Modifier.fillMaxWidth()) {
                    Column(Modifier.padding(ScreenPadding), verticalArrangement = Arrangement.spacedBy(SectionGap)) {
                        Text("Material 3, in DazzUI colors", style = MaterialTheme.typography.titleMedium)
                        Text("Standard Android components, using the selected ds palette.")
                    }
                }
                OutlinedTextField(value = name, onValueChange = { name = it; saved = false },
                    label = { Text("Project name") }, singleLine = true, modifier = Modifier.fillMaxWidth())
                Row(horizontalArrangement = Arrangement.spacedBy(SectionGap)) {
                    Button(onClick = { saved = true }, enabled = name.isNotBlank()) { Text("Save") }
                    OutlinedButton(onClick = { dialogOpen = true }) { Text("Open dialog") }
                }
                if (saved) Text("Saved: $name", color = MaterialTheme.colorScheme.primary)
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Checkbox(checked = checked, onCheckedChange = { checked = it })
                    Text("Enable notifications")
                }
                FilterChip(selected = checked, onClick = { checked = !checked }, label = { Text("Notifications") })
                Slider(value = progress, onValueChange = { progress = it })
                LinearProgressIndicator(progress = { progress }, modifier = Modifier.fillMaxWidth())
                Text("Error message", color = MaterialTheme.colorScheme.error)
            }
        }
        if (dialogOpen) {
            AlertDialog(
                onDismissRequest = { dialogOpen = false },
                title = { Text("System components") },
                text = { Text("This dialog uses Material 3 defaults and the current DazzUI color scheme.") },
                confirmButton = { TextButton(onClick = { dialogOpen = false }) { Text("Done") } },
            )
        }
    }
}
