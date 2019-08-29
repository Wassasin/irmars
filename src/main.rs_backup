mod client;
mod request;
mod session;

pub use crate::request::*;
pub use crate::session::*;

use futures::future::{loop_fn, ok, Either, Future, Loop};
use std::time::{Duration, Instant};
use tokio::timer::Delay;

fn print_qr(sp: &SessionPackage) {
    let s = serde_json::to_string(&sp.session_ptr).unwrap();

    let code = qrcode::QrCode::new(s).unwrap();
    let scode = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    println!("\n\n{}", scode);
}

fn main() {
    let dr = DisclosureRequest {
        disclose: AttributeConDisCon(vec![AttributeDisCon(vec![AttributeCon(vec![
            AttributeRequest {
                atype: "pbdf.pbdf.email.email".to_string(),
                value: None,
                not_null: true,
            },
        ])])]),
        labels: None,
    };

    let client = crate::client::Client::new("http://localhost:8088".to_string()).unwrap();

    let res = client
        .request(&dr)
        .and_then(move |sp| {
            print_qr(&sp);

            loop_fn(120, move |i: u8| {
                client.result(&sp.token).and_then(move |r: SessionResult| {
                    if r.status != SessionStatus::Done && i > 0 {
                        Either::A(
                            Delay::new(Instant::now() + Duration::new(0, 500_000_000))
                                .then(move |_| Ok(Loop::Continue(i - 1))),
                        )
                    } else {
                        Either::B(ok(Loop::Break(r)))
                    }
                })
            })
        })
        .and_then(|r: SessionResult| {
            println!("{:?}", r);
            Ok(())
        });

    let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
    rt.block_on(res).unwrap();
}
