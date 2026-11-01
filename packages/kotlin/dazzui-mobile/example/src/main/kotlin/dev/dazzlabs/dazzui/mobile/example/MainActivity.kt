package dev.dazzlabs.dazzui.mobile.example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import dev.dazzlabs.dazzui.mobile.DazzTheme
import dev.dazzlabs.dazzui.mobile.generated.DazzThemeFamily
import org.json.JSONArray
import org.json.JSONObject

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent { Playground(intent.getStringExtra("story")) }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun Playground(initialStory: String?) {
    val context = LocalContext.current
    val stories = remember { JSONArray(context.assets.open("scenes.json").bufferedReader().use { it.readText() }).items().map { it as JSONObject } }
    var family by rememberSaveable { mutableStateOf(DazzThemeFamily.Bright) }
    val systemDark = isSystemInDarkTheme()
    var followSystem by rememberSaveable { mutableStateOf(true) }
    var dark by rememberSaveable { mutableStateOf(systemDark) }
    var menuOpen by remember { mutableStateOf(false) }
    var selected by rememberSaveable { mutableStateOf(initialStory) }
    var search by rememberSaveable { mutableStateOf("") }
    val story = stories.firstOrNull { it.getString("id") == selected }
    BackHandler(story != null) { selected = null }
    DazzTheme(family, if (followSystem) systemDark else dark) {
        Scaffold(topBar = { TopAppBar(title = { Text(story?.getString("name") ?: "Playground") }, navigationIcon = {
            if (story != null) TextButton(onClick = { selected = null }) { Text("Back") }
        }) }) { insets ->
            Column(Modifier.fillMaxSize().padding(insets).verticalScroll(rememberScrollState()).padding(16.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
                if (story != null) {
                    key(selected) { GeneratedScene(remember { SceneState(story.getJSONObject("scene")) }) }
                } else {
                    Box {
                        OutlinedButton(onClick = { menuOpen = true }) { Text(family.name) }
                        DropdownMenu(expanded = menuOpen, onDismissRequest = { menuOpen = false }) {
                            DazzThemeFamily.entries.forEach { option -> DropdownMenuItem(text = { Text(option.name) }, onClick = { family = option; menuOpen = false }) }
                        }
                    }
                    ListItem(headlineContent = { Text("Follow system appearance") }, trailingContent = { Switch(followSystem, { followSystem = it }) })
                    ListItem(headlineContent = { Text("Dark theme") }, trailingContent = { Switch(if (followSystem) systemDark else dark, { dark = it }, enabled = !followSystem) })
                    OutlinedTextField(search, { search = it }, label = { Text("Find a story") }, modifier = Modifier.fillMaxWidth())
                    stories.filter { (it.getString("title") + " " + it.getString("name")).contains(search, ignoreCase = true) }.groupBy { it.getString("title") }.forEach { (title, entries) ->
                        Text(title, style = MaterialTheme.typography.titleMedium)
                        entries.forEach { item -> TextButton(onClick = { selected = item.getString("id") }) { Text(item.getString("name")) } }
                    }
                }
            }
        }
    }
}
