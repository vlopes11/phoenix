fn main() {
    tonic_build::compile_protos("proto/phoenix/phoenix.proto").unwrap();
    tonic_build::compile_protos("proto/phoenix/rusk.proto").unwrap();
}
