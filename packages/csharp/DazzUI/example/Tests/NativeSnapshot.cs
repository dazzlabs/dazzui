using System.ComponentModel;
using System.Runtime.InteropServices;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Automation.Peers;
using Windows.Graphics.Imaging;
using Windows.Storage;

namespace DazzUI.Example;

/// <summary>Capture the visible native popup, which RenderTargetBitmap cannot read across its island.</summary>
internal static class NativeSnapshot
{
    [StructLayout(LayoutKind.Sequential)]
    private struct NativePoint { internal int X, Y; }
    [DllImport("user32.dll")] private static extern bool ClientToScreen(IntPtr window, ref NativePoint point);
    [StructLayout(LayoutKind.Sequential)]
    private struct BitmapInfo
    {
        internal uint Size;
        internal int Width, Height;
        internal ushort Planes, BitCount;
        internal uint Compression, SizeImage;
        internal int XPelsPerMeter, YPelsPerMeter;
        internal uint ClrUsed, ClrImportant;
    }
    [DllImport("user32.dll")] private static extern IntPtr GetDC(IntPtr window);
    [DllImport("user32.dll")] private static extern int ReleaseDC(IntPtr window, IntPtr dc);
    [DllImport("gdi32.dll")] private static extern IntPtr CreateCompatibleDC(IntPtr dc);
    [DllImport("gdi32.dll")] private static extern bool DeleteDC(IntPtr dc);
    [DllImport("gdi32.dll")] private static extern IntPtr SelectObject(IntPtr dc, IntPtr item);
    [DllImport("gdi32.dll")] private static extern bool DeleteObject(IntPtr item);
    [DllImport("gdi32.dll", SetLastError = true)] private static extern IntPtr CreateDIBSection(IntPtr dc, ref BitmapInfo info, uint usage, out IntPtr bits, IntPtr section, uint offset);
    [DllImport("gdi32.dll", SetLastError = true)] private static extern bool BitBlt(IntPtr destination, int x, int y, int width, int height, IntPtr source, int sourceX, int sourceY, uint operation);

    internal static async Task SaveAsync(FrameworkElement popup, string directory, string name, double outset = 0)
    {
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(popup);
        var transform = popup.TransformToVisual(popup.XamlRoot.Content);
        var origin = transform.TransformPoint(default);
        var visualBounds = transform.TransformBounds(new Windows.Foundation.Rect(0, 0, popup.ActualWidth, popup.ActualHeight));
        var scale = popup.XamlRoot.RasterizationScale;
        // Native Borders have no automation peer. Their actual visual bounds
        // use the same root, converted once from logical to physical pixels.
        var bounds = peer?.GetBoundingRectangle() ?? new Windows.Foundation.Rect(
            visualBounds.X * scale, visualBounds.Y * scale, visualBounds.Width * scale, visualBounds.Height * scale);
        var appWindow = Microsoft.UI.Windowing.AppWindow.GetFromWindowId(popup.XamlRoot.ContentIslandEnvironment.AppWindowId);
        await File.WriteAllTextAsync(Path.Combine(directory, name + ".json"), System.Text.Json.JsonSerializer.Serialize(new
        {
            bounds.X, bounds.Y, bounds.Width, bounds.Height,
            boundsSource = peer is null ? "visual" : "automation",
            root = popup.XamlRoot.Content.GetType().Name, scale = popup.XamlRoot.RasterizationScale,
            originX = origin.X, originY = origin.Y, windowX = appWindow.Position.X, windowY = appWindow.Position.Y,
        }));
        // A direct WinUI peer reading is in root-relative physical pixels;
        // the external UIA provider normally performs this screen translation.
        var screenOrigin = new NativePoint();
        var window = Microsoft.UI.Win32Interop.GetWindowFromWindowId(popup.XamlRoot.ContentIslandEnvironment.AppWindowId);
        if (!ClientToScreen(window, ref screenOrigin)) throw new Win32Exception(Marshal.GetLastWin32Error());
        var left = (int)Math.Floor(bounds.X);
        var top = (int)Math.Floor(bounds.Y);
        var margin = (int)Math.Ceiling(outset * popup.XamlRoot.RasterizationScale);
        var x = left + screenOrigin.X - margin;
        var y = top + screenOrigin.Y - margin;
        var width = (int)Math.Ceiling(bounds.Right) - left + 2 * margin;
        var height = (int)Math.Ceiling(bounds.Bottom) - top + 2 * margin;
        if (width <= 0 || height <= 0) throw new InvalidOperationException("The native popup has empty screen bounds.");
        var pixels = new byte[checked(width * height * 4)];
        var screen = GetDC(IntPtr.Zero);
        var memory = CreateCompatibleDC(screen);
        var info = new BitmapInfo { Size = (uint)Marshal.SizeOf<BitmapInfo>(), Width = width, Height = -height, Planes = 1, BitCount = 32 };
        var bitmap = CreateDIBSection(screen, ref info, 0, out var bits, IntPtr.Zero, 0);
        var previous = IntPtr.Zero;
        try
        {
            if (screen == IntPtr.Zero || memory == IntPtr.Zero || bitmap == IntPtr.Zero) throw new Win32Exception(Marshal.GetLastWin32Error());
            previous = SelectObject(memory, bitmap);
            if (!BitBlt(memory, 0, 0, width, height, screen, x, y, 0x40CC0020)) throw new Win32Exception(Marshal.GetLastWin32Error());
            Marshal.Copy(bits, pixels, 0, pixels.Length);
            for (var alpha = 3; alpha < pixels.Length; alpha += 4) pixels[alpha] = 255;
        }
        finally
        {
            if (previous != IntPtr.Zero) SelectObject(memory, previous);
            if (bitmap != IntPtr.Zero) DeleteObject(bitmap);
            if (memory != IntPtr.Zero) DeleteDC(memory);
            if (screen != IntPtr.Zero) ReleaseDC(IntPtr.Zero, screen);
        }
        var folder = await StorageFolder.GetFolderFromPathAsync(Path.GetFullPath(directory));
        var file = await folder.CreateFileAsync(name, CreationCollisionOption.ReplaceExisting);
        using var stream = await file.OpenAsync(FileAccessMode.ReadWrite);
        var encoder = await BitmapEncoder.CreateAsync(BitmapEncoder.PngEncoderId, stream);
        encoder.SetPixelData(BitmapPixelFormat.Bgra8, BitmapAlphaMode.Ignore, (uint)width, (uint)height, 96, 96, pixels);
        await encoder.FlushAsync();
    }
}
