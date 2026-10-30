use std::path::Path;
use std::process::Command;

#[test]
fn cli() {
    const TOOLS_FILE: &str = "dotnet-tools.json";

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases_dir = manifest_dir.join("tests/cmd");
    for entry in
        std::fs::read_dir(&cases_dir).expect("reading the test cases directory should not fail")
    {
        let path = entry
            .expect("reading a test cases directory entry should not fail")
            .path();
        if path.is_dir() && path.extension().is_some_and(|ext| ext == "in") {
            std::fs::copy(manifest_dir.join(TOOLS_FILE), path.join(TOOLS_FILE))
                .expect("copying the tools manifest should not fail");
        }
    }

    // Restore the tools once, as concurrent restorations by the test cases fail when
    // the tools are not already in the NuGet cache.
    let status = Command::new("dotnet")
        .args(["tool", "restore"])
        .current_dir(manifest_dir)
        .status()
        .expect("running `dotnet tool restore` should not fail");
    assert!(status.success(), "`dotnet tool restore` failed: {status}");

    trycmd::TestCases::new()
        .case("tests/cmd/*.toml")
        .case("tests/cmd/*.trycmd");
}
