pub fn md5sum(content: &String) -> String {
    let mut context = md5::Context::new();
    context.consume(content);
    let digest = context.finalize();
    format!("{:x}", digest)
}
