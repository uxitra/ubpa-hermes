pub fn detect_pdf(data: &[u8]) -> bool {
    println!("detecting if pdf");
    if let Some(kind) = infer::get(data) {
        println!("{}", kind);
        kind.mime_type() == "application/pdf"
    } else {
        false
    }
}
