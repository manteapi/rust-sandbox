use anyhow::Result;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches(
        "Oh dear pattern.\n I am looking for you.",
        "pattern",
        &mut result,
    )
    .unwrap();
    assert_eq!(result, b"Oh dear pattern.\n");
}
