use rust_wordnik;
pub fn there_such_word(w:&str)->bool{
 !rust_wordnik::get_definitions(w).is_empty()
}