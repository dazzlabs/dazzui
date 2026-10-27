using System.Collections.Concurrent;
using System.Runtime.InteropServices;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Controls;
using Windows.Foundation;
using Windows.UI.Text;

namespace DazzUI;

/// <summary>The Windows system-ui face, which can differ from XAML's app-language default.</summary>
internal static class PlatformFonts
{
    internal static FontFamily Ui { get; private set; } = ReadUi();
    internal static void Refresh() { Ui = ReadUi(); Metrics.Clear(); }
    // The Windows ui-monospace face used by the browser's code-font role.
    internal static FontFamily Code { get; } = new("Consolas");
    private static readonly ConcurrentDictionary<string, ushort[]> Weights = new(StringComparer.OrdinalIgnoreCase);
    private static readonly ConcurrentDictionary<(string Family, double Size, ushort Weight), (double Ascent, double Descent)> Metrics = new();

    internal static double LineBaseline(FontFamily family, double size, FontWeight weight, double height)
    {
        var metrics = Metrics.GetOrAdd((family.Source, size, weight.Weight), key =>
        {
            var probe = new TextBlock
            {
                IsTextScaleFactorEnabled = false, Text = "x", FontFamily = family, FontSize = size, FontWeight = weight, UseLayoutRounding = false,
            };
            probe.Measure(new Size(double.PositiveInfinity, double.PositiveInfinity));
            return (Math.Round(probe.BaselineOffset, MidpointRounding.AwayFromZero),
                Math.Round(probe.DesiredSize.Height - probe.BaselineOffset, MidpointRounding.AwayFromZero));
        });
        // Blink uses integer font ascent/descent and floors the half-leading
        // above the baseline. BlockLineHeight instead scales the ascent ratio
        // to the entire line; those differ when a font's natural height does
        // not equal its declared leading.
        return metrics.Ascent + Math.Floor((height - metrics.Ascent - metrics.Descent) / 2);
    }

    // CSS matches a missing weight directionally, whereas DirectWrite may
    // select the nearer face. For example, YaHei UI has 400 and 700: CSS's
    // 600 is Bold, not the narrower fallback WinUI chooses for Semibold.
    internal static FontWeight MatchWeight(FontFamily family, FontWeight requested)
    {
        var name = family.Source;
        // Leave packaged fonts, fallback stacks and variable faces to XAML.
        if (name.Contains("Variable", StringComparison.OrdinalIgnoreCase)
            || !string.Equals(name, Ui.Source, StringComparison.OrdinalIgnoreCase)
                && !string.Equals(name, Code.Source, StringComparison.OrdinalIgnoreCase)) return requested;
        var weights = Weights.GetOrAdd(name, ReadWeights);
        if (weights.Length == 0 || weights.Contains(requested.Weight)) return requested;
        var weight = requested.Weight;
        IEnumerable<ushort> candidates = weight < 400
            ? weights.Where(w => w < weight).Reverse().Concat(weights.Where(w => w > weight))
            : weight > 500
                ? weights.Where(w => w > weight).Concat(weights.Where(w => w < weight).Reverse())
                : weights.Where(w => w >= weight && w <= 500)
                    .Concat(weights.Where(w => w < weight).Reverse())
                    .Concat(weights.Where(w => w > 500));
        return new FontWeight { Weight = candidates.First() };
    }

    private static ushort[] ReadWeights(string family)
    {
        var result = new SortedSet<ushort>();
        var dc = CreateCompatibleDC(IntPtr.Zero);
        if (dc == IntPtr.Zero) return [];
        try
        {
            var query = new LogicalFont { FaceName = family, CharacterSet = 1 };
            FontCallback callback = (font, _, _, _) =>
            {
                var face = Marshal.PtrToStructure<LogicalFont>(font);
                if (face.Italic == 0 && face.Weight is > 0 and <= 1000)
                    result.Add((ushort)face.Weight);
                return 1;
            };
            EnumFontFamiliesEx(dc, ref query, callback, IntPtr.Zero, 0);
            GC.KeepAlive(callback);
        }
        finally { DeleteDC(dc); }
        return result.ToArray();
    }

    private delegate int FontCallback(IntPtr font, IntPtr metrics, uint type, IntPtr data);
    [DllImport("gdi32.dll", ExactSpelling = true)]
    private static extern IntPtr CreateCompatibleDC(IntPtr dc);
    [DllImport("gdi32.dll", ExactSpelling = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool DeleteDC(IntPtr dc);
    [DllImport("gdi32.dll", EntryPoint = "EnumFontFamiliesExW", ExactSpelling = true, CharSet = CharSet.Unicode)]
    private static extern int EnumFontFamiliesEx(IntPtr dc, ref LogicalFont font, FontCallback callback, IntPtr data, uint flags);

    private static FontFamily ReadUi()
    {
        var metrics = new NonClientMetrics { Size = (uint)Marshal.SizeOf<NonClientMetrics>() };
        // Chromium's Windows system-ui resolves the message font too. Read the
        // OS choice rather than substituting a face based on locale ourselves.
        if (SystemParametersInfo(0x29, metrics.Size, ref metrics, 0) && !string.IsNullOrWhiteSpace(metrics.MessageFont.FaceName))
            return new FontFamily(metrics.MessageFont.FaceName);
        return FontFamily.XamlAutoFontFamily;
    }

    [DllImport("user32.dll", EntryPoint = "SystemParametersInfoW", ExactSpelling = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool SystemParametersInfo(uint action, uint parameter, ref NonClientMetrics metrics, uint flags);

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    private struct LogicalFont
    {
        public int Height, Width, Escapement, Orientation, Weight;
        public byte Italic, Underline, StrikeOut, CharacterSet, OutPrecision, ClipPrecision, Quality, PitchAndFamily;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)] public string FaceName;
    }

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    private struct NonClientMetrics
    {
        public uint Size;
        public int BorderWidth, ScrollWidth, ScrollHeight, CaptionWidth, CaptionHeight;
        public LogicalFont CaptionFont;
        public int SmallCaptionWidth, SmallCaptionHeight;
        public LogicalFont SmallCaptionFont;
        public int MenuWidth, MenuHeight;
        public LogicalFont MenuFont, StatusFont, MessageFont;
        public int PaddedBorderWidth;
    }
}
