extern crate dbus;
extern crate dbus_tokio;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate urlshortener;

use dbus::tree::MethodErr;
use dbus::{BusType, Connection, NameFlag};
use dbus_tokio::tree::{AFactory, ATree, ATreeServer};
use dbus_tokio::AConnection;
use std::rc::Rc;
use tokio_core::reactor::Core;

static DBUS_NAME: &str = "io.crates.urlshortener";
static DBUS_INTERFACE_NAME: &str = "io.crates.urlshortener";
static DBUS_OBJECT_PATH: &str = "/";
static DBUS_SHORTEN_METHOD_NAME: &str = "Shorten";

use futures::{future, Stream};

fn main() {
    let c = Rc::new(Connection::get_private(BusType::Session).unwrap());

    c.register_name(DBUS_NAME, NameFlag::ReplaceExisting as u32)
        .unwrap();

    let f = AFactory::new_afn::<()>();

    let tree = f.tree(ATree::new()).add(
        f.object_path(DBUS_OBJECT_PATH, ()).introspectable().add(
            f.interface(DBUS_INTERFACE_NAME, ()).add_m(
                f.amethod(DBUS_SHORTEN_METHOD_NAME, (), move |m| {
                    let long_url: String = match m.msg.get1() {
                        Some(s) => s,
                        None => {
                            return future::err(MethodErr::failed(&"Invalid argument".to_owned()))
                        }
                    };

                    let mret = m.msg.method_return();

                    let us = match urlshortener::client::UrlShortener::new() {
                        Ok(us) => us,
                        Err(e) => return future::err(MethodErr::failed(&e)),
                    };
                    let s = match us.try_generate(&long_url, None) {
                        Ok(short) => short,
                        Err(e) => return future::err(MethodErr::failed(&e)),
                    };
                    let mret = mret.append1(s);

                    let reply = vec![mret];
                    future::ok(reply)
                }).inarg::<String, _>("long_url")
                    .outarg::<&str, _>("reply"),
            ),
        ),
    );

    tree.set_registered(&c, true).unwrap();

    let mut core = Core::new().unwrap();
    let aconn = AConnection::new(c.clone(), core.handle()).unwrap();
    let server = ATreeServer::new(c.clone(), &tree, aconn.messages().unwrap());

    let server = server.for_each(|_| Ok(()));
    core.run(server).unwrap();
}
