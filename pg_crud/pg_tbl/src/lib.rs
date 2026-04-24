use app_state::{
    GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetPgPool, GetSrcPlaceType,
    GetTimezone,
};
pub use gen_pg_tbl::*;
pub trait CombinationOfAppStateLogicTraits:
    GetEnableApiGitCommitCheck
    + GetMaximumSizeOfHttpBodyInBytes
    + GetSrcPlaceType
    + GetTimezone
    + GetPgPool
    + Send
    + Sync
{
}
#[derive(Clone, Copy)]
enum InsertValuesFmt {
    Raw,
    Wrapped,
}
#[derive(Clone, Copy)]
enum SelectWhereFmt {
    Plain,
    Where,
}
#[derive(Clone, Copy)]
enum UpdateSelectorFmt {
    Eq,
    InList,
}
fn gen_insert_query_string(
    tbl: &str,
    cols: &str,
    values: &str,
    cols_to_return: &str,
    insert_values_fmt: InsertValuesFmt,
) -> String {
    match insert_values_fmt {
        InsertValuesFmt::Raw => {
            format!("insert into {tbl} ({cols}) values {values} returning {cols_to_return}")
        }
        InsertValuesFmt::Wrapped => {
            format!("insert into {tbl} ({cols}) values ({values}) returning {cols_to_return}")
        }
    }
}
fn gen_select_query_string(
    tbl: &str,
    sel_string: &str,
    wh_string: &str,
    select_where_fmt: SelectWhereFmt,
) -> String {
    match select_where_fmt {
        SelectWhereFmt::Plain => format!("select {sel_string} from {tbl} {wh_string}"),
        SelectWhereFmt::Where => format!("select {sel_string} from {tbl} where {wh_string}"),
    }
}
fn gen_update_query_string(
    tbl: &str,
    cols_or_els: &str,
    pk_field_name: &str,
    pk_selector: &str,
    cols_to_return: &str,
    update_selector_fmt: UpdateSelectorFmt,
) -> String {
    match update_selector_fmt {
        UpdateSelectorFmt::Eq => {
            format!(
                "update {tbl} set {cols_or_els} where {pk_field_name} = {pk_selector} returning {cols_to_return}"
            )
        }
        UpdateSelectorFmt::InList => {
            format!(
                "update {tbl} set {cols_or_els} where {pk_field_name} in ({pk_selector}) returning {cols_to_return}"
            )
        }
    }
}
fn gen_delete_query_string(tbl: &str, pk_field_name: &str, wh_string: Option<&str>) -> String {
    wh_string.map_or_else(
        || format!("delete from {tbl} where {pk_field_name} = $1 returning {pk_field_name}"),
        |v| format!("delete from {tbl} {v} returning {pk_field_name}"),
    )
}
#[must_use]
pub fn gen_cm_query_string(tbl: &str, cols: &str, values: &str, cols_to_return: &str) -> String {
    gen_insert_query_string(tbl, cols, values, cols_to_return, InsertValuesFmt::Raw)
}
#[must_use]
pub fn gen_co_query_string(tbl: &str, cols: &str, values: &str, cols_to_return: &str) -> String {
    gen_insert_query_string(tbl, cols, values, cols_to_return, InsertValuesFmt::Wrapped)
}
#[must_use]
pub fn gen_rm_query_string(tbl: &str, sel_string: &str, wh_string: &str) -> String {
    gen_select_query_string(tbl, sel_string, wh_string, SelectWhereFmt::Plain)
}
#[must_use]
pub fn gen_ro_query_string(tbl: &str, sel_string: &str, wh_string: &str) -> String {
    gen_select_query_string(tbl, sel_string, wh_string, SelectWhereFmt::Where)
}
#[must_use]
pub fn gen_col_queals_v_comma_uo_qp(col: &str, value: &str) -> String {
    format!("{col} = {value},")
}
#[must_use]
pub fn gen_when_col_id_then_v_um_qp(col: &str, id: &str, value: &str) -> String {
    format!("when {col} = {id} then {value} ")
}
#[must_use]
pub fn gen_col_eqs_case_acc_else_col_end_comma_um_qp(col: &str, acc: &str) -> String {
    format!("{col} = case {acc}else {col} end,")
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_um_query_string(
    tbl: &str,
    els: &str,
    pk_field_name: &str,
    pks: &str,
    cols_to_return: &str,
) -> String {
    gen_update_query_string(
        tbl,
        els,
        pk_field_name,
        pks,
        cols_to_return,
        UpdateSelectorFmt::InList,
    )
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_uo_query_string(
    tbl: &str,
    cols: &str,
    pk_field_name: &str,
    pk_qp: &str,
    cols_to_return: &str,
) -> String {
    gen_update_query_string(
        tbl,
        cols,
        pk_field_name,
        pk_qp,
        cols_to_return,
        UpdateSelectorFmt::Eq,
    )
}
#[must_use]
pub fn gen_dm_query_string(tbl: &str, wh_string: &str, pk_field_name: &str) -> String {
    gen_delete_query_string(tbl, pk_field_name, Some(wh_string))
}
#[must_use]
pub fn gen_dlo_query_string(tbl: &str, pk_field_name: &str) -> String {
    gen_delete_query_string(tbl, pk_field_name, None)
}
#[cfg(test)]
mod tests {
    use super::{
        gen_cm_query_string, gen_co_query_string, gen_col_eqs_case_acc_else_col_end_comma_um_qp,
        gen_col_queals_v_comma_uo_qp, gen_delete_query_string, gen_dlo_query_string,
        gen_dm_query_string, gen_rm_query_string, gen_ro_query_string, gen_um_query_string,
        gen_uo_query_string, gen_when_col_id_then_v_um_qp,
    };
    fn users_base() -> (&'static str, &'static str) {
        ("users", "id")
    }
    fn assert_q(actual: &str, expected: &'static str) {
        assert_eq!(actual, expected);
    }
    #[test]
    fn gen_cm_query_string_is_expected() {
        assert_q(
            &gen_cm_query_string("users", "id,name", "($1,$2),($3,$4)", "id"),
            "insert into users (id,name) values ($1,$2),($3,$4) returning id",
        );
    }
    #[test]
    fn gen_co_query_string_is_expected() {
        assert_q(
            &gen_co_query_string("users", "id,name", "$1,$2", "id"),
            "insert into users (id,name) values ($1,$2) returning id",
        );
    }
    #[test]
    fn gen_rm_query_string_is_expected() {
        assert_q(
            &gen_rm_query_string("users", "id,name", "order by id"),
            "select id,name from users order by id",
        );
    }
    #[test]
    fn gen_ro_query_string_is_expected() {
        assert_q(
            &gen_ro_query_string("users", "id,name", "id = $1"),
            "select id,name from users where id = $1",
        );
    }
    #[test]
    fn gen_col_queals_v_comma_uo_qp_is_expected() {
        assert_q(&gen_col_queals_v_comma_uo_qp("name", "$2"), "name = $2,");
    }
    #[test]
    fn gen_when_col_id_then_v_um_qp_is_expected() {
        assert_q(
            &gen_when_col_id_then_v_um_qp("id", "$1", "$2"),
            "when id = $1 then $2 ",
        );
    }
    #[test]
    fn gen_col_eqs_case_acc_else_col_end_comma_um_qp_is_expected() {
        assert_q(
            &gen_col_eqs_case_acc_else_col_end_comma_um_qp("name", "when id = $1 then $2 "),
            "name = case when id = $1 then $2 else name end,",
        );
    }
    #[test]
    fn gen_um_query_string_is_expected() {
        assert_q(
            &gen_um_query_string("users", "name = case ... end,", "id", "$1,$2", "id,name"),
            "update users set name = case ... end, where id in ($1,$2) returning id,name",
        );
    }
    #[test]
    fn gen_uo_query_string_is_expected() {
        assert_q(
            &gen_uo_query_string("users", "name = $2", "id", "$1", "id,name"),
            "update users set name = $2 where id = $1 returning id,name",
        );
    }
    #[test]
    fn gen_dm_query_string_is_expected() {
        assert_q(
            &gen_dm_query_string("users", "where id in ($1,$2)", "id"),
            "delete from users where id in ($1,$2) returning id",
        );
    }
    #[test]
    fn gen_dlo_query_string_is_expected() {
        let (tbl, pk) = users_base();
        assert_q(
            &gen_dlo_query_string(tbl, pk),
            "delete from users where id = $1 returning id",
        );
    }
    #[test]
    fn gen_um_query_string_wraps_pk_selector_for_in_clause() {
        let v = gen_um_query_string("users", "name = case ... end,", "id", "$1,$2", "id,name");
        assert!(v.contains("where id in ($1,$2)"));
    }
    #[test]
    fn gen_delete_query_string_uses_provided_filter_without_rewrite() {
        let (tbl, pk) = users_base();
        assert_q(
            &gen_delete_query_string(tbl, pk, Some("where id in ($1,$2) and active = true")),
            "delete from users where id in ($1,$2) and active = true returning id",
        );
    }
    #[test]
    fn gen_update_query_string_eq_keeps_selector_without_extra_wrapping() {
        assert_q(
            &super::gen_update_query_string(
                "users",
                "name = $2",
                "id",
                "$1",
                "id,name",
                super::UpdateSelectorFmt::Eq,
            ),
            "update users set name = $2 where id = $1 returning id,name",
        );
    }
    #[test]
    fn gen_update_query_string_in_list_wraps_selector_once() {
        assert_q(
            &super::gen_update_query_string(
                "users",
                "name = case ... end,",
                "id",
                "$1,$2",
                "id,name",
                super::UpdateSelectorFmt::InList,
            ),
            "update users set name = case ... end, where id in ($1,$2) returning id,name",
        );
    }
}
