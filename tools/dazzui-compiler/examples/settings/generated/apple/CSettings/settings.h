#ifndef SETTINGS_BRIDGE_H
#define SETTINGS_BRIDGE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifndef NUI_STR_DEFINED
#define NUI_STR_DEFINED
/// UTF-8 bytes and their length. Rust owns those in a state snapshot;
/// the caller owns those in an action, for the length of the call.
typedef struct {
  const uint8_t *ptr;
  size_t len;
} NuiStr;
#endif

typedef void (*NuiStateChanged)(void *context);

typedef struct SettingsApp SettingsApp;

/// A `Plugin`.
typedef struct {
  uint32_t id;
  NuiStr name;
  bool enabled;
} SettingsPluginDto;

typedef struct {
  const SettingsPluginDto *ptr;
  size_t len;
} SettingsPluginList;

/// A snapshot of `SettingsState`. Hand it back to `settings_state_free`.
typedef struct {
  bool launch_at_startup;
  bool auto_update;
  bool saved;
  uint32_t saves;
  NuiStr status;
  bool show_disabled_plugins;
  SettingsPluginList plugins;
  SettingsPluginList visible_plugins;
  bool has_hidden_plugins;
} SettingsStateDto;

#define SETTINGS_ACTION_SET_LAUNCH_AT_STARTUP 0u
#define SETTINGS_ACTION_SET_AUTO_UPDATE 1u
#define SETTINGS_ACTION_SAVE 2u
#define SETTINGS_ACTION_RESET 3u
#define SETTINGS_ACTION_SET_SHOW_DISABLED_PLUGINS 4u
#define SETTINGS_ACTION_TOGGLE_PLUGIN 5u

typedef union {
  bool set_launch_at_startup;
  bool set_auto_update;
  bool set_show_disabled_plugins;
  uint32_t toggle_plugin;
} SettingsActionPayload;

/// A `SettingsAction`: a tag, and the value its variant carries.
typedef struct {
  uint32_t tag;
  SettingsActionPayload payload;
} SettingsActionDto;

SettingsApp *settings_create(void);
void settings_destroy(SettingsApp *app);
SettingsStateDto settings_get_state(const SettingsApp *app);
void settings_state_free(SettingsStateDto state);
void settings_send(SettingsApp *app, SettingsActionDto action);
/// `callback` runs after every action, on the thread that sent it.
void settings_subscribe(SettingsApp *app, NuiStateChanged callback, void *context);

#endif
