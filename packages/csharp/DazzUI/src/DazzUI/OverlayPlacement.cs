using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Windows.Foundation;

namespace DazzUI;

public enum PopupSide { Top, Bottom, Left, Right }
public enum PopupAlignment { Start, Center, End }

/// <summary>Expresses shared placement through native anchoring and collision handling.</summary>
internal static class OverlayPlacement
{
    internal static FlyoutPlacementMode Resolve(PopupSide side, PopupAlignment align) => (side, align) switch
    {
        (PopupSide.Top, PopupAlignment.Start) => FlyoutPlacementMode.TopEdgeAlignedLeft,
        (PopupSide.Top, PopupAlignment.End) => FlyoutPlacementMode.TopEdgeAlignedRight,
        (PopupSide.Top, _) => FlyoutPlacementMode.Top,
        (PopupSide.Bottom, PopupAlignment.Start) => FlyoutPlacementMode.BottomEdgeAlignedLeft,
        (PopupSide.Bottom, PopupAlignment.End) => FlyoutPlacementMode.BottomEdgeAlignedRight,
        (PopupSide.Bottom, _) => FlyoutPlacementMode.Bottom,
        (PopupSide.Left, PopupAlignment.Start) => FlyoutPlacementMode.LeftEdgeAlignedTop,
        (PopupSide.Left, PopupAlignment.End) => FlyoutPlacementMode.LeftEdgeAlignedBottom,
        (PopupSide.Left, _) => FlyoutPlacementMode.Left,
        (PopupSide.Right, PopupAlignment.Start) => FlyoutPlacementMode.RightEdgeAlignedTop,
        (PopupSide.Right, PopupAlignment.End) => FlyoutPlacementMode.RightEdgeAlignedBottom,
        _ => FlyoutPlacementMode.Right,
    };
    internal static PopupSide Side(FlyoutPlacementMode placement) => placement switch
    {
        FlyoutPlacementMode.Top or FlyoutPlacementMode.TopEdgeAlignedLeft or FlyoutPlacementMode.TopEdgeAlignedRight => PopupSide.Top,
        FlyoutPlacementMode.Left or FlyoutPlacementMode.LeftEdgeAlignedTop or FlyoutPlacementMode.LeftEdgeAlignedBottom => PopupSide.Left,
        FlyoutPlacementMode.Right or FlyoutPlacementMode.RightEdgeAlignedTop or FlyoutPlacementMode.RightEdgeAlignedBottom => PopupSide.Right,
        _ => PopupSide.Bottom,
    };
    internal static PopupAlignment Align(FlyoutPlacementMode placement) => placement switch
    {
        FlyoutPlacementMode.TopEdgeAlignedLeft or FlyoutPlacementMode.BottomEdgeAlignedLeft
            or FlyoutPlacementMode.LeftEdgeAlignedTop or FlyoutPlacementMode.RightEdgeAlignedTop => PopupAlignment.Start,
        FlyoutPlacementMode.TopEdgeAlignedRight or FlyoutPlacementMode.BottomEdgeAlignedRight
            or FlyoutPlacementMode.LeftEdgeAlignedBottom or FlyoutPlacementMode.RightEdgeAlignedBottom => PopupAlignment.End,
        _ => PopupAlignment.Center,
    };

    internal static void Show(FlyoutBase flyout, FrameworkElement anchor)
    {
        // WinUI's element-target FlyoutMargin is the same four-pixel spacing.1
        // used by popup.css. Keep this path: point-target MenuFlyouts ignore
        // Placement and would lose native edge alignment and collision flips.
        flyout.ShowAt(anchor);
    }

    internal static void Hint(ToolTip hint, FrameworkElement target, PlacementMode side, PopupAlignment align)
    {
        var gap = Appearance.GetTheme(target).Number("spacing.1");
        var horizontal = side is PlacementMode.Left or PlacementMode.Right;
        var width = target.ActualWidth;
        var height = target.ActualHeight;
        var x = 0d;
        var y = 0d;
        // The native positioner centers on PlacementRect. A cross-axis anchor
        // box the size of the hint expresses start/end without moving a popup
        // ourselves; WinUI still chooses the fitting side and clamps the box.
        if (align != PopupAlignment.Center)
        {
            if (horizontal && hint.ActualHeight > 0)
            {
                height = hint.ActualHeight;
                if (align == PopupAlignment.End) y = target.ActualHeight - height;
            }
            else if (!horizontal && hint.ActualWidth > 0)
            {
                width = hint.ActualWidth;
                if (align == PopupAlignment.End) x = target.ActualWidth - width;
            }
        }
        var anchor = new Rect(x, y, width, height);
        if (target.XamlRoot?.Content is UIElement root)
        {
            // ToolTip's native positioner converts the root-space anchor to
            // integer logical RECT coordinates. Truncating both fractional
            // edges biases its center toward the origin (over a pixel once
            // the popup's odd width is centered too). Round the cross-axis
            // anchor before that conversion, retaining native side/collision
            // handling and leaving the requested main-axis gap untouched.
            var inRoot = target.TransformToVisual(root).TransformBounds(anchor);
            if (horizontal)
            {
                var top = Math.Round(inRoot.Top);
                inRoot = new Rect(inRoot.Left, top, inRoot.Width, Math.Round(inRoot.Bottom) - top);
            }
            else
            {
                var left = Math.Round(inRoot.Left);
                inRoot = new Rect(left, inRoot.Top, Math.Round(inRoot.Right) - left, inRoot.Height);
            }
            anchor = root.TransformToVisual(target).TransformBounds(inRoot);
        }
        hint.PlacementTarget = target;
        hint.Placement = side;
        hint.HorizontalOffset = horizontal ? gap : 0;
        hint.VerticalOffset = horizontal ? 0 : gap;
        hint.PlacementRect = anchor;
    }
}
