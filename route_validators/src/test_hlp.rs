use crate::GetAxumHttpStatusCode;
use axum::http::StatusCode;
use std::{
    fmt,
    task::{Context, Poll, Waker},
    thread::yield_now,
};
pub(crate) fn block_on<T>(input_future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(input_future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => {
                return output;
            }
            Poll::Pending => {
                yield_now();
            }
        }
    }
}
pub(crate) fn expect_ok<T, E>(v: Result<T, E>, exp_id: &'static str) -> T
where
    E: fmt::Debug,
{
    let Ok(ok_v) = v else {
        panic!("unexpected Err for expect_ok, id={exp_id}");
    };
    ok_v
}
pub(crate) fn expect_er<T, E>(v: Result<T, E>, exp_id: &'static str) -> E
where
    T: fmt::Debug,
{
    let Err(er_v) = v else {
        panic!("unexpected Ok for expect_er, id={exp_id}");
    };
    er_v
}
pub(crate) fn assert_status_code<T>(v: &T, expected: StatusCode)
where
    T: GetAxumHttpStatusCode,
{
    assert_eq!(v.get_axum_http_status_code(), expected);
}
