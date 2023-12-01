fn main() {
    cc::Build::new()
        .file("c_src/dext.cpp")
        .include("c_src/Karabiner-DriverKit-VirtualHIDDevice/include/pqrs/karabiner/driverkit")
        .include("c_src/Karabiner-DriverKit-VirtualHIDDevice/src/Client/vendor/include")
        .cpp(true)
        .std("c++2a")
        .flag("-w")
        .shared_flag(true)
        .flag("-fPIC")
        .compile("dext");

    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    //cc c_src/list-keyboards.c -o list-keyboards -framework IOKit -framework CoreFoundation
}
