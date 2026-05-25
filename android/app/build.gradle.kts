plugins {
	alias(libs.plugins.android.application)
	alias(libs.plugins.compose.compiler)
	alias(libs.plugins.kotlin.serialization)
	alias(libs.plugins.ktlint)
}

android {
	namespace = "dev.paperback.mobile"
	compileSdk = 36
	defaultConfig {
		applicationId = "dev.paperback.mobile"
		minSdk = 24
		targetSdk = 36
		versionCode = 1
		versionName = "1.0"
	}

	splits {
		abi {
			isEnable = true
			reset()
			include("armeabi-v7a", "arm64-v8a")
			isUniversalApk = false
		}
	}

	buildTypes {
		release {
			isMinifyEnabled = true
			isShrinkResources = true
			proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
			signingConfig = signingConfigs.getByName("debug")
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
	implementation(libs.androidx.media)
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
	implementation("net.java.dev.jna:jna:5.13.0@aar")
}
