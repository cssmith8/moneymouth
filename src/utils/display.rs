pub async fn label_display(index: u32, length: u32, string: &String) -> String {
    return format!("-# {}/{}\n{}", index + 1, length, string);
}
