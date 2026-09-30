using Microsoft.UI.Input;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation;
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
        Loaded += (_, _) => Refresh();
        PointerEntered += (_, _) => Show(true);
        PointerExited += (_, _) => Show(from is not null);
        PointerPressed += (_, args) =>
        {
            from = Value;
            start = args.GetCurrentPoint(null).Position.X;
            CapturePointer(args.Pointer);
            args.Handled = true;
        };
        PointerMoved += (_, args) =>
        {
            if (from is not double origin) return;
            Set(origin + (args.GetCurrentPoint(null).Position.X - start) * outward);
        };
        PointerReleased += (_, args) => { from = null; ReleasePointerCapture(args.Pointer); Show(false); };
        PointerCaptureLost += (_, _) => { from = null; Show(false); };
        DoubleTapped += (_, _) => Set(column.Initial);
        KeyDown += (_, args) =>
        {
            var step = args.Key switch { VirtualKey.Right => 8, VirtualKey.Left => -8, _ => 0 };
            if (step == 0) return;
            Set(Value + step * outward);
            args.Handled = true;
        };
    }

    /// <summary>The column's width now.</summary>
    public double Value { get; private set; }

    private void Set(double width)
    {
        Value = column.Clamp(width);
        apply(Value);
    }

    private void Show(bool on) => line.Opacity = on ? 1 : 0;

    private void Refresh()
    {
        var theme = Appearance.GetTheme(this);
        Width = theme.Number("spacing.2");
        line.Width = theme.Number("spacing.0.5");
        line.Background = theme.Brush("color.primary.500");
        // The line sits on the column's own rule.
        line.HorizontalAlignment = outward > 0 ? HorizontalAlignment.Left : HorizontalAlignment.Right;
    }
}
