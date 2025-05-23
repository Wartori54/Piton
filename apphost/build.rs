use embed_manifest::{embed_manifest, new_manifest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::{Command};

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        //Statically link vcruntime140.dll
        static_vcruntime::metabuild();

        //Embed a Windows manifest
        embed_manifest(new_manifest("Piton")).expect("unable to embed manifest file");
    }


    if std::env::var_os("CARGO_FEATURE_TESTAPP").is_some() {
        println!("cargo:rerun-if-changed=test");
        let out = Command::new("dotnet")
            .env("DOTNET_SYSTEM_GLOBALIZATION_INVARIANT", "true")
            .arg("build")
            .arg("test/Test.csproj")
            .output()
            .expect("Failed to run test C#");
        println!("cargo:warning={:?}", String::from_utf8(out.stdout).expect("Our bytes should be valid utf8"));
        println!("cargo:warning={:?}", String::from_utf8(out.stderr).expect("Our bytes should be valid utf8"));
    }
    
    println!("cargo:rerun-if-changed=build.rs")
}
