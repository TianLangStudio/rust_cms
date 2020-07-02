use crate::schema::{tb_file};

#[derive(Insertable)]
#[table_name = "tb_file"]
pub struct NewFileMod <'a>{
   pub id: &'a str,
   pub name: Option<&'a str>,
   pub  ext: Option<&'a str>,
    pub is_private: i32, 
    pub creater: &'a str
}
#[derive(Queryable)]
pub struct FileMod {
   pub id: String,
    pub name: Option<String>,
    pub ext: Option<String>,
    pub is_private: Option<i32>,
    pub creater: String
}