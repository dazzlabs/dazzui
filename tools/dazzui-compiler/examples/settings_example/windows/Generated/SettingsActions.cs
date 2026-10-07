// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/model.rs

namespace SettingsExample;

/// <summary>The methods of <c>SettingsModel</c> a page calls. The Rust model's store
/// hands them on; a host, or a test, may implement them itself.</summary>
public interface ISettingsActions
{
    void SetLaunchAtStartup(bool on);
    void SetAutoUpdate(bool on);
    void SetShowDisabledPlugins(bool on);
    void TogglePlugin(uint id);
    void Save();
    void Reset();
}
