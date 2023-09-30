fn main() {
    // Run prisma generate if schema changed
    // println!("cargo:rerun-if-changed=prisma/schema.prisma"); // The double deploy hack struggled with this, so left it out
    if !std::process::Command::new("cargo")
        .args(["run", "prisma", "generate"])
        .status()
        .expect("failed to build prisma")
        .success()
    {
        panic!("failed to generate prisma code")
    }
}
