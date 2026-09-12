package dev.dazzlabs.dazzui.mobile

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.luminance
import dev.dazzlabs.dazzui.mobile.generated.DazzThemeFamily
import dev.dazzlabs.dazzui.mobile.generated.dazzColorScheme
import org.junit.Assert.*
import org.junit.Test

class ColorSchemeTest {
    @Test
    fun everyFamilyHasDistinctLightAndDarkSchemes() {
        for (family in DazzThemeFamily.entries) {
            val light = dazzColorScheme(family, false)
            val dark = dazzColorScheme(family, true)
            assertTrue("$family paper", light.surface.luminance() > dark.surface.luminance())
            assertTrue("$family light ink", contrast(light.onSurface, light.surface) >= 4.5f)
            assertTrue("$family dark ink", contrast(dark.onSurface, dark.surface) >= 4.5f)
            assertEquals(light.primaryFixed, dark.primaryFixed)
            for (scheme in listOf(light, dark)) {
                assertEquals(1f, scheme.primaryContainer.alpha)
                assertTrue("$family primary", contrast(scheme.onPrimary, scheme.primary) >= 3f)
                assertTrue("$family error", contrast(scheme.onError, scheme.error) >= 4.5f)
                assertTrue("$family tertiary", contrast(scheme.onTertiary, scheme.tertiary) >= 4.5f)
            }
        }
    }

    @Test
    fun brightUsesItsAcidAndNavyPairInsteadOfWhiteOnEveryAccent() {
        val light = dazzColorScheme(DazzThemeFamily.Bright, false)
        val dark = dazzColorScheme(DazzThemeFamily.Bright, true)
        assertEquals(Color(0xFFD6FF3F), light.onPrimary)
        assertEquals(Color(0xFF111C2E), dark.onPrimary)
    }

    private fun contrast(a: Color, b: Color): Float {
        val first = a.luminance()
        val second = b.luminance()
        return (maxOf(first, second) + 0.05f) / (minOf(first, second) + 0.05f)
    }
}
