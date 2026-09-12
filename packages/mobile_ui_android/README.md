# DazzUI Mobile — Android

Android Jetpack Compose with the ds color schemes and **unmodified AndroidX
Material 3 components**. This package supplies `DazzTheme`, not a parallel
component library. Import `Button`, `TextField`, `Switch`, `AlertDialog`, and
other controls directly from `androidx.compose.material3`.

Material owns typography, shapes, dimensions, touch targets, ripple, animation,
accessibility, and interaction. These are the official Compose components,
not wrappers around Android View widgets. Desktop component geometry and
React visual parity do not apply to this package.

The package is named for the Android platform. A future sibling,
`mobile_ui_android_react`, will support React-based Android UI prototyping;
this package remains the native Android implementation.

## Use

Include this Android library in your Gradle project and depend on it:

```kotlin
// settings.gradle.kts (adjust the path for your checkout)
include(":mobile_ui_android")
project(":mobile_ui_android").projectDir = file("packages/mobile_ui_android")

// app/build.gradle.kts
dependencies {
    implementation(project(":mobile_ui_android"))
}
```

The host needs the Android and Kotlin Compose plugins. Align their versions
with this package when including it as a source module.

```kotlin
import androidx.compose.material3.Button
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import dev.dazzlabs.dazzui.mobile.DazzTheme
import dev.dazzlabs.dazzui.mobile.generated.DazzThemeFamily

@Composable
fun App() {
    DazzTheme(family = DazzThemeFamily.Bright) {
        Surface {
            Button(onClick = { /* action */ }) { Text("Continue") }
        }
    }
}
```

Studio is the default family. Studio, Bright, Frost, Graphite, Ember, and
Nocturne each have light and dark schemes. Appearance follows the system
unless `darkTheme` is supplied. Wallpaper dynamic colors are not enabled,
so the selected ds palette remains authoritative.

`dazzColorScheme(family, darkTheme)` also exposes the generated scheme
for a host that already owns its `MaterialTheme`.

## Color generation

Run `pnpm tokens` at the repository root. `packages/ds/plugins/compose.mjs`
maps the resolved token graph to Material color roles and distributes
`src/main/kotlin/dev/dazzlabs/dazzui/mobile/generated/DazzColorSchemes.kt`.
The generated file is committed; the Android build does not need Node or ds.
Do not edit it by hand.

- Primary uses the filled primary recipe and `color.on-accent`.
- Secondary uses the neutral semantic ink and surfaces; tertiary uses info.
- Error uses danger. Info and danger fills use whichever of the theme's paper
  and ink provides greater contrast, since `on-accent` belongs to the brand.
  If both miss 4.5:1, they fall back to ds white or black for readable text.
- Containers composite the tinted recipe over paper to make an opaque fill.
- Surfaces, ink, and outlines use ds semantic roles. Fixed roles use the light
  counterpart in both appearances. Material applies its own state opacities.

No token typography or shapes are generated for Compose.

## Build and playground

Requires JDK 17 and Android SDK 36. Set `ANDROID_HOME`, or add `sdk.dir` to
the ignored `local.properties`. Open this directory in Android Studio, or:

```sh
./gradlew assembleDebug testDebugUnitTest :example:assembleDebug
./gradlew :example:installDebug
```

The example is an Android app with a family menu, system/manual appearance,
and interactive stock Material controls. Launch **DazzUI Mobile** after
installing on a device or emulator. Gradle is independent of pnpm.
