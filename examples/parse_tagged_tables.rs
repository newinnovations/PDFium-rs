use pdfium::*;

fn print_tree(
    element: &PdfiumStructElement,
    text_page: &PdfiumTextPage,
    objects: &[PdfiumPageObject],
    indent: usize,
) {
    let tag_type = element
        .element_type()
        .unwrap_or_else(|| "Unknown".to_string());
    let alt_text = element.alt_text().unwrap_or_default();

    // Find associated text
    let mut text_content = String::new();
    let _mcid = element.marked_content_id();

    let mcid_count = element.marked_content_id_count().unwrap_or(0);
    let mut all_mcids = Vec::new();
    for i in 0..mcid_count {
        if let Some(id) = element.marked_content_id_at_index(i) {
            all_mcids.push(id);
        }
    }
    if let Some(id) = element.marked_content_id() {
        if !all_mcids.contains(&id) {
            all_mcids.push(id);
        }
    }

    // Check elements by mcid
    for obj in objects {
        let obj_mcid = obj.get_marked_content_id();
        if obj_mcid >= 0 && all_mcids.contains(&obj_mcid) {
            if let Some(t) = obj.get_text(text_page) {
                text_content.push_str(&t);
            }
        }
    }

    let mcid_str = if all_mcids.is_empty() {
        "".to_string()
    } else {
        format!(" mcids={:?}", all_mcids)
    };

    if alt_text.is_empty() {
        if text_content.is_empty() {
            println!("{:indent$}<{}{}>", "", tag_type, mcid_str, indent = indent);
        } else {
            println!(
                "{:indent$}<{}{}>: {}",
                "",
                tag_type,
                mcid_str,
                text_content,
                indent = indent
            );
        }
    } else {
        println!(
            "{:indent$}<{}{} alt=\"{}\">: {}",
            "",
            tag_type,
            mcid_str,
            alt_text,
            text_content,
            indent = indent
        );
    }

    let child_count = element.count_children();
    for i in 0..child_count {
        if let Ok(child) = element.child(i) {
            print_tree(&child, text_page, objects, indent + 2);
        }
    }
}

fn main() -> PdfiumResult<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: parse_tagged_tables <path/to/pdf>");
        return Ok(());
    }
    let path = &args[1];
    let document = PdfiumDocument::new_from_path(path, None)?;

    println!("Document loaded successfully.");
    let page_count = document.pages().count();
    println!("Total pages: {}", page_count);

    // Let's just check the first 20 pages or so to see if tables/lists show up
    for i in 0..page_count {
        let page_result = document.page(i as i32);
        if let Ok(page) = page_result {
            let text_page = page.text()?;

            let mut objects = Vec::new();
            for obj in page.objects().flatten() {
                objects.push(obj);
            }

            if let Some(tree) = page.struct_tree() {
                let children_count = tree.count_children();

                if children_count > 0 {
                    println!("\nPage {} Structure Tree:", i + 1);
                    for j in 0..children_count {
                        if let Ok(child) = tree.child(j) {
                            print_tree(&child, &text_page, &objects, 2);
                        }
                    }
                }
            }
        }

        if i > 50 {
            break;
        } // stop early if it's too long
    }

    Ok(())
}
