use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();

    if !target.contains("msvc") {
        build.flag("-std=c++17");
    }
    
    build
    .include("vendor/Rush")
    .file("vendor/Rush/GfxBitmapFont.cpp")
    .file("vendor/Rush/GfxCommon.cpp")
    .file("vendor/Rush/GfxEmbeddedShaders.cpp")
    .file("vendor/Rush/GfxEmbeddedShadersMSL.cpp")
    .file("vendor/Rush/GfxPrimitiveBatch.cpp")
    .file("vendor/Rush/MathTypes.cpp")
    //.file("vendor/Rush/Platform.cpp")
    .file("vendor/Rush/UtilCamera.cpp")
    .file("vendor/Rush/UtilFile.cpp")
    .file("vendor/Rush/UtilLog.cpp")
    .file("vendor/Rush/UtilTimer.cpp")
    .file("vendor/Rush/Window.cpp")
    .file("vendor/Rush/RushC.cpp")
    .cpp(true)
    .warnings(false);

    if target.contains("darwin") {
        build
        .define("RUSH_RENDER_API", "RUSH_RENDER_API_MTL")
        .file("vendor/Rush/GfxDeviceMtl.mm")
        .file("vendor/Rush/PlatformMac.mm")
        .file("vendor/Rush/WindowMac.mm")
        .flag("-ObjC++");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
    } else {
        build
        .include("vendor/External/Vulkan-Headers")
        .include("vendor/External/Volk")
        .file("vendor/External/Volk/volk.c")
        .file("vendor/Rush/GfxDeviceVK.cpp");

        if target.contains("windows") {
            build
            .define("RUSH_RENDER_API", "RUSH_RENDER_API_VK")
            .define("RUSH_PLATFORM_WINDOWS", "1")
            .define("NOMINMAX", "1")
            .define("WIN32_LEAN_AND_MEAN", "1")
            .define("VK_USE_PLATFORM_WIN32_KHR", "1")
            .file("vendor/Rush/PlatformWin32.cpp")
            .file("vendor/Rush/WindowWin32.cpp");
            println!("cargo:rustc-link-lib=user32");
        }
        else if target.contains("linux") {
            build
            .define("RUSH_RENDER_API", "RUSH_RENDER_API_VK")
            .define("RUSH_PLATFORM_LINUX", "1")
            .define("VK_USE_PLATFORM_XCB_KHR", "1")
            .file("vendor/Rush/PlatformLinux.cpp")
            .file("vendor/Rush/WindowXCB.cpp");
            println!("cargo:rustc-link-lib=xcb");
            println!("cargo:rustc-link-lib=xcb-keysyms");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=dl");
        }
    }

    build.compile("rush")
}
