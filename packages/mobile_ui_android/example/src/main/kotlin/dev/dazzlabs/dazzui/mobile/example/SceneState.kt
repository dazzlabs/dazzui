package dev.dazzlabs.dazzui.mobile.example

import androidx.compose.runtime.mutableStateMapOf
import org.json.JSONArray
import org.json.JSONObject

internal fun JSONArray.items(): List<Any> = (0 until length()).map { get(it) }
internal fun JSONObject.entries(): Map<String, Any> = keys().asSequence().associateWith { get(it) }

/** The host owns lifecycle; every value and transition comes from scene data. */
class SceneState(val scene: JSONObject) {
    val values = mutableStateMapOf<String, Any>().apply {
        putAll(scene.optJSONObject("state")?.entries() ?: emptyMap())
    }
    private val timers = scene.optJSONArray("timers")?.items()?.map { it as JSONObject } ?: emptyList()
    private val due = timers.map { it.getLong("every") }.toMutableList()
    private var elapsed = 0L

    fun evaluate(raw: Any): Any {
        if (raw !is JSONObject) return raw
        fun operand(key: String, index: Int): Any = evaluate(raw.getJSONArray(key).get(index))
        return when {
            raw.has("state") -> values.getValue(raw.getString("state"))
            raw.has("if") -> evaluate(raw.get(if (evaluate(raw.get("if")) as Boolean) "then" else "else"))
            raw.has("trim") -> (evaluate(raw.get("trim")) as String).trim { it.code in sceneTrimCodeUnits }
            raw.has("concat") -> raw.getJSONArray("concat").items().joinToString("") { evaluate(it) as String }
            raw.has("not") -> !(evaluate(raw.get("not")) as Boolean)
            raw.has("and") -> raw.getJSONArray("and").items().all { evaluate(it) as Boolean }
            raw.has("or") -> raw.getJSONArray("or").items().any { evaluate(it) as Boolean }
            raw.has("lower") -> (evaluate(raw.get("lower")) as String).lowercase()
            raw.has("length") -> (evaluate(raw.get("length")) as String).codePointCount(0, (evaluate(raw.get("length")) as String).length).toDouble()
            raw.has("contains") -> (operand("contains", 0) as String).contains(operand("contains", 1) as String)
            raw.has("eq") -> {
                val a = operand("eq", 0); val b = operand("eq", 1)
                if (a is Number && b is Number) a.toDouble() == b.toDouble() else a == b
            }
            raw.has("add") -> (operand("add", 0) as Number).toDouble() + (operand("add", 1) as Number).toDouble()
            raw.has("multiply") -> (operand("multiply", 0) as Number).toDouble() * (operand("multiply", 1) as Number).toDouble()
            raw.has("text") -> sceneText(evaluate(raw.get("text")))
            raw.has("gte") -> (operand("gte", 0) as Number).toDouble() >= (operand("gte", 1) as Number).toDouble()
            else -> error("Unsupported scene expression: $raw")
        }
    }

    fun props(node: JSONObject): Map<String, Any> = buildMap {
        putAll(node.optJSONObject("props")?.entries() ?: emptyMap())
        node.optJSONObject("derive")?.entries()?.forEach { (key, value) -> put(key, evaluate(value)) }
    }

    fun press(node: JSONObject) = fire(node, "press")

    /** Runs a declared event's assignment; an empty action assigns nothing. */
    fun fire(node: JSONObject, event: String) {
        if (props(node)["disabled"] == true) return
        node.optJSONObject("on")?.optJSONObject(event)?.takeIf { it.has("set") }?.let { values[it.getString("set")] = it.get("value") }
    }

    fun bound(node: JSONObject, axis: String, fallback: Any): Any =
        node.optJSONObject("bind")?.optString(axis)?.takeIf { it.isNotEmpty() }?.let { values.getValue(it) } ?: fallback

    fun change(node: JSONObject, axis: String, value: Any) {
        node.optJSONObject("bind")?.optString(axis)?.takeIf { it.isNotEmpty() }?.let { values[it] = value }
    }

    /** Fire timers chronologically, resolving each expression against current state. */
    fun advance(milliseconds: Long) {
        require(milliseconds >= 0)
        val end = Math.addExact(elapsed, milliseconds)
        while (true) {
            val next = due.minOrNull() ?: break
            if (next > end) break
            timers.indices.filter { due[it] == next }.forEach { index ->
                val timer = timers[index]
                values[timer.getString("set")] = evaluate(timer.get("value"))
                due[index] = Math.addExact(due[index], timer.getLong("every"))
            }
        }
        elapsed = end
    }
}

internal fun sceneText(value: Any): String = if (value is Number) value.toString().removeSuffix(".0") else value.toString()
