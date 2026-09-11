using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Windows.System;

namespace DazzUI;

/// <summary>A shared track with two independently focusable, keyboard and UIA range handles.</summary>
public sealed class RangeSlider : Surface
{
    private readonly Canvas track = new();
    private readonly Border groove = new();
    private readonly Border selection = new();
    public RangeEndpoint Lower { get; } = new();
    public RangeEndpoint Upper { get; } = new() { Value = 100 };
    private bool changing;
    public event EventHandler? ValueChanged;
    public RangeSlider()
    {
        Content = track;
        track.Children.Add(groove); track.Children.Add(selection);
        track.Children.Add(Lower); track.Children.Add(Upper);
        Lower.ValueChanged += (_, _) => Changed(Lower);
        Upper.ValueChanged += (_, _) => Changed(Upper);
        Lower.Drag = delta => Move(Lower, delta); Upper.Drag = delta => Move(Upper, delta);
        SizeChanged += (_, _) => ArrangeTrack();
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(Lower, "Range start");
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(Upper, "Range end");
    }
    private void Changed(RangeEndpoint source)
    {
        if (changing) return;
        changing = true;
        if (Lower.Value > Upper.Value) source.Value = source == Lower ? Upper.Value : Lower.Value;
        changing = false; ArrangeTrack(); ValueChanged?.Invoke(this, EventArgs.Empty);
    }
    private void Move(RangeEndpoint endpoint, double delta)
    {
        var width = ActualWidth - endpoint.Width;
        if (width > 0) endpoint.Value = Math.Clamp(endpoint.Value + delta / width * (endpoint.Maximum - endpoint.Minimum), endpoint.Minimum, endpoint.Maximum);
    }
    protected override void RefreshAppearance()
    {
        Height = Theme.Number($"control.{Theme.Key(Size)}.size");
        foreach (var endpoint in new[] { Lower, Upper })
        {
            endpoint.Width = endpoint.Height = Theme.Number($"slider.{Skin.SizeKey(Size)}.thumb");
            endpoint.Background = Theme.Brush("color.surface");
            endpoint.BorderBrush = Theme.Brush("color.border");
            endpoint.BorderThickness = new Thickness(Theme.Number("stroke.hairline"));
            endpoint.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
            Skin.Focus(endpoint, Theme);
        }
        groove.Background = Theme.Brush("color.surface-sunken");
        selection.Background = Theme.Recipe(ControlVariant.Filled, Tint, "surface", IsEnabled ? "normal" : "disabled");
        groove.Height = selection.Height = Theme.Number($"slider.{Skin.SizeKey(Size)}.track");
        groove.CornerRadius = selection.CornerRadius = new CornerRadius(Theme.Number("radius.full"));
        ArrangeTrack();
    }
    private void ArrangeTrack()
    {
        var diameter = double.IsNaN(Lower.Width) ? 0 : Lower.Width;
        var width = Math.Max(0, ActualWidth - diameter);
        double Position(RangeEndpoint endpoint) => endpoint.Maximum <= endpoint.Minimum ? 0 : (endpoint.Value - endpoint.Minimum) / (endpoint.Maximum - endpoint.Minimum) * width;
        var start = Position(Lower); var end = Position(Upper);
        groove.Width = width; selection.Width = Math.Max(0, end - start);
        Canvas.SetLeft(groove, diameter / 2); Canvas.SetLeft(selection, start + diameter / 2);
        Canvas.SetTop(groove, (ActualHeight - groove.Height) / 2); Canvas.SetTop(selection, (ActualHeight - selection.Height) / 2);
        Canvas.SetLeft(Lower, start); Canvas.SetLeft(Upper, end);
        Canvas.SetTop(Lower, (ActualHeight - diameter) / 2); Canvas.SetTop(Upper, (ActualHeight - diameter) / 2);
    }
}

public sealed class RangeEndpoint : RangeBase
{
    internal Action<double> Drag { get; set; } = _ => { };
    public RangeEndpoint()
    {
        DefaultStyleKey = typeof(RangeEndpoint);
        IsTabStop = true; Maximum = 100; SmallChange = 1; LargeChange = 10;
        KeyDown += (_, args) =>
        {
            double? next = args.Key switch
            {
                VirtualKey.Left or VirtualKey.Down => Value - SmallChange,
                VirtualKey.Right or VirtualKey.Up => Value + SmallChange,
                VirtualKey.PageDown => Value - LargeChange,
                VirtualKey.PageUp => Value + LargeChange,
                VirtualKey.Home => Minimum,
                VirtualKey.End => Maximum,
                _ => null,
            };
            if (next is null) return;
            Value = Math.Clamp(next.Value, Minimum, Maximum); args.Handled = true;
        };
    }
    protected override void OnApplyTemplate()
    {
        base.OnApplyTemplate();
        if (GetTemplateChild("Handle") is Thumb thumb)
        {
            thumb.DragStarted += (_, _) => Focus(FocusState.Pointer);
            thumb.DragDelta += (_, args) => Drag(args.HorizontalChange);
        }
    }
    protected override AutomationPeer OnCreateAutomationPeer() => new EndpointPeer(this);
    private sealed class EndpointPeer(RangeEndpoint owner) : FrameworkElementAutomationPeer(owner), IRangeValueProvider
    {
        public bool IsReadOnly => !owner.IsEnabled;
        public double Minimum => owner.Minimum;
        public double Maximum => owner.Maximum;
        public double Value => owner.Value;
        public double SmallChange => owner.SmallChange;
        public double LargeChange => owner.LargeChange;
        public void SetValue(double value)
        {
            if (IsReadOnly) throw new InvalidOperationException("The range slider is disabled.");
            if (!double.IsFinite(value) || value < Minimum || value > Maximum) throw new ArgumentOutOfRangeException(nameof(value));
            owner.Value = value;
        }
        protected override object GetPatternCore(PatternInterface pattern) => pattern == PatternInterface.RangeValue ? this : base.GetPatternCore(pattern);
        protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.Slider;
    }
}
