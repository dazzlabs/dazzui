using System.Collections.Concurrent;
using System.Buffers.Binary;
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
    private static readonly ConcurrentDictionary<(char Character, int Size, ushort Weight), int> BitmapSpacing = new();
    private static readonly ConcurrentDictionary<(string Family, char Character), bool> Glyphs = new();

    internal static bool MathFallback(char character) => !HasGlyph(Ui.Source, character) && HasGlyph("Cambria Math", character);
    private static bool HasGlyph(string family, char character) => Glyphs.GetOrAdd((family, character), key =>
    {
        var dc = CreateCompatibleDC(IntPtr.Zero);
        if (dc == IntPtr.Zero) return false;
        var font = CreateFont(-12, 0, 0, 0, 400, 0, 0, 0, 1, 7, 0, 6, 0, key.Family);
        var old = SelectObject(dc, font);
        try
        {
            var glyph = new ushort[1];
            return GetGlyphIndices(dc, key.Character.ToString(), 1, glyph, 1) != uint.MaxValue && glyph[0] != ushort.MaxValue;
        }
        finally { SelectObject(dc, old); DeleteObject(font); DeleteDC(dc); }
    });

    // Chromium honors Cambria Math's embedded bitmap strikes; XAML uses its
    // outline advances. Read the installed font's strike sizes and GDI advance
    // rather than baking a correction for any particular glyph or theme.
    internal static int BitmapTracking(char character, double size, FontWeight weight)
    {
        var pixels = (int)Math.Round(size);
        if (pixels <= 0 || Math.Abs(size - pixels) > .001) return 0;
        return BitmapSpacing.GetOrAdd((character, pixels, weight.Weight), key =>
        {
            var dc = CreateCompatibleDC(IntPtr.Zero);
            if (dc == IntPtr.Zero) return 0;
            var font = CreateFont(-key.Size, 0, 0, 0, key.Weight, 0, 0, 0, 1, 7, 0, 6, 0, "Cambria Math");
            var old = SelectObject(dc, font);
            try
            {
                // OpenType EBLC: 8-byte header, then 48-byte bitmapSizeTables;
                // ppemX and ppemY are bytes 44 and 45 of each size table.
                const uint eblc = 0x434c4245;
                var header = new byte[8];
                if (GetFontData(dc, eblc, 0, header, 8) != 8) return 0;
                var count = BinaryPrimitives.ReadUInt32BigEndian(header.AsSpan(4));
                if (count is 0 or > 256) return 0;
                var tables = new byte[count * 48];
                if (GetFontData(dc, eblc, 8, tables, (uint)tables.Length) != tables.Length) return 0;
                var strike = false;
                for (var i = 0; i < count; i++)
                    if (tables[i * 48 + 44] == key.Size && tables[i * 48 + 45] == key.Size) { strike = true; break; }
                // Bitmap mode grid-fits advances for the whole strike,
                // including glyphs whose outline is used at that size.
                if (!strike || !GetTextExtentPoint32(dc, key.Character.ToString(), 1, out var advance)) return 0;
                var glyph = new ushort[1];
                if (GetGlyphIndices(dc, key.Character.ToString(), 1, glyph, 1) == uint.MaxValue || glyph[0] == ushort.MaxValue) return 0;
                var outline = new TextBlock
                {
                    Text = key.Character.ToString(), FontFamily = new FontFamily("Cambria Math"), FontSize = key.Size,
                    FontWeight = new FontWeight { Weight = key.Weight }, UseLayoutRounding = false, IsTextScaleFactorEnabled = false,
                };
                outline.Measure(new Size(double.PositiveInfinity, double.PositiveInfinity));
                return (int)Math.Round((advance.Width - outline.DesiredSize.Width) * 1000 / key.Size);
            }
            finally
            {
                SelectObject(dc, old);
                DeleteObject(font);
                DeleteDC(dc);
            }
        });
    }

    internal static double InlineHeight(FontFamily family, double size, FontWeight weight)
    {
        // An inline anchor's box is the rounded font ascent and descent,
        // while its paragraph still owns the body's declared line height.
        LineBaseline(family, size, weight, size);
        var metrics = Metrics[(family.Source, size, weight.Weight)];
        return metrics.Ascent + metrics.Descent;
    }

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
        var name = family.Source.Split(',')[0].Trim();
        // Leave packaged fonts, other families and variable faces to XAML.
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
    [StructLayout(LayoutKind.Sequential)]
    private struct FontSize { public int Width, Height; }
    [DllImport("gdi32.dll", EntryPoint = "CreateFontW", ExactSpelling = true, CharSet = CharSet.Unicode)]
    private static extern IntPtr CreateFont(int height, int width, int escapement, int orientation, int weight, uint italic, uint underline, uint strikeout, uint charset, uint output, uint clip, uint quality, uint pitch, string family);
    [DllImport("gdi32.dll", ExactSpelling = true)]
    private static extern IntPtr SelectObject(IntPtr dc, IntPtr value);
    [DllImport("gdi32.dll", ExactSpelling = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool DeleteObject(IntPtr value);
    [DllImport("gdi32.dll", ExactSpelling = true)]
    private static extern uint GetFontData(IntPtr dc, uint table, uint offset, [Out] byte[]? data, uint length);
    [DllImport("gdi32.dll", EntryPoint = "GetGlyphIndicesW", ExactSpelling = true, CharSet = CharSet.Unicode)]
    private static extern uint GetGlyphIndices(IntPtr dc, string text, int length, [Out] ushort[] glyphs, uint flags);
    [DllImport("gdi32.dll", EntryPoint = "GetTextExtentPoint32W", ExactSpelling = true, CharSet = CharSet.Unicode)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool GetTextExtentPoint32(IntPtr dc, string text, int count, out FontSize size);
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
