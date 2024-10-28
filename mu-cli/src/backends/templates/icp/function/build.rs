use uuid::Uuid;

fn main() {
    let build_uuid = Uuid::new_v4().to_string();
    println!("cargo::rustc-env=MU_BUILD_UUID={}", build_uuid);
}
