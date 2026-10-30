use std::path::Path;

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
    duct::cmd!("dotnet", "tool", "restore")
        .dir(manifest_dir)
        .run()
        .expect("restoring the dotnet tools should not fail");

    trycmd::TestCases::new()
        .case("tests/cmd/*.toml")
        .case("tests/cmd/*.trycmd");
}
