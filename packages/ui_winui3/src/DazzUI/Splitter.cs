using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Windows.System;

namespace DazzUI;

public sealed record SplitterPanel(UIElement Content, double Weight = 1, double Minimum = 0, double Maximum = double.PositiveInfinity);
public sealed class Splitter : Surface
{
    public IList<SplitterPanel> Panels { get; } = new List<SplitterPanel>();
    public Orientation Orientation { get; set; }
    protected override void RefreshAppearance()
    {
        var previous = Content as Grid;
        var weights = Panels.Select((panel, i) => previous is null ? panel.Weight : Orientation == Orientation.Horizontal && previous.ColumnDefinitions.Count > i * 2 ? previous.ColumnDefinitions[i * 2].ActualWidth : Orientation == Orientation.Vertical && previous.RowDefinitions.Count > i * 2 ? previous.RowDefinitions[i * 2].ActualHeight : panel.Weight).ToArray();
        var grid = new Grid();
        for (var i = 0; i < Panels.Count; i++)
        {
            var panel = Panels[i]; var index = i * 2;
            if (panel.Minimum < 0 || panel.Maximum < panel.Minimum || panel.Weight <= 0) throw new ArgumentException("Panel weights must be positive and limits must form a nonnegative range.");
            var weight = weights[i] > 0 ? weights[i] : panel.Weight;
            if (Orientation == Orientation.Horizontal) grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(weight, GridUnitType.Star), MinWidth = panel.Minimum, MaxWidth = panel.Maximum });
            else grid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(weight, GridUnitType.Star), MinHeight = panel.Minimum, MaxHeight = panel.Maximum });
            Skin.Detach(panel.Content); panel.Content.SetValue(Orientation == Orientation.Horizontal ? Grid.ColumnProperty : Grid.RowProperty, index); grid.Children.Add(panel.Content);
            if (i + 1 == Panels.Count) continue;
            if (Orientation == Orientation.Horizontal) grid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            else grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
            var grip = new SplitterGrip { Background = Theme.Brush("color.border"), IsTabStop = true, MinimumValue = panel.Minimum, MaximumValue = panel.Maximum };
            if (Orientation == Orientation.Horizontal) grip.Width = Theme.Number("spacing.1"); else grip.Height = Theme.Number("spacing.1");
            Skin.Focus(grip, Theme);
            grip.ReadValue = () => Orientation == Orientation.Horizontal ? grid.ColumnDefinitions[index].ActualWidth : grid.RowDefinitions[index].ActualHeight;
            grip.ReadMaximum = () => Orientation == Orientation.Horizontal ? Math.Min(panel.Maximum, grid.ColumnDefinitions[index].ActualWidth + grid.ColumnDefinitions[index + 2].ActualWidth - grid.ColumnDefinitions[index + 2].MinWidth) : Math.Min(panel.Maximum, grid.RowDefinitions[index].ActualHeight + grid.RowDefinitions[index + 2].ActualHeight - grid.RowDefinitions[index + 2].MinHeight);
            grip.Move = delta =>
            {
                if (Orientation == Orientation.Horizontal)
                {
                    var first = grid.ColumnDefinitions[index]; var second = grid.ColumnDefinitions[index + 2];
                    var total = first.ActualWidth + second.ActualWidth;
                    var minimum = Math.Max(first.MinWidth, total - second.MaxWidth);
                    var size = Math.Clamp(first.ActualWidth + delta, minimum, Math.Max(minimum, Math.Min(first.MaxWidth, total - second.MinWidth)));
                    first.Width = new GridLength(size); second.Width = new GridLength(total - size);
                }
                else
                {
                    var first = grid.RowDefinitions[index]; var second = grid.RowDefinitions[index + 2];
                    var total = first.ActualHeight + second.ActualHeight;
                    var minimum = Math.Max(first.MinHeight, total - second.MaxHeight);
                    var size = Math.Clamp(first.ActualHeight + delta, minimum, Math.Max(minimum, Math.Min(first.MaxHeight, total - second.MinHeight)));
                    first.Height = new GridLength(size); second.Height = new GridLength(total - size);
                }
            };
            grip.Handle.DragDelta += (_, args) => grip.Move(Orientation == Orientation.Horizontal ? args.HorizontalChange : args.VerticalChange);
            grip.KeyDown += (_, args) =>
            {
                if (args.Key == (Orientation == Orientation.Horizontal ? VirtualKey.Left : VirtualKey.Up)) { grip.Move(-Theme.Number("spacing.4")); args.Handled = true; }
                else if (args.Key == (Orientation == Orientation.Horizontal ? VirtualKey.Right : VirtualKey.Down)) { grip.Move(Theme.Number("spacing.4")); args.Handled = true; }
            };
            grip.SetValue(Orientation == Orientation.Horizontal ? Grid.ColumnProperty : Grid.RowProperty, index + 1); grid.Children.Add(grip);
        }
        Content = grid;
    }
}
internal sealed class SplitterGrip : Surface
{
    internal Thumb Handle { get; } = new();
    public SplitterGrip() { Content = Handle; Handle.IsTabStop = false; }
    protected override void RefreshAppearance() { Handle.Background = Background; }
    internal Action<double> Move { get; set; } = _ => { };
    internal Func<double> ReadValue { get; set; } = () => 0;
    internal Func<double> ReadMaximum { get; set; } = () => 0;
    internal double MinimumValue { get; set; }
    internal double MaximumValue { get; set; }
    protected override AutomationPeer OnCreateAutomationPeer() => new GripPeer(this);
    private sealed class GripPeer(SplitterGrip owner) : FrameworkElementAutomationPeer(owner), IRangeValueProvider
    {
        public bool IsReadOnly => !owner.IsEnabled;
        public double LargeChange => Appearance.GetTheme(owner).Number("spacing.8");
        public double SmallChange => Appearance.GetTheme(owner).Number("spacing.4");
        public double Maximum => owner.ReadMaximum();
        public double Minimum => owner.MinimumValue;
        public double Value => owner.ReadValue();
        public void SetValue(double value) { if (IsReadOnly) throw new InvalidOperationException("The splitter is disabled."); owner.Move(Math.Clamp(value, Minimum, Maximum) - Value); }
        protected override object GetPatternCore(PatternInterface pattern) => pattern == PatternInterface.RangeValue ? this : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.Separator;
        protected override string GetNameCore() => "Resize panels";
    }
}
