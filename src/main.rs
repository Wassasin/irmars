mod client;
mod request;
mod session;

pub use crate::request::*;
pub use crate::session::*;

use futures::future::Future;

fn main() {
    let dr = DisclosureRequest {
        disclose: AttributeConDisCon(vec![AttributeDisCon(vec![AttributeCon(vec![
            AttributeRequest {
                atype: "pbdf.pbdf.diploma.degree".to_string(),
                value: None,
                not_null: true,
            },
        ])])]),
        labels: None,
    };

    let client = crate::client::Client::new("http://localhost:8088".to_string()).unwrap();

    let res = client
        .request(&dr)
        .and_then(move |r| client.result(&r.token));
    /*.map(|resp| {
        println!("status: {:?}", resp);

        let resp2 = serde_json::to_string(&resp.session_ptr).unwrap();

        let code = qrcode::QrCode::new(resp2).unwrap();
        let scode = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();

        println!("\n\n{}", scode);
    });*/

    let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
    rt.block_on(res).unwrap();

    // println!("{}", );

    // println!("Hello, world!");
}
