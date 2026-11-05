using System.Runtime.InteropServices;
using Microsoft.UI.Xaml;

namespace DazzUI.Example;

/// <summary>Send native test keys only to the verified fixture window.</summary>
internal static class NativeKeyboard
{
    [DllImport("user32.dll")]
    private static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool SetForegroundWindow(IntPtr window);
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool BringWindowToTop(IntPtr window);
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool IsWindowVisible(IntPtr window);
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool IsIconic(IntPtr window);
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool ShowWindow(IntPtr window, int command);
    [DllImport("user32.dll")]
    private static extern uint GetWindowThreadProcessId(IntPtr window, out uint process);
    [DllImport("kernel32.dll")]
    private static extern uint GetCurrentThreadId();
    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool AttachThreadInput(uint from, uint to, [MarshalAs(UnmanagedType.Bool)] bool attach);
    [DllImport("user32.dll")]
    private static extern void keybd_event(byte key, byte scan, uint flags, UIntPtr extraInfo);
    [DllImport("user32.dll", SetLastError = true)]
    private static extern uint SendInput(uint count, NativeInput[] inputs, int size);

    [StructLayout(LayoutKind.Sequential)]
    private struct NativeInput
    {
        public uint Type;
        public InputData Data;
    }
    [StructLayout(LayoutKind.Explicit)]
    private struct InputData
    {
        [FieldOffset(0)] public KeyboardInput Keyboard;
        [FieldOffset(0)] public MouseInput Mouse;
    }
    [StructLayout(LayoutKind.Sequential)]
    private struct KeyboardInput
    {
        public ushort Key;
        public ushort Scan;
        public uint Flags;
        public uint Time;
        public UIntPtr ExtraInfo;
    }
    [StructLayout(LayoutKind.Sequential)]
    private struct MouseInput
    {
        public int X;
        public int Y;
        public uint MouseData;
        public uint Flags;
        public uint Time;
        public UIntPtr ExtraInfo;
    }

    private static IntPtr Window(FrameworkElement owner)
    {
        var root = owner.XamlRoot ?? throw new InvalidOperationException("The keyboard fixture is not attached to a native window.");
        var window = Microsoft.UI.Win32Interop.GetWindowFromWindowId(root.ContentIslandEnvironment.AppWindowId);
        if (window == IntPtr.Zero) throw new InvalidOperationException("The keyboard fixture has no native window handle.");
        return window;
    }

    internal static async Task ActivateAsync(FrameworkElement owner)
    {
        var window = Window(owner);
        if (IsIconic(window)) ShowWindow(window, 9);
        else if (!IsWindowVisible(window)) ShowWindow(window, 5);
        SetForegroundWindow(window);
        for (var attempt = 0; attempt < 5 && GetForegroundWindow() != window; attempt++)
        {
            // A test launched in the background can have XAML focus without
            // owning the Windows input queue. Join only for activation, then
            // detach before the test sends any key.
            var foreground = GetForegroundWindow();
            var otherThread = GetWindowThreadProcessId(foreground, out _);
            var thisThread = GetCurrentThreadId();
            var attached = otherThread != 0 && otherThread != thisThread && AttachThreadInput(thisThread, otherThread, true);
            try
            {
                BringWindowToTop(window);
                SetForegroundWindow(window);
            }
            finally
            {
                if (attached) AttachThreadInput(thisThread, otherThread, false);
            }
            await Task.Delay(40);
        }
        Verify(owner);
    }

    private static void Verify(FrameworkElement owner)
    {
        var window = Window(owner);
        var foreground = GetForegroundWindow();
        if (foreground != window)
            throw new InvalidOperationException($"Native keyboard fixture is not foreground (expected 0x{window.ToInt64():X}, actual 0x{foreground.ToInt64():X}); no key was sent.");
    }

    internal static void Press(FrameworkElement owner, byte key)
    {
        Verify(owner);
        keybd_event(key, 0, 0, UIntPtr.Zero);
        keybd_event(key, 0, 2, UIntPtr.Zero);
    }

    internal static void ReplaceText(FrameworkElement owner, string text)
    {
        Verify(owner);
        var events = new List<NativeInput>();
        static NativeInput Key(ushort key, ushort scan, uint flags) => new()
        {
            Type = 1,
            Data = new InputData { Keyboard = new KeyboardInput { Key = key, Scan = scan, Flags = flags } },
        };
        events.Add(Key(0x11, 0, 0));
        events.Add(Key(0x41, 0, 0));
        events.Add(Key(0x41, 0, 2));
        events.Add(Key(0x11, 0, 2));
        foreach (var character in text)
        {
            events.Add(Key(0, character, 4));
            events.Add(Key(0, character, 6));
        }
        if (text.Length == 0)
        {
            events.Add(Key(0x08, 0, 0));
            events.Add(Key(0x08, 0, 2));
        }
        var inputs = events.ToArray();
        var sent = SendInput((uint)inputs.Length, inputs, Marshal.SizeOf<NativeInput>());
        if (sent != inputs.Length)
        {
            // A partial injection must not leave the Select All modifier held.
            keybd_event(0x11, 0, 2, UIntPtr.Zero);
            throw new InvalidOperationException($"Native text input accepted {sent}/{inputs.Length} events (Win32 error {Marshal.GetLastWin32Error()}).");
        }
    }
}
