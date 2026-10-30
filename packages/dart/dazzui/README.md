# dazzui

The [DazzUI](https://github.com/dazzlabs/dazzui) design system for Flutter:
the same tokens, the same themes and the same components as
`@dazzlabs/dazzui`, drawn without Material.

The package imports nothing from `package:flutter/material.dart` and ships
`uses-material-design: false`. Every control is drawn by the kit; behaviour
Flutter already owns — focus, shortcuts, the overlay a menu opens into — is
Flutter's.

## Using it

```bash
flutter pub add dazzui
```

A host supplies the app plumbing with `WidgetsApp` and puts a `Theme` over
the subtree the kit draws:

```dart
import 'package:dazzui/dazzui.dart';

void main() => runApp(
  WidgetsApp(
    color: themeVariables.colorCanvas,
    pageRouteBuilder: <T>(settings, builder) => PageRouteBuilder<T>(
      settings: settings,
      pageBuilder: (context, _, _) => builder(context),
    ),
    home: Theme(
      data: ThemeData.studioDark(),
      child: Center(
        child: Button(
          variant: ButtonVariant.filled,
          onPressed: () {},
          child: const Text('Save'),
        ),
      ),
    ),
  ),
);
```

The kit's text field is a Cupertino one underneath and asks
`CupertinoLocalizations` for the word on its clear button; add
`DefaultCupertinoLocalizations.delegate` to `localizationsDelegates` when you
use it.

## Themes

`ThemeData` has a constructor per theme — `studioLight()`, `studioDark()`,
`frostLight()` and the rest. A component reads the theme in force with
`Theme.of(context)`. The names come from ds, the design system's single
token source; `lib/src/generated/` is what it wrote. **Never hand-edit it.**

## Native menus

The package requires Dart 3.13 or newer, matching its direct `nativeapi`
dependency. Wrap your window once to use native desktop dropdowns:

```dart
MenuConfiguration(
  presentation: MenuPresentation.native,
  child: Theme(data: ThemeData.studioLight(), child: MyWindow()),
)
```

Menu, Select, ContextMenu and Menubar inherit this preference. The default is
`MenuPresentation.custom`; a nested configuration can override it. Changing
presentation closes an open menu and applies the new mode on its next opening.
Triggers keep their DazzUI appearance. The native list uses the system's
appearance, spacing, navigation and dismissal behavior. Shortcut strings are
still display hints, not newly registered keyboard shortcuts.

Native menus are available on macOS, Windows and Linux when the nativeapi
backend and current host window are available. Web and mobile use the custom
panel, as does a desktop host where the backend cannot open a menu. Linux builds
need nativeapi's GTK 3, X11 and Xi development packages.

Combobox and Autocomplete retain their searchable custom suggestion lists in
both modes, so typing continues to filter them. Rich popovers such as the date
picker also stay custom; ordinary dropdowns inside them inherit the setting.
The example's theme panel offers a Native menus switch.

## The example

`example/` is the React example's window drawn with this package: every
component, live, in every theme.

```bash
cd example && flutter run
```

The macOS integration test opens real nativeapi menus, selects with AppKit
item actions, cancels and reopens, and switches menubar menus at the pointer.
Its debug-only Swift driver is compiled into the example; release builds omit
it. Run from `example/`:

```bash
flutter test integration_test/native_menu_test.dart -d macos
```
