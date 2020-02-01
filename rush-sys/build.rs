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
        .file("vendor/Rush/GfxDeviceMtl.mm")
        .file("vendor/Rush/PlatformMac.mm")
        .file("vendor/Rush/GfxEmbeddedShadersMSL.cpp")
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
        .file("vendor/Rush/GfxEmbeddedShaders.cpp")
        .file("vendor/Rush/GfxDeviceVK.cpp")
        ;
    }

    build.compile("rush")
}
