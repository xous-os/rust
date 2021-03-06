use crate::spec::{LinkerFlavor, Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::windows_msvc_base::opts();
    base.cpu = "pentium4".to_string();
    base.max_atomic_width = Some(64);

    // Mark all dynamic libraries and executables as compatible with the larger 4GiB address
    // space available to x86 Windows binaries on x86_64.
    base.pre_link_args.get_mut(&LinkerFlavor::Msvc).unwrap().push("/LARGEADDRESSAWARE".to_string());

    // Ensure the linker will only produce an image if it can also produce a table of
    // the image's safe exception handlers.
    // https://docs.microsoft.com/en-us/cpp/build/reference/safeseh-image-has-safe-exception-handlers
    base.pre_link_args.get_mut(&LinkerFlavor::Msvc).unwrap().push("/SAFESEH".to_string());

    Ok(Target {
        llvm_target: "i686-pc-windows-msvc".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i64:64-f80:32-n8:16:32-a:0:32-S32"
            .to_string(),
        arch: "x86".to_string(),
        target_os: "windows".to_string(),
        target_env: "msvc".to_string(),
        target_vendor: "pc".to_string(),
        linker_flavor: LinkerFlavor::Msvc,
        options: base,
    })
}
