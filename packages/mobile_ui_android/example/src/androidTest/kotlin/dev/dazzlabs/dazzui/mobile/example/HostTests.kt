package dev.dazzlabs.dazzui.mobile.example

import androidx.compose.ui.test.*
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Rule
import org.junit.Test

class HostTests {
    @get:Rule val compose = createAndroidComposeRule<MainActivity>()

    @Test fun catalogOpensSharedContentAndRevisitingResetsState() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val stories = JSONArray(context.assets.open("scenes.json").bufferedReader().use { it.readText() }).items().map { it as JSONObject }
        // Pick an editable scene from declarations, with no native fixture values.
        fun fills(scene: JSONObject): List<JSONObject> = (scene.optJSONArray("checkpoints")?.items() ?: emptyList()).flatMap { (it as JSONObject).getJSONArray("steps").items() }.map { it as JSONObject }.filter { it.getString("action") == "fill" }
        fun nodes(raw: Any): List<JSONObject> = if (raw !is JSONObject) emptyList() else listOf(raw) + (raw.optJSONArray("children")?.items() ?: emptyList()).flatMap { nodes(it) } + (raw.optJSONObject("slots")?.entries()?.values ?: emptyList()).flatMap { nodes(it) }
        val story = stories.first { fills(it.getJSONObject("scene")).isNotEmpty() }
        val scene = story.getJSONObject("scene")
        val fill = fills(scene).first()
        val field = nodes(scene.get("content")).first { it.optString("id") == fill.getString("target") }
        val initial = scene.getJSONObject("state").getString(field.getJSONObject("bind").getString("value"))
        compose.onNode(hasSetTextAction()).performTextReplacement(story.getString("title") + " " + story.getString("name"))
        compose.onNodeWithText(story.getString("name")).performClick()
        compose.onNodeWithTag(field.getString("id")).assertTextEquals(initial).performTextReplacement(fill.getString("value"))
        compose.onNodeWithTag(field.getString("id")).assertTextEquals(fill.getString("value"))
        compose.onNodeWithText("Back").performClick()
        compose.onNodeWithText(story.getString("name")).performClick()
        compose.onNodeWithTag(field.getString("id")).assertTextEquals(initial)
    }
}
