use crate::objects::Object;

pub fn cat_file(pretty_print: bool, sha: &String) -> anyhow::Result<()> {
    let file_object = Object::read(sha)?;
    if pretty_print {
        println!("{}", file_object);
    }
    Ok(())
}
