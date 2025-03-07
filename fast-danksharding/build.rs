use std::env;

fn main() {
    //TODO: check cargo features selected
    //TODO: can conflict/duplicate with make ?

    println!("cargo:rerun-if-env-changed=CXXFLAGS");
    println!("cargo:rerun-if-changed=../icicle");

    let arch_type = env::var("ARCH_TYPE")
        .unwrap_or(String::from("native"));

    let mut arch = String::from("-arch=");
    arch.push_str(&arch_type);

    let mut nvcc = cc::Build::new();

    println!("Compiling icicle library using arch: {}", &arch);

    nvcc.cuda(true);
    nvcc.debug(false);
    nvcc.flag(&arch);
    nvcc.flag("--std=c++17");
    nvcc.files([
        "./src/cuda/lib.cu",
    ]);
    nvcc.compile("ingo_dankshard"); //TODO: extension??
}
