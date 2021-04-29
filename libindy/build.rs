use std::env;
use std::fs;
use regex::Regex;
use std::{
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
    process,
};
use walkdir::WalkDir;
use std::fs::{remove_dir_all, create_dir_all};

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    let sodium_static = env::var("CARGO_FEATURE_SODIUM_STATIC").ok();
    println!("sodium_static={:?}", sodium_static);

    if sodium_static.is_some() {
        println!("cargo:rustc-link-lib=static=sodium");
    }

    build_proto();

    if target.find("-windows-").is_some() {
        // do not build c-code on windows, use binaries
        let output_dir = env::var("OUT_DIR").unwrap();
        let prebuilt_dir = env::var("INDY_PREBUILT_DEPS_DIR").unwrap();

        let dst = Path::new(&output_dir[..]).join("..\\..\\..");
        let prebuilt_lib = Path::new(&prebuilt_dir[..]).join("lib");

        println!("cargo:rustc-link-search=native={}", prebuilt_dir);
        println!("cargo:rustc-flags=-L {}\\lib", prebuilt_dir);
        println!("cargo:include={}\\include", prebuilt_dir);

        let files = vec!["libeay32md.dll", "libsodium.dll", "libzmq.dll", "ssleay32md.dll"];
        for f in files.iter() {
            if let Ok(_) = fs::copy(&prebuilt_lib.join(f), &dst.join(f)) {
                println!("copy {} -> {}", &prebuilt_lib.join(f).display(), &dst.join(f).display());
            }
        }
    } else if target.find("linux-android").is_some() {
        //statically link files
        let openssl = match env::var("OPENSSL_LIB_DIR") {
            Ok(val) => val,
            Err(..) => match env::var("OPENSSL_DIR") {
                Ok(dir) => Path::new(&dir[..]).join("lib").to_string_lossy().into_owned(),
                Err(..) => panic!("Missing required environment variables OPENSSL_DIR or OPENSSL_LIB_DIR")
            }
        };

        let sodium = match env::var("SODIUM_LIB_DIR") {
            Ok(val) => val,
            Err(..) => panic!("Missing required environment variable SODIUM_LIB_DIR")
        };

        let zmq = match env::var("LIBZMQ_LIB_DIR") {
            Ok(val) => val,
            Err(..) => match env::var("LIBZMQ_PREFIX") {
                Ok(dir) => Path::new(&dir[..]).join("lib").to_string_lossy().into_owned(),
                Err(..) => panic!("Missing required environment variables LIBZMQ_PREFIX or LIBZMQ_LIB_DIR")
            }
        };

        println!("cargo:rustc-link-search=native={}", openssl);
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-search=native={}", sodium);
        println!("cargo:rustc-link-lib=static=sodium");
        println!("cargo:rustc-link-search=native={}", zmq);
        println!("cargo:rustc-link-lib=static=zmq");
    }
}


/// The Cosmos commit or tag to be cloned and used to build the proto files
const COSMOS_REV: &str = "v0.42.3";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated proto files go into in this repo
const COSMOS_SDK_PROTO_DIR: &str = "target/prost/";
/// Directory where the submodule is located
const COSMOS_SDK_DIR: &str = "cosmos-sdk-go";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];
/// Regex for locating instances of `tendermint-proto` in prost/tonic build output
const TENDERMINT_PROTO_REGEX: &str = "(super::)+tendermint";
/// Attribute preceeding a Tonic client definition
const TONIC_CLIENT_ATTRIBUTE: &str = "#[doc = r\" Generated client implementations.\"]";
/// Attributes to add to gRPC clients
const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    TONIC_CLIENT_ATTRIBUTE,
];


pub fn build_proto() {
    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = COSMOS_SDK_PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    fs::create_dir(tmp_build_dir.clone()).unwrap();

    update_submodule();
    output_sdk_version(&tmp_build_dir);
    compile_protos(&tmp_build_dir);
    compile_proto_services(&tmp_build_dir);
    copy_generated_files(&tmp_build_dir, &proto_dir)
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let exit_status = process::Command::new("git")
        .args(args)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        panic!("git exited with error code: {:?}", exit_status.code());
    }
}

fn update_submodule() {
    println!("[info] Updating cosmos/cosmos-sdk-go submodule...");
    run_git(&["submodule", "update", "--init"]);
    run_git(&["-C", COSMOS_SDK_DIR, "fetch"]);
    run_git(&["-C", COSMOS_SDK_DIR, "reset", "--hard", COSMOS_REV]);
}

fn output_sdk_version(out_dir: &Path) {
    let path = out_dir.join("COSMOS_SDK_COMMIT");
    fs::write(path, COSMOS_REV).unwrap();
}

fn compile_protos(out_dir: &Path) {
    let sdk_dir = Path::new(COSMOS_SDK_DIR);

    println!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");

    // Paths
    let proto_paths = [
        format!("{}/../proto/definitions/mock", root),
        format!("{}/proto/ibc", sdk_dir.display()),
        format!("{}/proto/cosmos/tx", sdk_dir.display()),
        format!("{}/proto/cosmos/bank", sdk_dir.display()),
        format!("{}/proto/cosmos/base", sdk_dir.display()),
        format!("{}/proto/cosmos/staking", sdk_dir.display()),
    ];

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in &proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);
    config.extern_path(".tendermint", "::tendermint_proto");

    if let Err(e) = config.compile_protos(&protos, &includes) {
        eprintln!("[error] couldn't compile protos: {}", e);
        panic!("protoc failed!");
    }
}

fn compile_proto_services(out_dir: impl AsRef<Path>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_dir = PathBuf::from(COSMOS_SDK_DIR);

    let proto_includes_paths = [
        root.join("../proto"),
        sdk_dir.join("proto"),
        sdk_dir.join("third_party/proto"),
    ];

    // List available paths for dependencies
    let includes = proto_includes_paths
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    let proto_services_path = [
        sdk_dir.join("proto/cosmos/auth/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmos/staking/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmos/bank/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmos/bank/v1beta1/tx.proto"),
        sdk_dir.join("proto/cosmos/tx/v1beta1/service.proto"),
        sdk_dir.join("proto/cosmos/tx/v1beta1/tx.proto"),
        sdk_dir.join("proto/cosmos/verimcosmos/nym.proto"),
        // sdk_dir.join("proto/cosmos/verimcosmos/tx.proto"),
    ];

    // List available paths for dependencies
    let services = proto_services_path
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    // Compile all proto client for GRPC services
    println!("[info ] Compiling proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .compile(&services, &includes)
        .unwrap();

    println!("[info ] => Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    println!(
        "[info ] Copying generated files into '{}'...",
        to_dir.display()
    );

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let contents = fs::read_to_string(src)?;

    // `prost-build` output references types from `tendermint-proto` crate
    // relative paths, which we need to munge into `tendermint_proto` in
    // order to leverage types from the upstream crate.
    let contents = Regex::new(TENDERMINT_PROTO_REGEX)
        .unwrap()
        .replace_all(&contents, "tendermint_proto");

    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}
