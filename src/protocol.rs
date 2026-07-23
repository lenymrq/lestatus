pub fn format_blocks_text(blocks_text: &Vec<String>) -> String {
    let mut out = String::from("[");
    for (i, block_text) in blocks_text.iter().enumerate() {
        if i > 0 {
            out.push_str(",");
        }
        out.push_str(&format!("{{\"full_text\":\"{}\"}}", block_text))
    }
    out.push_str("],");
    return out;
}

// TODO: use serde_json with a list of serializable structs or something like that i guess
