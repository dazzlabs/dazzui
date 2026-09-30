using System.Text.Json.Nodes;
using System.Text.RegularExpressions;

namespace DazzUI.Example;

/// <summary>
/// The walk that addresses a scene's specimens, as tools/codegen/scenes.mjs
/// defines it: pre-order — an element, then the elements among its props in
/// the order the story wrote them, then its children — at the scene's
/// initial state. The nodes it yields are the story's own objects, the ones
/// <see cref="Nodes.Built"/> reports, so a node leads to the element that
/// drew it.
/// </summary>
internal static partial class Addresses
{
    private static readonly HashSet<string> Stage = ["Row", "Col", "Frame", "Text"];

    public static List<JsonNode> SpecimensOf(Scene scene)
    {
        var found = new List<JsonNode>();

        void InProps(JsonNode? value)
        {
            var resolved = Resolve(value, scene.State);
            if (resolved is JsonArray list) { foreach (var item in list) InProps(item); return; }
            if (resolved is not JsonObject item2) return;
            if (item2.ContainsKey("$el")) { Visit(item2); return; }
            if (item2.ContainsKey("$do") || item2.ContainsKey("$icon")) return;
            foreach (var pair in item2) InProps(pair.Value);
        }

        void Visit(JsonNode? node)
        {
            if (node.Element() is not string component) return;
            if (!Stage.Contains(component)) found.Add(node!);
            if (node!["props"] is JsonObject props)
                foreach (var pair in props) InProps(pair.Value);
            foreach (var child in Flatten(node["children"], scene.State)) Visit(child);
        }

        foreach (var node in Flatten(scene.Content, scene.State)) Visit(node);
        return found;
    }

    private static List<JsonNode?> Flatten(JsonNode? value, JsonObject state)
    {
        var resolved = Resolve(value, state);
        if (resolved is JsonArray list) return list.SelectMany(item => Flatten(item, state)).ToList();
        if (resolved is null || resolved.Kind() is System.Text.Json.JsonValueKind.True or System.Text.Json.JsonValueKind.False)
            return [];
        return [resolved];
    }

    private static JsonNode? Resolve(JsonNode? value, JsonObject state)
    {
        if (value is not JsonObject item) return value;
        if (item["$state"].String() is string slot) return Resolve(state[slot], state);
        if (item.ContainsKey("$arg")) return null;
        if (item.ContainsKey("$not")) return JsonValue.Create(!Resolve(item["$not"], state).Truthy());
        if (item["$eq"] is JsonArray pair && pair.Count == 2)
            return JsonValue.Create(Json.Same(Resolve(pair[0], state), Resolve(pair[1], state)));
        if (item["$if"] is JsonArray branches && branches.Count == 3)
            return Resolve(Resolve(branches[0], state).Truthy() ? branches[1] : branches[2], state);
        if (item.ContainsKey("$concat"))
            return JsonValue.Create(string.Concat(item["$concat"].Items().Select(part => Resolve(part, state).Text())));
        return value;
    }

    /// <summary><c>SegmentedControl</c> → <c>segmented-control</c>, as scenes.json names it.</summary>
    public static string Kebab(string name) => Camel().Replace(name, "$1-$2").ToLowerInvariant();

    [GeneratedRegex("([a-z0-9])([A-Z])")]
    private static partial Regex Camel();
}
