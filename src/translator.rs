use crate::Markdown;

pub fn translate(md: Vec<Markdown>) -> String {
    md.iter()
        .map(|bit| match bit {
            Markdown::Heading(line) => translate_header(line.to_vec()),
            Markdown::Paragraph(line) => translate_line(line.to_vec()),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn translate_header(text: Vec<String>) -> String {
    format!("{{ \"innerText\": \"{}\",\n  \"type\": \"heading\"\n}},\n", translate_text(text))
}

fn translate_line(text: Vec<String>) -> String {
    let line = translate_text(text);
    if line.len() > 0 {
        format!("{{ \"innerText\": \"{}\",\n  \"type\": \"paragraph\"\n}},\n", line)
    } else {
        format!("{}", line)
    }
}

fn translate_text(text: Vec<String>) -> String {
    text.iter()
        .map(|part| match part {
            text => text.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
