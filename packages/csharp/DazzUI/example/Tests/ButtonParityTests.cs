using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;

namespace DazzUI.Example;

internal static class ButtonParityTests
{
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    private static void Check(bool pass, string message)
    {
        if (!pass) throw new InvalidOperationException(message);
    }

    internal static async Task RunAsync(Shell shell, string output)
    {
        var source = JsonNode.Parse("""
            {"$el":"Button","props":{"size":"large","shortcut":{"$el":"KeyCap","props":{"variant":"key","size":"large"},"children":["Ctrl+S"]}},"children":["Save",{"$el":"Icon","props":{"icon":{"$icon":"ArrowDownload20Regular"}}}]}
            """)!;
        var shortcut = source["props"]!["shortcut"]!;
        var built = new List<UIElement>();
        void Record(JsonNode node, UIElement element) { if (ReferenceEquals(node, shortcut)) built.Add(element); }
        Nodes.Built += Record;
        Button button;
        try { button = (Button)Nodes.Build(source, new Context(new SceneState(new JsonObject())))!; }
        finally { Nodes.Built -= Record; }
        Check(built is [KeyCap { Variant: KeyCapVariant.Key, Size: ControlSize.Large }], "Button mapping flattened the shortcut or discarded its props/address.");
        var cap = (KeyCap)built[0];
        var plainSource = JsonNode.Parse("""{"$el":"Button","props":{"shortcut":"F2"},"children":["Rename"]}""")!;
        var plain = (Button)Nodes.Build(plainSource, new Context(new SceneState(new JsonObject())))!;
        var scope = new ThemeScope { Content = new StackPanel { Spacing = 12, Children = { button, plain } }, Padding = new Thickness(24), HorizontalAlignment = HorizontalAlignment.Left };
        shell.ShowFixture(scope);
        foreach (var name in new[] { "studio-light", "bright-dark", "windows11-light" })
        {
            var theme = new Theme(name);
            scope.Theme = theme; scope.Background = theme.Brush("color.surface");
            await Task.Delay(80); shell.UpdateLayout();
            Check(cap.XamlRoot is not null && Walk(button).Contains(cap), $"{name}: the authored shortcut was not mounted in the button.");
            Check(cap.Parent is Border capHost && Math.Abs(capHost.Opacity - .7) < .000001, $"{name}: shortcut did not retain React's quieter slot opacity: parent={cap.Parent?.GetType().Name}, opacity={(cap.Parent as UIElement)?.Opacity:R}.");
            Check(cap.Variant == KeyCapVariant.Key && cap.FontSize == theme.Text("", "labelMedium").FontSize && cap.ActualHeight == theme.Number("spacing.5") + theme.Number("spacing.0.5"), $"{name}: shortcut lost its own key variant or large type.");
            Check(button.Padding.Right == theme.Number("control.large.padding.inline") - theme.Number("control.label-glyph.inset"), $"{name}: appending a shortcut removed the label's trailing-glyph trim.");
            Check(!Walk(plain).OfType<KeyCap>().Any(), $"{name}: a plain-text shortcut was converted to a KeyCap.");
            var hint = Walk(plain).OfType<TextBlock>().Single(text => text.Text == "F2");
            Check(hint.Parent is Border hintHost && Math.Abs(hintHost.Opacity - .7) < .000001 && hint.FontSize == plain.FontSize, $"{name}: plain shortcut lost inherited button type or quiet opacity: parent={hint.Parent?.GetType().Name}, opacity={(hint.Parent as UIElement)?.Opacity:R}, font={hint.FontSize}, buttonFont={plain.FontSize}.");
            await SmokeTest.SaveAsync(scope, output, $"button-shortcuts-{name}.png");
        }
    }
}
