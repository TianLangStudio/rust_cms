table! {
    tb_article (id) {
        id -> Varchar,
        title -> Nullable<Varchar>,
        subtitle -> Nullable<Varchar>,
        intro -> Nullable<Varchar>,
    }
}

table! {
    tb_article_content (id) {
        id -> Bigint,
        article_id -> Varchar,
        content -> Nullable<Longtext>,
    }
}

table! {
    tb_file (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        ext -> Nullable<Varchar>,
        is_private -> Nullable<Integer>,
        creater -> Varchar,
    }
}

table! {
    tb_login_info (id) {
        id -> Bigint,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    tb_article,
    tb_article_content,
    tb_file,
    tb_login_info,
);