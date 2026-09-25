plugins {
	alias(libs.plugins.android.application)
	alias(libs.plugins.compose.compiler)
	alias(libs.plugins.kotlin.serialization)
	alias(libs.plugins.ktlint)
}

android {
	namespace = "dev.paperback.android"
	compileSdk = 37
	// Bundle builds deliver per-ABI themselves; APK builds split by ABI instead. Gradle refuses
	// to configure both at once, so each build picks exactly one of them below.
	val isBundleBuild = gradle.startParameter.taskNames.any { it.contains("bundle", ignoreCase = true) }
	defaultConfig {
		applicationId = "dev.paperback.android"
		minSdk = 24
		targetSdk = 36
		versionCode = 7
		versionName = "1.0"
		if (isBundleBuild) {
			ndk {
				// A bundle packages every ABI present in jniLibs and in dependency AARs. JNA and
				// AndroidX ship x86/x86_64 slices we have no libpaperback_core.so for, so without
				// this Play would build an x86 split that installs and then crashes on the
				// missing library. The splits block below does the same job for APK builds.
				abiFilters += listOf("armeabi-v7a", "arm64-v8a")
			}
		}
	}

	splits {
		abi {
			isEnable = !isBundleBuild
			reset()
			include("armeabi-v7a", "arm64-v8a")
			isUniversalApk = false
		}
	}

	val keystorePath = System.getenv("ANDROID_KEYSTORE_PATH")
	if (keystorePath != null) {
		signingConfigs {
			create("release") {
				storeFile = file(keystorePath)
				storePassword = System.getenv("ANDROID_STORE_PASSWORD")
				keyAlias = System.getenv("ANDROID_KEY_ALIAS")
				keyPassword = System.getenv("ANDROID_KEY_PASSWORD")
			}
		}
	}

	buildTypes {
		release {
			isMinifyEnabled = true
			isShrinkResources = true
			ndk {
				// Without this a native crash or ANR arrives as raw addresses. SYMBOL_TABLE
				// gives Play the function names to resolve them against; FULL would add line
				// numbers as well, at several times the upload size.
				debugSymbolLevel = "SYMBOL_TABLE"
			}
			proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
			signingConfig = if (keystorePath != null) {
				signingConfigs.getByName("release")
			} else {
				signingConfigs.getByName("debug")
			}
		}
	}
	compileOptions {
		sourceCompatibility = JavaVersion.VERSION_17
		targetCompatibility = JavaVersion.VERSION_17
	}
	buildFeatures {
		compose = true
		aidl = false
		buildConfig = false
		shaders = false
	}

	packaging {
		resources {
			excludes += "/META-INF/{AL2.0,LGPL2.1}"
		}
	}
}

kotlin {
	jvmToolchain(17)
}

ktlint {
	filter {
		exclude("**/uniffi/**")
	}
}

dependencies {
	val composeBom = platform(libs.androidx.compose.bom)
	implementation(composeBom)
	androidTestImplementation(composeBom)
	// Core Android dependencies
	implementation(libs.androidx.core.ktx)
	implementation(libs.androidx.lifecycle.runtime.ktx)
	implementation(libs.androidx.activity.compose)
	// Arch Components
	implementation(libs.androidx.lifecycle.runtime.compose)
	implementation(libs.androidx.lifecycle.viewmodel.compose)
	implementation(libs.androidx.media3.session)
	// Compose
	implementation(libs.androidx.compose.ui)
	implementation(libs.androidx.compose.ui.tooling.preview)
	implementation(libs.androidx.compose.material3)
	implementation(libs.androidx.compose.material.icons.core)
	implementation(libs.androidx.compose.material.icons.extended)
	// Tooling
	debugImplementation(libs.androidx.compose.ui.tooling)
	// Instrumented tests
	androidTestImplementation(libs.androidx.compose.ui.test.junit4)
	debugImplementation(libs.androidx.compose.ui.test.manifest)
	// Local tests: jUnit, coroutines, Android runner
	testImplementation(libs.junit)
	testImplementation(libs.kotlinx.coroutines.test)
	// Instrumented tests: jUnit rules and runners
	androidTestImplementation(libs.androidx.test.core)
	androidTestImplementation(libs.androidx.test.ext.junit)
	androidTestImplementation(libs.androidx.test.runner)
	androidTestImplementation(libs.androidx.test.espresso.core)
	// Navigation
	implementation(libs.androidx.navigation3.ui)
	implementation(libs.androidx.navigation3.runtime)
	implementation(libs.androidx.lifecycle.viewmodel.navigation3)
	// JNA for UniFFI
	implementation("net.java.dev.jna:jna:5.19.1@aar")
}
