extern crate winres;
extern crate slint_build;

#[cfg(windows)]
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material".into());
    slint_build::compile_with_config("components/app.slint", config).unwrap();

    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator" />
                </requestedPrivileges>
            </security>
        </trustInfo>
        </assembly>
    "#);
    res.set_icon("logo.ico");
    res.compile().unwrap()
}