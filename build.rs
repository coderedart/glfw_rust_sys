fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let features = Features::default();
    #[allow(unused)]
    let out_dir = std::env::var("OUT_DIR").expect("failed to get out dir");
    if TargetOs::Others == features.os && !(features.x11 || features.wayland || features.osmesa) {
        println!(
        "cargo:warning=unsupported os/platform. you may want to choose x11 or wayland features for linux-like targets"
    )
    }
    // gen bindings at build time, instead of using pre-generated bindings
    #[cfg(feature = "bindings")]
    generate_bindings(features, &out_dir);
    // build from src, instead of using prebuilt-libraries
    #[cfg(feature = "src_build")]
    build_from_src(features, &out_dir);
    #[cfg(not(feature = "src_build"))]
    download_libs(features, &out_dir);
    // emit the linker flags
    if features.static_link {
        println!("cargo:rustc-link-lib=static=glfw3");
    } else {
        match features.os {
            TargetOs::Win => println!("cargo:rustc-link-lib=dylib=glfw3dll"),
            _ => println!("cargo:rustc-link-lib=dylib=glfw"),
        }
    }
    match features.os {
        TargetOs::Win => {
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
            println!("cargo:rustc-link-lib=dylib=shell32");
        }
        TargetOs::Mac => {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
        }
        TargetOs::Linux => {
            // Gl?
        }
        _ => {}
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetOs {
    Win,
    Mac,
    Linux,
    Others,
}
#[allow(unused)]
#[derive(Clone, Copy)]
struct Features {
    static_link: bool,
    vulkan: bool,
    native: bool,
    os: TargetOs,
    wayland: bool,
    x11: bool,
    egl: bool,
    osmesa: bool,
    bindings: bool,
    gl: bool,
}
impl Default for Features {
    fn default() -> Self {
        Self {
            static_link: cfg!(feature = "static_link"),
            vulkan: cfg!(feature = "vulkan"),
            native: cfg!(feature = "native_handles"),
            os: match std::env::var("CARGO_CFG_TARGET_OS")
                .expect("failed to get target os")
                .as_str()
            {
                "windows" => TargetOs::Win,
                "macos" => TargetOs::Mac,
                "linux" => TargetOs::Linux,
                _ => TargetOs::Others,
            },
            wayland: cfg!(feature = "wayland"),
            x11: cfg!(feature = "x11"),
            egl: cfg!(feature = "native_egl"),
            osmesa: cfg!(feature = "osmesa"),
            bindings: cfg!(feature = "bindings"),
            gl: cfg!(feature = "native_gl"),
        }
    }
}
#[cfg(feature = "src_build")]
fn build_from_src(features: Features, _out_dir: &str) {
    let mut config = cmake::Config::new("./glfw");
    config
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF");
    if features.os == TargetOs::Linux || features.os == TargetOs::Others {
        if features.wayland {
            config.define("GLFW_BUILD_WAYLAND", "ON");
        } else {
            config.define("GLFW_BUILD_WAYLAND", "OFF");
        }
        if features.x11 {
            config.define("GLFW_BUILD_X11", "ON");
        } else {
            config.define("GLFW_BUILD_X11", "OFF");
        }
    }
    if features.static_link {
        config.define("GLFW_LIBRARY_TYPE", "STATIC");
    } else {
        config.define("GLFW_LIBRARY_TYPE", "SHARED");
    }
    let dst_dir = config.build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst_dir.join("lib").display()
    );
    if !features.static_link && features.os == TargetOs::Win {
        println!(
            "cargo:rustc-link-search=native={}",
            dst_dir.join("bin").display()
        );
    }
}
#[cfg(feature = "bindings")]
fn generate_bindings(features: Features, out_dir: &str) {
    assert!(features.bindings);
    let glfw_header = include_str!("./glfw/include/GLFW/glfw3.h");
    let mut bindings = bindgen::Builder::default();
    let vulkan_include = if features.vulkan {
        match features.os {
            TargetOs::Win | TargetOs::Mac => {
                let vulkan_sdk_dir =
                    std::env::var("VULKAN_SDK").expect("failed to get vulkan sdk dir");
                if !std::path::Path::new(&vulkan_sdk_dir).exists() {
                    println!("cargo:warning=missing vulkan sdk dir {vulkan_sdk_dir} for vulkan.h");
                }
                bindings = bindings.clang_arg(format!("-I{vulkan_sdk_dir}/include"));
            }
            _ => {}
        };
        "#define GLFW_INCLUDE_VULKAN\n"
    } else {
        ""
    };
    let mut native_include = "".to_string();
    let glfw_native_header = features
        .native
        .then_some(include_str!("./glfw/include/GLFW/glfw3native.h"))
        .unwrap_or("");

    if features.native {
        match features.os {
            TargetOs::Win => {
                native_include.push_str("#define GLFW_EXPOSE_NATIVE_WIN32\n");
                if features.gl {
                    native_include.push_str("#define GLFW_EXPOSE_NATIVE_WGL\n");
                }
            }
            TargetOs::Mac => {
                native_include.push_str("#define GLFW_EXPOSE_NATIVE_COCOA\n");
                if features.gl {
                    native_include.push_str("#define GLFW_EXPOSE_NATIVE_NSGL\n");
                }
            }
            _ => {
                if features.wayland {
                    native_include.push_str("#define GLFW_EXPOSE_NATIVE_WAYLAND\n");
                }
                // egl can be enabled explicitly for x11. or just implicitly via gl + wayland
                if features.egl || (features.gl && features.wayland) {
                    native_include.push_str("\n#define GLFW_EXPOSE_NATIVE_EGL\n");
                }

                if features.x11 {
                    native_include.push_str("#define GLFW_EXPOSE_NATIVE_X11\n");
                    if features.gl {
                        native_include.push_str("#define GLFW_EXPOSE_NATIVE_GLX\n");
                    }
                }
                if features.osmesa {
                    native_include.push_str("\n#define GLFW_EXPOSE_NATIVE_OS_MESA\n");
                }
            }
        };
    }
    // if we don't define this, on some platforms (like mac),
    // glfw will include gl.h by default, which is not something we want
    let gl_include = "#define GLFW_INCLUDE_NONE";
    bindings = bindings.header_contents(
        "glfw3.h",
        &format!(
            "{vulkan_include}\n{gl_include}\n{glfw_header}\n{native_include}\n{glfw_native_header}"
        ),
    );
    const DUPLICATE_ITEMS: &[&str] = &[
        "FP_NAN",
        "FP_INFINITE",
        "FP_ZERO",
        "FP_SUBNORMAL",
        "FP_NORMAL",
    ];
    for item in DUPLICATE_ITEMS {
        bindings = bindings.blocklist_item(item);
    }

    bindings
        .merge_extern_blocks(true)
        .allowlist_file(".*glfw3\\.h")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file(format!("{out_dir}/bindings.rs"))
        .expect("failed to write bindings to out_dir/bindings.rs");
}

#[cfg(not(feature = "src_build"))]
fn download_libs(features: Features, out_dir: &str) {
    const URL: &str = "https://github.com/glfw/glfw/releases/download/3.4";
    let zip_name: &str = match features.os {
        TargetOs::Win => {
            let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            if arch == "x86" {
                "glfw-3.4.bin.WIN32"
            } else {
                assert_eq!(arch, "x86_64");
                "glfw-3.4.bin.WIN64"
            }
        }
        TargetOs::Mac => "glfw-3.4.bin.MACOS",
        _ => {
            return;
        }
    };
    let url = format!("{}/{}.zip", URL, zip_name);
    let curl_status = std::process::Command::new("curl")
        .current_dir(out_dir)
        .args(["--progress-bar", "--fail", "-L", &url, "-o", "glfw.zip"])
        .status();

    assert!(
        curl_status.expect("failed to run curl command").success(),
        "curl failed to download {url} and store it in {out_dir:?}"
    );
    println!("downloaded impeller library from {url} and stored it in {out_dir:?}");
    let mut command = if cfg!(unix) {
        std::process::Command::new("unzip")
    } else {
        let mut command = std::process::Command::new("tar");
        command.arg("-xvf");
        command
    };
    let tar_status = command.arg("glfw.zip").current_dir(&out_dir).status();
    assert!(
        tar_status
            .expect("failed to run tar/unzip command")
            .success(),
        "tar failed to extract zip and store it in {out_dir:?}"
    );
    println!("extracted glfw library from zip and stored it in {out_dir:?}");
    let lib_dir = std::path::Path::new(out_dir).join(zip_name);
    match features.os {
        TargetOs::Win => {
            println!(
                "cargo:rustc-link-search=native={}",
                lib_dir.join("lib-vc2022").display()
            );
        }
        TargetOs::Mac => {
            let lib_dir = lib_dir.join("lib-universal");
            // hack because mac fails to recognize libglfw.3.dylib with -lglfw flag
            std::fs::copy(
                lib_dir.join("libglfw.3.dylib"),
                lib_dir.join("libglfw.dylib"),
            )
            .expect("failed to copy libglfw.3.dylib to libglfw.dylib");
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }
        _ => {
            unimplemented!()
        }
    }
}
