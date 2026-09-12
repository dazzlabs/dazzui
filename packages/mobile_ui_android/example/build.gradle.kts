plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "dev.dazzlabs.dazzui.mobile.example"
    compileSdk = 36
    defaultConfig {
        applicationId = "dev.dazzlabs.dazzui.mobile.example"
        minSdk = 23
        targetSdk = 36
        versionCode = 1
        versionName = "0.0.0"
    }
    buildFeatures { compose = true }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

kotlin { jvmToolchain(17) }

dependencies {
    implementation(project(":"))
    implementation("androidx.activity:activity-compose:1.11.0")
    implementation("androidx.compose.foundation:foundation")
}
