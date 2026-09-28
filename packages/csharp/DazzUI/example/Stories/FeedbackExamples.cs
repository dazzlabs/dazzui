using DazzUI.Playground.Storybook;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using UI = DazzUI;

namespace DazzUI.Playground.Stories;

public static partial class ComponentExamples
{
    public static UIElement Progress(UI.Theme t, string s) => Example(t, "Progress", s, "How much of a task has completed, or an indeterminate indicator when no total is known.", o =>
    {
        UI.Progress Make(double value) => Configure(new UI.Progress { Value = value, Width = t.Number("spacing.20") * 3, IsIndeterminate = s == "Indeterminate", Gradient = s == "Gradient" }, o);
        if (s == "Values") return Stage.Column(t, Make(0), Make(25), Make(50), Make(75), Make(100));
        var bar = Make(42);
        if (s == "Live") return Stage.Column(t, bar, Action("Advance", () => bar.Value = bar.Value >= 100 ? 0 : bar.Value + 10));
        if (s == "With a count") return Stage.Column(t, bar, t.Text("42 of 100 pages", "bodySmall"));
        if (s == "Beside stages") return Stage.Column(t, bar, StepList(t, "In flight"));
        return bar;
    });
    public static UIElement Meter(UI.Theme t, string s) => Example(t, "Meter", s, "A measured quantity within a known range, such as a storage quota.", o =>
    {
        var meter = Configure(new UI.Meter { Value = 64, Width = t.Number("spacing.20") * 3 }, o);
        return s is "With value" or "In a quota list" ? Stage.Column(t, t.Text("Storage · 64 of 100 GB", "labelMedium"), meter) : meter;
    });
    public static UIElement Slider(UI.Theme t, string s) => Example(t, "Slider", s, "Adjust a bounded value using the pointer, arrow keys or assistive technology.", o =>
    {
        if (s == "Range") return Configure(new UI.RangeSlider { Width = t.Number("spacing.20") * 3 }, o);
        var slider = Configure(new UI.Slider { Value = 40, Minimum = 0, Maximum = 100, StepFrequency = 5, Width = t.Number("spacing.20") * 3 }, o);
        if (s is "With value" or "Interactive") { var value = t.Text("40%", "labelSmall"); slider.ValueChanged += (_, args) => value.Text = $"{args.NewValue:0}%"; return Stage.Column(t, slider, value); }
        return slider;
    });
    public static UIElement Spinner(UI.Theme t, string s) => Example(t, "Spinner", s, "An indeterminate wait indicator that can sit alongside an action or message.", o =>
    {
        var spinner = Configure(new UI.Spinner(), o);
        if (s == "In a button") return new UI.Button { Content = Stage.Row(t, spinner, t.Text("Saving…")), IsEnabled = false };
        if (s == "In a callout") return new UI.Callout { Title = "Uploading glossary", Description = "This will take a moment.", Icon = spinner };
        if (s == "In a toast") return new UI.Toast { Options = new("Uploading glossary", "Please wait.") };
        if (s == "On an accent") return new UI.Card(spinner, t) { Variant = UI.CardVariant.Tinted };
        return spinner;
    });
    public static UIElement Skeleton(UI.Theme t, string s) => Example(t, "Skeleton", s, "A placeholder that preserves the shape of content while it loads.", o =>
    {
        if (s is "Text" or "Lines") return new UI.SkeletonText { Lines = s == "Lines" ? 5 : 3, Width = t.Number("spacing.20") * 3 };
        var image = new UI.Skeleton { Shape = UI.SkeletonShape.Circle };
        if (s == "A list row") return Stage.Row(t, image, new UI.SkeletonText { Width = t.Number("spacing.20") * 2 });
        return Stage.Column(t, image, new UI.Skeleton { Width = t.Number("spacing.20") * 3, Height = t.Number("spacing.20") });
    });
    public static UIElement StepList(UI.Theme t, string s) => Example(t, "StepList", s, "A process broken into named stages, with progress and detail for each stage.", o =>
    {
        var list = new UI.StepList();
        var names = new[] { "Read document", "Translate content", "Review terminology", "Export result" };
        for (var i = 0; i < names.Length; i++) list.Steps.Add(Configure(new UI.Step { Title = names[i], Detail = s == "Without detail" ? null : $"Stage {i + 1} of 4", Status = s == "Finished" ? UI.StepStatus.Complete : s == "Statuses" ? (UI.StepStatus)i : i == 0 ? UI.StepStatus.Complete : i == 1 ? UI.StepStatus.Active : UI.StepStatus.Pending }, o));
        if (s == "Advancing") return Stage.Column(t, list, Action("Complete next stage", () => { var next = list.Steps.FirstOrDefault(step => step.Status != UI.StepStatus.Complete); if (next is not null) next.Status = UI.StepStatus.Complete; UI.Appearance.Refresh(list); }));
        return list;
    });
    public static UIElement Toast(UI.Theme t, string s) => Example(t, "Toast", s, "A brief status message with optional detail and action. Messages can be dismissed or time out.", o =>
    {
        var viewport = new UI.ToastViewport();
        var options = new UI.ToastOptions(s == "Truncation" ? "The selected document has been translated and exported to the workspace output directory" : "Saved to Downloads", s == "Shapes" ? "Your document is ready to share." : null, o.Tint, s == "Shapes" ? "Open" : null, Duration: TimeSpan.Zero);
        if (s == "Posting one") return Stage.Column(t, Action("Show toast", () => viewport.Add(options with { Duration = TimeSpan.FromSeconds(5) })), viewport);
        viewport.Add(options);
        if (s == "A stack") { viewport.Add(new("Translation finished", Tint: UI.WidgetTint.Success, Duration: TimeSpan.Zero)); viewport.Add(new("Connection interrupted", Tint: UI.WidgetTint.Danger, Duration: TimeSpan.Zero)); }
        return viewport;
    });
}
