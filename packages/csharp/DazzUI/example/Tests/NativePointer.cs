using System.Runtime.InteropServices;
using Microsoft.UI.Xaml;

namespace DazzUI.Example;

/// <summary>Move the test pointer in physical client coordinates and restore it.</summary>
internal sealed class NativePointer : IDisposable
{
    [StructLayout(LayoutKind.Sequential)]
    private struct Point { internal int X, Y; }
    [DllImport("user32.dll")]
    private static extern bool GetCursorPos(out Point point);
    [DllImport("user32.dll")]
    private static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")]
    private static extern bool ClientToScreen(IntPtr window, ref Point point);
    [DllImport("user32.dll")]
    private static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")]
    private static extern int GetSystemMetrics(int index);
    [StructLayout(LayoutKind.Sequential)]
    private struct MouseInput { internal int X, Y; internal uint Data, Flags, Time; internal UIntPtr Extra; }
    [StructLayout(LayoutKind.Explicit, Size = 32)]
    private struct InputData { [FieldOffset(0)] internal MouseInput Mouse; }
    [StructLayout(LayoutKind.Sequential)]
    private struct Input { internal uint Type; internal InputData Data; }
    [DllImport("user32.dll", SetLastError = true)]
    private static extern uint SendInput(uint count, Input[] inputs, int size);
    private readonly Point original;
    internal NativePointer()
    {
        if (!GetCursorPos(out original)) throw new InvalidOperationException("Could not read the test pointer position.");
    }
    internal void MoveTo(FrameworkElement element)
    {
        var root = element.XamlRoot ?? throw new InvalidOperationException("The pointer target is not mounted.");
        var window = Microsoft.UI.Win32Interop.GetWindowFromWindowId(root.ContentIslandEnvironment.AppWindowId);
        if (GetForegroundWindow() != window) throw new InvalidOperationException("The pointer fixture window is not foreground; no pointer was moved.");
        var point = element.TransformToVisual(root.Content).TransformPoint(new Windows.Foundation.Point(element.ActualWidth / 2, element.ActualHeight / 2));
        var screen = new Point { X = (int)Math.Round(point.X * root.RasterizationScale), Y = (int)Math.Round(point.Y * root.RasterizationScale) };
        if (!ClientToScreen(window, ref screen))
            throw new InvalidOperationException("Could not move the test pointer to the native target.");
        var move = new Input { Data = new InputData { Mouse = new MouseInput
        {
            X = (int)Math.Round((screen.X - GetSystemMetrics(76)) * 65535d / (GetSystemMetrics(78) - 1)),
            Y = (int)Math.Round((screen.Y - GetSystemMetrics(77)) * 65535d / (GetSystemMetrics(79) - 1)),
            Flags = 0x8000 | 0x4000 | 0x0001,
        } } };
        if (SendInput(1, [move], Marshal.SizeOf<Input>()) != 1)
            throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
    }
    public void Dispose() => SetCursorPos(original.X, original.Y);
}
