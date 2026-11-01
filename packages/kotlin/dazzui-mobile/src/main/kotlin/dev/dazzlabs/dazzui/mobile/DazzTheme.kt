package dev.dazzlabs.dazzui.mobile

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import dev.dazzlabs.dazzui.mobile.generated.DazzThemeFamily
import dev.dazzlabs.dazzui.mobile.generated.dazzColorScheme

/**
 * Applies ds colors to unmodified AndroidX Material 3 components.
 * Typography, shapes, motion, touch targets and behavior remain Material's.
 * Wallpaper-derived dynamic color is intentionally not applied: ds owns color.
 */
@Composable
fun DazzTheme(
    family: DazzThemeFamily = DazzThemeFamily.Bright,
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit,
) {
    MaterialTheme(
        colorScheme = dazzColorScheme(family, darkTheme),
        content = content,
    )
}
