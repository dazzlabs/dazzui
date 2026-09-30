using System.Globalization;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace DazzUI.Example;

/// <summary>
/// A story, as codegen compiled it out of the React example: what the list
/// shows, the one scene that stands for the component on the overview, and
/// the examples its page shows.
/// </summary>
public sealed record Story(string Id, string Name, string Summary, Scene Preview, IReadOnlyList<StoryExample> Examples)
{
    /// <summary>Reads the JSON codegen wrote for the story.</summary>
    public static Story Parse(string json)
    {
        var story = JsonNode.Parse(json)?.AsObject() ?? throw new FormatException("A generated story is not an object");
        return new(
            story["id"].Text(),
            story["name"].Text(),
            story["summary"].Text(),
            Scene.Of(story["preview"]),
            (story["examples"]?.AsArray() ?? []).Select(example => new StoryExample(
                example!["title"].Text(),
                example["note"]?.Text(),
                Scene.Of(example["demo"]))).ToArray());
    }
}

public sealed record StoryExample(string Title, string? Note, Scene Demo);

/// <summary>
/// A finished tree of components, and the slots it keeps live.
///
/// <c>Content</c> is a node: a string is text; an object with <c>$el</c> is a
/// component with its <c>props</c> and <c>children</c>. A prop may hold a
/// node, a glyph (<c>$icon</c>), a slot's value (<c>$state</c>), a test on one
/// (<c>$eq</c>, <c>$not</c>, <c>$if</c>), or — for an event — the writes it
/// makes (<c>$do</c>).
/// </summary>
public sealed record Scene(JsonObject State, JsonNode? Content)
{
    public static Scene Of(JsonNode? value) =>
        new(value?["state"]?.AsObject() ?? [], value?["content"]);
}

/// <summary>A story's JSON, read the way JavaScript would read it.</summary>
public static class Json
{
    public static JsonValueKind Kind(this JsonNode? value) => value?.GetValueKind() ?? JsonValueKind.Null;

    /// <summary>JavaScript's truthiness, which is what a story's tests were written in.</summary>
    public static bool Truthy(this JsonNode? value) => value.Kind() switch
    {
        JsonValueKind.Null or JsonValueKind.Undefined or JsonValueKind.False => false,
        JsonValueKind.Number => value!.GetValue<double>() != 0,
        JsonValueKind.String => value!.GetValue<string>().Length > 0,
        _ => true,
    };

    /// <summary>The value as text, the way JavaScript would print it.</summary>
    public static string Text(this JsonNode? value) => value.Kind() switch
    {
        JsonValueKind.Null or JsonValueKind.Undefined => "",
        JsonValueKind.True => "true",
        JsonValueKind.False => "false",
        JsonValueKind.Number => value!.GetValue<double>().ToString(CultureInfo.InvariantCulture),
        JsonValueKind.String => value!.GetValue<string>(),
        JsonValueKind.Array => string.Join(",", value!.AsArray().Select(Text)),
        _ => "[object Object]",
    };

    public static string? String(this JsonNode? value) =>
        value.Kind() == JsonValueKind.String ? value!.GetValue<string>() : null;

    public static double? Number(this JsonNode? value) =>
        value.Kind() == JsonValueKind.Number ? value!.GetValue<double>() : null;

    public static IReadOnlyList<JsonNode?> Items(this JsonNode? value) =>
        value.Kind() == JsonValueKind.Array ? value!.AsArray().ToArray() : [];

    public static string[] Strings(this JsonNode? value) => value.Items().Select(Text).ToArray();

    public static bool Same(JsonNode? a, JsonNode? b) => JsonNode.DeepEquals(a, b);

    /// <summary>The component an element names, or null for text and glyphs.</summary>
    public static string? Element(this JsonNode? value) =>
        value.Kind() == JsonValueKind.Object ? value!["$el"].String() : null;
}
