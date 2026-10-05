// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace SettingsExample;

/// <summary>The C ABI of <c>settings_example</c>, from the Rust app core.</summary>
internal static unsafe partial class SettingsNative
{
    private const string Library = "settings_example";

    [StructLayout(LayoutKind.Sequential)]
    internal struct NuiStr { public byte* Ptr; public nuint Len; }

    [StructLayout(LayoutKind.Sequential)]
    internal struct PluginDto
    {
        public uint Id;
        public NuiStr Name;
        public byte Enabled;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct PluginList { public PluginDto* Ptr; public nuint Len; }

    [StructLayout(LayoutKind.Sequential)]
    internal struct StateDto
    {
        public byte LaunchAtStartup;
        public byte AutoUpdate;
        public byte Saved;
        public uint Saves;
        public NuiStr Status;
        public byte ShowDisabledPlugins;
        public PluginList Plugins;
        public PluginList VisiblePlugins;
        public byte HasHiddenPlugins;
    }

    [StructLayout(LayoutKind.Explicit)]
    internal struct ActionPayload
    {
        [FieldOffset(0)] public byte SetLaunchAtStartup;
        [FieldOffset(0)] public byte SetAutoUpdate;
        [FieldOffset(0)] public byte SetShowDisabledPlugins;
        [FieldOffset(0)] public uint TogglePlugin;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct ActionDto
    {
        public uint Tag;
        public ActionPayload Payload;
    }

    [LibraryImport(Library)] internal static partial IntPtr settings_create();
    [LibraryImport(Library)] internal static partial void settings_destroy(IntPtr app);
    [LibraryImport(Library)] internal static partial StateDto settings_get_state(IntPtr app);
    [LibraryImport(Library)] internal static partial void settings_state_free(StateDto state);
    [LibraryImport(Library)] internal static partial void settings_send(IntPtr app, ActionDto action);
    [LibraryImport(Library)] internal static partial void settings_subscribe(IntPtr app, delegate* unmanaged[Cdecl]<IntPtr, void> callback, IntPtr context);

    internal static SettingsState ReadState(IntPtr app)
    {
        var dto = settings_get_state(app);
        try { return Read(dto); }
        finally { settings_state_free(dto); }
    }

    private static Plugin Read(PluginDto dto) => new(dto.Id, Decode(dto.Name), dto.Enabled != 0);

    private static IReadOnlyList<Plugin> ReadList(PluginList list)
    {
        var items = new List<Plugin>(checked((int)list.Len));
        for (nuint i = 0; i < list.Len; i++) items.Add(Read(list.Ptr[i]));
        return items;
    }

    private static SettingsState Read(StateDto dto) => new(dto.LaunchAtStartup != 0, dto.AutoUpdate != 0, dto.Saved != 0, dto.Saves, Decode(dto.Status), dto.ShowDisabledPlugins != 0, ReadList(dto.Plugins), ReadList(dto.VisiblePlugins), dto.HasHiddenPlugins != 0);

    internal static void Send(IntPtr app, SettingsAction action)
    {
        switch (action)
        {
            case SettingsAction.SetLaunchAtStartup a:
            {
                var dto = new ActionDto { Tag = 0 };
                dto.Payload.SetLaunchAtStartup = a.Value ? (byte)1 : (byte)0;
                settings_send(app, dto);
                break;
            }
            case SettingsAction.SetAutoUpdate a:
            {
                var dto = new ActionDto { Tag = 1 };
                dto.Payload.SetAutoUpdate = a.Value ? (byte)1 : (byte)0;
                settings_send(app, dto);
                break;
            }
            case SettingsAction.Save:
                settings_send(app, new ActionDto { Tag = 2 });
                break;
            case SettingsAction.Reset:
                settings_send(app, new ActionDto { Tag = 3 });
                break;
            case SettingsAction.SetShowDisabledPlugins a:
            {
                var dto = new ActionDto { Tag = 4 };
                dto.Payload.SetShowDisabledPlugins = a.Value ? (byte)1 : (byte)0;
                settings_send(app, dto);
                break;
            }
            case SettingsAction.TogglePlugin a:
            {
                var dto = new ActionDto { Tag = 5 };
                dto.Payload.TogglePlugin = a.Value;
                settings_send(app, dto);
                break;
            }
        }
    }

    private static string Decode(NuiStr text) =>
        text.Len == 0 ? "" : Encoding.UTF8.GetString(text.Ptr, checked((int)text.Len));
}
