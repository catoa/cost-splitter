use std::path::PathBuf;

pub fn process_receipt(receipt_path: PathBuf) {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    match lt.set_image(receipt_path) {
        Ok(_) => {
            println!("{}", lt.get_utf8_text().unwrap());
        }
        Err(err) => panic!("Unable to parse image, {}", err),
    }
}
