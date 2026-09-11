using Microsoft.UI.Xaml;
using Microsoft.UI.Input;
using Windows.System;
using Windows.UI.Core;

namespace DazzUI;

public sealed class ShortcutRecorder : Button
{
    private string shortcut = "";
    private bool recording;
    public string Shortcut { get => shortcut; set { shortcut = value; Content = string.IsNullOrEmpty(value) ? "Record shortcut" : value; } }
    public Func<string, string?>? Validate { get; set; }
    public string? Error { get; private set; }
    public event EventHandler<string>? ShortcutChanged;
    public ShortcutRecorder()
    {
        Variant = ControlVariant.Recessed;
        Click += (_, _) => { recording = true; Content = "Press shortcut…"; Focus(FocusState.Keyboard); };
        LostFocus += (_, _) => { recording = false; Shortcut = shortcut; };
        KeyDown += (_, args) =>
        {
            if (!recording) return;
            args.Handled = true;
            if (args.Key == VirtualKey.Escape) { recording = false; Shortcut = shortcut; return; }
            if (args.Key is VirtualKey.Control or VirtualKey.Shift or VirtualKey.Menu or VirtualKey.LeftWindows or VirtualKey.RightWindows) return;
            var modifiers = new List<string>();
            if (Down(VirtualKey.Control)) modifiers.Add("Ctrl");
            if (Down(VirtualKey.Menu)) modifiers.Add("Alt");
            if (Down(VirtualKey.Shift)) modifiers.Add("Shift");
            if (Down(VirtualKey.LeftWindows) || Down(VirtualKey.RightWindows)) modifiers.Add("Win");
            if (args.Key is VirtualKey.Back or VirtualKey.Delete && modifiers.Count == 0) Shortcut = "";
            else
            {
                modifiers.Add(args.Key.ToString()); var candidate = string.Join('+', modifiers);
                Error = Validate?.Invoke(candidate);
                if (Error is not null) { Content = Error; return; }
                Shortcut = candidate;
            }
            recording = false; ShortcutChanged?.Invoke(this, Shortcut);
        };
        Loaded += (_, _) => Width = Theme.Number("shortcut-recorder.width");
    }
    private static bool Down(VirtualKey key) => InputKeyboardSource.GetKeyStateForCurrentThread(key).HasFlag(CoreVirtualKeyStates.Down);
}
