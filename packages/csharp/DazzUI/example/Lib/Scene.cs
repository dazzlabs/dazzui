using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace DazzUI.Example;

/// <summary>A scene's slots: the values its <c>useState</c>s hold.</summary>
public sealed class SceneState(JsonObject initial)
{
    private readonly Dictionary<string, JsonNode?> slots =
        initial.ToDictionary(pair => pair.Key, pair => pair.Value?.DeepClone());

    /// <summary>Raised after a slot takes a new value.</summary>
    public event Action? Changed;

    public JsonNode? Read(string slot) => slots.GetValueOrDefault(slot);

    public void Write(string slot, JsonNode? value)
    {
        if (Json.Same(Read(slot), value)) return;
        slots[slot] = value?.DeepClone();
        Changed?.Invoke();
    }
}

/// <summary>
/// What a node is built with besides its own props — React's context: the
/// scene's slots, the toaster a provider holds, the press a trigger is
/// handed, the close a presented layer offers, the sidebar it stands in.
/// </summary>
public sealed record Context(SceneState Scene)
{
    public Toaster? Toaster { get; init; }
    /// <summary>The press a trigger is given — the button a dialog, a drawer or a menu opens from never declares its own.</summary>
    public Action? Press { get; init; }
    /// <summary>Dismisses the dialog, drawer or popover the node is drawn in.</summary>
    public Action? Close { get; init; }
    public SidebarContext? Sidebar { get; init; }
    /// <summary>What a submit button in a form does: the form's own <c>onSubmit</c>.</summary>
    public Action? Submit { get; init; }
}

/// <summary>Draws one <see cref="Scene"/>: its slots, held here, and its tree.</summary>
public sealed class SceneView : Grid
{
    /// <param name="align">Whether the tree stands centred in the room, as it does on a stage.</param>
    public SceneView(Scene scene, bool align = false)
    {
        if (Nodes.Build(scene.Content, new Context(new SceneState(scene.State))) is not UIElement content) return;
        if (align && content is FrameworkElement framed)
        {
            framed.HorizontalAlignment = HorizontalAlignment.Center;
            framed.VerticalAlignment = VerticalAlignment.Center;
        }
        Children.Add(content);
    }
}

public static class Nodes
{
    private static readonly string[] LiveKeys = ["$state", "$eq", "$not", "$if", "$concat", "$arg"];

    /// <summary>Whether a value is only known while the scene runs.</summary>
    public static bool IsLive(JsonNode? value) =>
        value is JsonObject item && LiveKeys.Any(item.ContainsKey);

    /// <summary>Whether a value holds anything live, however deep.</summary>
    public static bool HasLive(JsonNode? value) => value switch
    {
        // An event's writes are read when it fires, not drawn.
        JsonObject item when item.ContainsKey("$do") => false,
        JsonObject item => IsLive(item) || item.Any(pair => HasLive(pair.Value)),
        JsonArray list => list.Any(HasLive),
        _ => false,
    };

    /// <summary>One node of a scene, drawn.</summary>
    public static UIElement? Build(JsonNode? node, Context context)
    {
        if (node.String() is string text) return Stage.Words(text);
        if (node?["$icon"].String() is string glyph) return Glyphs.Draw(glyph, 20);
        if (node.Element() is string component)
        {
            if (!Mapping.Components.TryGetValue(component, out var build))
                throw new NotSupportedException($"No WinUI mapping for <{component}>");
            return build(new Node(node!, context));
        }
        if (IsLive(node))
        {
            var words = Stage.Words("");
            void Update() => words.Text = context.Resolve(node).Text();
            Update();
            Watch(context.Scene, words, Update);
            return words;
        }
        return null;
    }

    /// <summary>Runs <paramref name="update"/> whenever a slot changes, for as long as the element is in the window.</summary>
    public static void Watch(SceneState scene, FrameworkElement element, Action update)
    {
        void Changed() => update();
        scene.Changed += Changed;
        element.Loaded += (_, _) => { scene.Changed -= Changed; scene.Changed += Changed; update(); };
        element.Unloaded += (_, _) => scene.Changed -= Changed;
    }

    /// <summary>The glyph an icon element or a bare <c>$icon</c> names.</summary>
    public static string? GlyphOf(JsonNode? value) =>
        value?["$icon"].String() ?? value?["props"]?["icon"]?["$icon"].String();

    /// <summary>A prop's value now: a slot read, a test worked out, an argument taken.</summary>
    public static JsonNode? Resolve(this Context context, JsonNode? value, IReadOnlyList<JsonNode?>? args = null)
    {
        if (value is not JsonObject item || !IsLive(item)) return value;
        if (item["$state"].String() is string slot) return context.Scene.Read(slot);
        if (item["$arg"].Number() is double index)
            return args is not null && (int)index < args.Count ? args[(int)index] : null;
        if (item["$eq"] is JsonArray pair && pair.Count == 2)
            return JsonValue.Create(Json.Same(context.Resolve(pair[0], args), context.Resolve(pair[1], args)));
        if (item.ContainsKey("$not")) return JsonValue.Create(!context.Resolve(item["$not"], args).Truthy());
        if (item["$if"] is JsonArray branches && branches.Count == 3)
            return context.Resolve(branches[0], args).Truthy()
                ? context.Resolve(branches[1], args)
                : context.Resolve(branches[2], args);
        return JsonValue.Create(string.Concat(item["$concat"].Items().Select(part => context.Resolve(part, args).Text())));
    }

    /// <summary>Makes the writes an event declares: slots set, toasts raised.</summary>
    public static void Run(this Context context, JsonNode? action, IReadOnlyList<JsonNode?>? args = null)
    {
        foreach (var step in action?["$do"].Items() ?? [])
        {
            if (step?["set"].String() is string slot)
                context.Scene.Write(slot, context.Resolve(step["to"], args));
            else if (step?["toast"] is JsonNode toast)
            {
                var act = toast["action"];
                context.Toaster?.Add(new ToastOptions(
                    context.Resolve(toast["title"], args).Text(),
                    toast["description"] is null ? null : context.Resolve(toast["description"], args).Text(),
                    Mapping.Option(context.Resolve(toast["tint"], args).String(), WidgetTint.Neutral),
                    act is null ? null : act["label"].Text(),
                    act is null ? null : () => context.Run(act["onClick"])));
            }
        }
    }
}

/// <summary>A component in the tree, read the way its builder needs it.</summary>
public sealed class Node(JsonNode json, Context context)
{
    public string Component { get; } = json.Element() ?? "";
    public JsonObject Props { get; } = json["props"]?.AsObject() ?? [];
    public IReadOnlyList<JsonNode?> Children { get; } = json["children"].Items();
    public Context Context { get; } = context;

    public bool Has(string name) => Props.ContainsKey(name);

    /// <summary>A prop's current value.</summary>
    public JsonNode? this[string name] => Context.Resolve(Props[name]);

    public string? String(string name) => this[name] is JsonNode value ? value.Text() : null;

    public bool Flag(string name) => this[name].Truthy();

    public double? Number(string name) => this[name].Number();

    /// <summary>A string prop read as one of a kit enum's cases, or the fallback.</summary>
    public T Option<T>(string name, T fallback) where T : struct, Enum => Mapping.Option(String(name), fallback);

    /// <summary>A prop that holds a component, drawn.</summary>
    public UIElement? Element(string name, Context? context = null) =>
        Props[name] is JsonNode value ? Nodes.Build(value, context ?? Context) : null;

    /// <summary>The children, drawn.</summary>
    public IEnumerable<UIElement> Kids(Context? context = null) =>
        Children.Select(child => Nodes.Build(child, context ?? Context)).OfType<UIElement>();

    /// <summary>The children as one string — the label of a button, the body of a callout.</summary>
    public string Text => string.Concat(Children.Select(child => Context.Resolve(child).Text()));

    /// <summary>An event with no argument, or null when there is none.</summary>
    public Action? Action(string name) =>
        Props[name] is JsonNode value ? () => Context.Run(value) : null;

    /// <summary>An event that passes what changed.</summary>
    public Action<JsonNode?>? Handler(string name) =>
        Props[name] is JsonNode value ? argument => Context.Run(value, [argument]) : null;

    /// <summary>What a press does: a trigger's own, the story's event, or nothing — a React button with no handler is still a button.</summary>
    public Action Press(string name = "onClick") => Context.Press ?? Action(name) ?? (() => { });

    /// <summary>
    /// Draws what depends on the scene's slots, now and whenever one
    /// changes, for as long as the element is in the window.
    /// </summary>
    public void Live(FrameworkElement element, Action update)
    {
        update();
        if (Nodes.HasLive(Props) || Children.Any(Nodes.IsLive)) Nodes.Watch(Context.Scene, element, update);
    }
}

/// <summary>
/// The toasts a <c>ToastProvider</c> raises: shown at the window's edge, as
/// the web's fixed viewport shows them.
/// </summary>
public sealed class Toaster
{
    public string Placement { get; set; } = "bottom";

    public void Add(ToastOptions options) => Overlay.Toast(options, Placement == "top");
}
