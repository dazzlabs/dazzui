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

## The example

`example/` is the React example's window drawn with this package: every
component, live, in every theme.

```bash
cd example && flutter run
```
