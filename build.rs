extern crate protobuf_codegen;

fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("src/")
        .inputs(&["proto/OpenControllerLib.proto"])
        .include("proto")
        .run()
        .expect("protoc");
}