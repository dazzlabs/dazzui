using Microsoft.UI.Input;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Input;
using Windows.System;

namespace DazzUI.Example;

/// <summary>
/// The edge of a column, to be dragged. It draws nothing until the pointer
/// is on it — the rule is already there, drawn by the column — and then the
/// accent line; a double-click resets it.
/// </summary>
public sealed class Resizer : Grid
{
    private readonly double outward;
    private readonly ColumnWidth column;
    private readonly Action<double> apply;
    private readonly Border line = new() { Opacity = 0 };
    private double? from;
    private double start;
    private bool hovered;
    private bool keyboardFocused;

    /// <param name="outward"><c>1</c> when dragging right widens the column, <c>-1</c> when dragging left does.</param>
    public Resizer(double outward, ColumnWidth column, Action<double> apply, string label)
    {
        this.outward = outward;
        this.column = column;
        this.apply = apply;
        Value = column.Initial;
        Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent);
        IsTabStop = true;
        Children.Add(line);
        AutomationProperties.SetName(this, label);
        ProtectedCursor = InputSystemCursor.Create(InputSystemCursorShape.SizeWestEast);
        Themed.Follow(this, Refresh);
        PointerEntered += (_, _) => { hovered = true; Show(); };
        PointerExited += (_, _) => { hovered = false; Show(); };
        GettingFocus += (_, args) => { keyboardFocused = args.FocusState == FocusState.Keyboard; Show(); };
        LostFocus += (_, _) => { keyboardFocused = false; Show(); };
        PointerPressed += (_, args) =>
        {
            from = Value;
            start = args.GetCurrentPoint(null).Position.X;
            keyboardFocused = false;
            CapturePointer(args.Pointer);
            Show();
            args.Handled = true;
        };
        PointerMoved += (_, args) =>
        {
            if (from is not double origin) return;
            Set(origin + (args.GetCurrentPoint(null).Position.X - start) * outward);
        };
        PointerReleased += (_, args) => { from = null; ReleasePointerCapture(args.Pointer); Show(); };
        PointerCaptureLost += (_, _) => { from = null; Show(); };
        DoubleTapped += (_, _) => Set(column.Initial);
        KeyDown += (_, args) =>
        {
            var coarse = InputKeyboardSource.GetKeyStateForCurrentThread(VirtualKey.Shift).HasFlag(Windows.UI.Core.CoreVirtualKeyStates.Down);
            if (!ResizeKey(args.Key, coarse)) return;
            keyboardFocused = true;
            Show();
            args.Handled = true;
        };
    }

    /// <summary>The column's width now.</summary>
    public double Value { get; private set; }

    internal bool ResizeKey(VirtualKey key, bool coarse)
    {
        var step = coarse ? 32 : 8;
        var next = key switch
        {
            VirtualKey.Right => Value + step * outward,
            VirtualKey.Left => Value - step * outward,
            VirtualKey.Home => column.Min,
            VirtualKey.End => column.Max,
            _ => double.NaN,
        };
        if (double.IsNaN(next)) return false;
        Set(next);
        return true;
    }

    private void Set(double width)
    {
        Value = column.Clamp(width);
        apply(Value);
    }

    private void Show() => line.Opacity = hovered || keyboardFocused || from is not null ? 1 : 0;

    private void Refresh(Theme theme)
    {
        Width = theme.Number("spacing.2");
        line.Width = theme.Number("spacing.0.5");
        line.Background = theme.Brush("color.primary.500");
        // The line sits on the column's own rule.
        line.HorizontalAlignment = outward > 0 ? HorizontalAlignment.Left : HorizontalAlignment.Right;
    }

    protected override AutomationPeer OnCreateAutomationPeer() => new ResizerPeer(this);

    private sealed class ResizerPeer(Resizer owner) : FrameworkElementAutomationPeer(owner), IRangeValueProvider
    {
        public bool IsReadOnly => false;
        public double Minimum => owner.column.Min;
        public double Maximum => owner.column.Max;
        public double Value => owner.Value;
        public double SmallChange => 8;
        public double LargeChange => 32;
        public void SetValue(double value) => owner.Set(value);
        protected override object GetPatternCore(PatternInterface pattern) => pattern == PatternInterface.RangeValue ? this : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.Separator;
        protected override AutomationOrientation GetOrientationCore() => AutomationOrientation.Vertical;
    }
}
