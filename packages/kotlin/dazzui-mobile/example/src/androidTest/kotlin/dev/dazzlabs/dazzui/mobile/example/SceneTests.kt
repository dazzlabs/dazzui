package dev.dazzlabs.dazzui.mobile.example

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Surface
import androidx.compose.material3.MaterialTheme
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toPixelMap
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.unit.dp
import androidx.test.platform.app.InstrumentationRegistry
import dev.dazzlabs.dazzui.mobile.DazzTheme
import dev.dazzlabs.dazzui.mobile.generated.DazzThemeFamily
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assert.*
import org.junit.Rule
import org.junit.Test

class SceneTests {
    @get:Rule val compose = createComposeRule()
    private fun stories(): List<JSONObject> = JSONArray(InstrumentationRegistry.getInstrumentation().targetContext.assets.open("scenes.json").bufferedReader().use { it.readText() }).items().map { it as JSONObject }
    private fun text(node: SemanticsNode): String = (node.config.getOrElse(SemanticsProperties.Text) { emptyList() }.joinToString("") { it.text }) + node.children.joinToString("") { text(it) }
    private fun assertValue(expected: Any, actual: Any?) {
        if (expected is Number && actual is Number) assertEquals(expected.toDouble(), actual.toDouble(), 0.0)
        else assertEquals(expected, actual)
    }

    @Test fun sharedCheckpointsDriveNativeControls() {
        var current by mutableStateOf<SceneState?>(null)
        compose.setContent { DazzTheme { Surface { current?.let { state -> key(state) { Column(Modifier.verticalScroll(rememberScrollState()).padding(16.dp)) { GeneratedScene(state, running = false) } } } } } }
        for (story in stories()) {
            val scene = story.getJSONObject("scene")
            for (raw in scene.optJSONArray("checkpoints")?.items() ?: emptyList()) {
                val checkpoint = raw as JSONObject
                val state = SceneState(scene)
                compose.runOnIdle { current = state }
                for (rawStep in checkpoint.getJSONArray("steps").items()) {
                    val step = rawStep as JSONObject
                    when (step.getString("action")) {
                        "click" -> compose.onNodeWithTag(step.getString("target"), useUnmergedTree = true).performClick()
                        "fill" -> compose.onNodeWithTag(step.getString("target"), useUnmergedTree = true).performTextReplacement(step.getString("value"))
                        "advance" -> compose.runOnIdle { state.advance(step.getLong("milliseconds")) }
                        else -> error("Unsupported checkpoint action: $step")
                    }
                }
                compose.runOnIdle {
                    checkpoint.optJSONObject("expect")?.entries()?.forEach { (key, expected) -> assertValue(expected, state.values[key]) }
                }
                checkpoint.optJSONObject("expectDisabled")?.entries()?.forEach { (target, disabled) ->
                    val node = compose.onNodeWithTag(target, useUnmergedTree = true)
                    if (disabled as Boolean) node.assertIsNotEnabled() else node.assertIsEnabled()
                }
                checkpoint.optJSONObject("expectText")?.entries()?.forEach { (target, expected) ->
                    assertEquals("${story.getString("id")} / ${checkpoint.getString("id")} / $target", expected, text(compose.onNodeWithTag(target, useUnmergedTree = true).fetchSemanticsNode()))
                }
                if (scene.getJSONObject("content").getString("component") == "Dialog") {
                    val key = scene.getJSONObject("content").getJSONObject("bind").getString("open")
                    if (state.values[key] == true) compose.onNodeWithTag("scene-dialog").assertExists() else compose.onNodeWithTag("scene-dialog").assertDoesNotExist()
                }
            }
        }
    }

    @Test fun everySceneRendersInEveryTheme() {
        var current by mutableStateOf<SceneState?>(null)
        var family by mutableStateOf(DazzThemeFamily.Bright)
        var dark by mutableStateOf(false)
        compose.setContent { DazzTheme(family, dark) { Surface { current?.let { state -> key(state) { Column(Modifier.fillMaxSize().verticalScroll(rememberScrollState()).padding(16.dp)) { GeneratedScene(state, running = false) } } } } } }
        val stories = stories()
        assertEquals(stories.size, stories.map { it.getString("id") }.toSet().size)
        for (theme in DazzThemeFamily.entries) for (appearance in listOf(false, true)) for (story in stories) {
            compose.runOnIdle { family = theme; dark = appearance; current = SceneState(story.getJSONObject("scene")) }
            compose.waitForIdle()
            val roots = compose.onAllNodes(isRoot(), useUnmergedTree = true).fetchSemanticsNodes()
            assertEquals("Declared content: ${story.getString("id")} / $theme / $appearance",
                declaredText(current!!).groupingBy { it }.eachCount(),
                roots.flatMap { renderedText(it) }.groupingBy { it }.eachCount())
            // Dialog creates a separate native root; capture each visible window.
            roots.forEach { root ->
                val image = compose.onNode(SemanticsMatcher("root ${root.id}") { it.id == root.id }).captureToImage()
                assertTrue("${story.getString("id")} / $theme / $appearance", image.width > 0 && image.height > 0)
            }
        }
    }


    @Test fun verticalRulesFollowNativeControlHeight() {
        val state = SceneState(JSONObject("""{"content":{"component":"Box","id":"row","props":{"display":"flex"},"children":[{"component":"Button","id":"control","children":["Control"]},{"component":"Divider","id":"rule","props":{"orientation":"vertical"}}]}}"""))
        compose.setContent { DazzTheme { GeneratedScene(state, running = false) } }
        val rule = compose.onNodeWithTag("rule").fetchSemanticsNode().boundsInRoot.height
        val control = compose.onNodeWithTag("control").fetchSemanticsNode().boundsInRoot.height
        assertTrue(rule > 0f)
        val row = compose.onNodeWithTag("row", useUnmergedTree = true).fetchSemanticsNode().boundsInRoot.height
        assertEquals(row, rule, 0f)
        assertTrue(rule >= control)
    }

    @Test fun spinnerRemainsVisibleOnNativeFilledButton() {
        val state = SceneState(JSONObject("""{"content":{"component":"Button","props":{"variant":"filled"},"children":[{"component":"Spinner","id":"spinner","props":{"onAccent":true}},"Loading"]}}"""))
        var ink = Color.Unspecified
        compose.mainClock.autoAdvance = false
        compose.setContent { DazzTheme { ink = MaterialTheme.colorScheme.onPrimary; GeneratedScene(state, running = false) } }
        compose.mainClock.advanceTimeBy(1000)
        val pixels = compose.onNodeWithTag("spinner", useUnmergedTree = true).captureToImage().toPixelMap()
        var count = 0
        for (y in 0 until pixels.height) for (x in 0 until pixels.width) {
            val color = pixels[x, y]
            if (kotlin.math.abs(color.red - ink.red) < 0.05 && kotlin.math.abs(color.green - ink.green) < 0.05 && kotlin.math.abs(color.blue - ink.blue) < 0.05) count++
        }
        assertTrue("The indicator must paint the native button's content color", count > 0)
    }

    @Test fun timersAreChronologicalAndUnmountCancelsFrameUpdates() {
        val scene = JSONObject("""{"state":{"value":0},"timers":[{"every":100,"set":"value","value":{"add":[{"state":"value"},1]}},{"every":250,"set":"value","value":{"multiply":[{"state":"value"},2]}}],"content":{"component":"Value","value":{"state":"value"}}}""")
        val manual = SceneState(scene)
        manual.advance(500)
        assertValue(14, manual.values["value"])
        var shown by mutableStateOf(true)
        val live = SceneState(scene)
        compose.mainClock.autoAdvance = false
        compose.setContent { if (shown) GeneratedScene(live) }
        compose.mainClock.advanceTimeBy(550)
        compose.runOnIdle { assertTrue((live.values.getValue("value") as Number).toDouble() > 0); shown = false }
        compose.mainClock.advanceTimeByFrame()
        val stopped = live.values.getValue("value")
        compose.mainClock.advanceTimeBy(1000)
        compose.runOnIdle { assertValue(stopped, live.values.getValue("value")) }
    }
}
