using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Media.Animation;

namespace DazzUI.Example;

/// <summary>
/// A panel's width over the motion token: set it while dragging, animate it
/// when a column opens or closes, and stop cleanly when the window changes
/// its mind halfway.
/// </summary>
public sealed class WidthAnimation
{
    private readonly FrameworkElement element;
    private Storyboard? run;
    private int version;

    public WidthAnimation(FrameworkElement element) => this.element = element;

    public bool Animating => run is not null;

    /// <summary>Width now, with no animation — a drag, or the first layout.</summary>
    public void Set(double width)
    {
        Stop();
        element.Width = width;
    }

    /// <summary>Width over the motion token, from where the element stands now.</summary>
    public void AnimateTo(double width, Theme theme, Action? completed = null)
    {
        Stop();
        var from = element.ActualWidth > 0 ? element.ActualWidth : element.Width;
        if (double.IsNaN(from) || Math.Abs(from - width) < 0.5 || !element.IsLoaded)
        {
            element.Width = width;
            completed?.Invoke();
            return;
        }
        element.Width = from;
        var animation = new DoubleAnimation
        {
            From = from,
            To = width,
            Duration = TimeSpan.FromMilliseconds(theme.Milliseconds("motion.duration")),
            EasingFunction = new CubicEase { EasingMode = EasingMode.EaseInOut },
            EnableDependentAnimation = true,
        };
        Storyboard.SetTarget(animation, element);
        Storyboard.SetTargetProperty(animation, "Width");
        run = new Storyboard { Children = { animation } };
        var current = ++version;
        run.Completed += (_, _) =>
        {
            if (current != version) return;
            run = null;
            element.Width = width;
            completed?.Invoke();
        };
        run.Begin();
    }

    private void Stop()
    {
        version++;
        run?.Stop();
        run = null;
    }
}
