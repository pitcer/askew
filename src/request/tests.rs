use crate::request::macros::{declare_requests, delegate_requests};
use crate::request::sieve::RequestSieve;
use crate::request::{RequestHandler, Response};

#[test]
fn handler_without_sieve() {
    let result = Handler.handle(Foo);
    assert!(result.is_ok());
    let result = Handler.handle(Bar);
    assert!(result.is_ok());
    let result = Handler.handle(Qux);
    assert!(result.is_ok());
}

#[test]
fn identity_sieve() {
    let sieve = RequestSieve::<Identity, _>::new(Handler);
    let result = sieve.handle(Foo);
    assert!(result.is_ok());
    let result = sieve.handle(Bar);
    assert!(result.is_ok());
    let result = sieve.handle(Qux);
    assert!(result.is_ok());
}

#[test]
fn exclude_foo_sieve() {
    let sieve = RequestSieve::<ExcludeFoo, _>::new(Handler);
    let result = sieve.handle(Foo);
    assert!(result.is_err());
    let result = sieve.handle(Bar);
    assert!(result.is_ok());
    let result = sieve.handle(Qux);
    assert!(result.is_ok());
}

#[test]
fn exclude_bar_sieve() {
    let sieve = RequestSieve::<ExcludeBar, _>::new(Handler);
    let result = sieve.handle(Foo);
    assert!(result.is_ok());
    let result = sieve.handle(Bar);
    assert!(result.is_err());
    let result = sieve.handle(Qux);
    assert!(result.is_ok());
}

#[test]
fn sieve_composition() {
    let sieve = RequestSieve::<ExcludeFoo, _>::new(Handler);
    let sieve = RequestSieve::<ExcludeBar, _>::new(sieve);
    let result = sieve.handle(Foo);
    assert!(result.is_err());
    let result = sieve.handle(Bar);
    assert!(result.is_err());
    let result = sieve.handle(Qux);
    assert!(result.is_ok());
}

pub struct Handler;

impl RequestHandler<Foo> for Handler {
    fn handle(&self, _request: Foo) -> Response<Foo> {
        Ok(())
    }
}

impl RequestHandler<Bar> for Handler {
    fn handle(&self, _request: Bar) -> Response<Bar> {
        Ok(())
    }
}

impl RequestHandler<Qux> for Handler {
    fn handle(&self, _request: Qux) -> Response<Qux> {
        Ok(())
    }
}

declare_requests! {
    { Foo () -> () },
    { Bar () -> () },
    { Qux () -> () },
}

pub struct Identity;

delegate_requests! {
    <T> RequestSieve<Identity, T> {
        { Foo => T },
        { Bar => T },
        { Qux => T },
    }
}

pub struct ExcludeFoo;

delegate_requests! {
    <T> RequestSieve<ExcludeFoo, T> {
        { Foo => ! },
        { Bar => T },
        { Qux => T },
    }
}

pub struct ExcludeBar;

delegate_requests! {
    <T> RequestSieve<ExcludeBar, T> {
        { Foo => T },
        { Bar => ! },
        { Qux => T },
    }
}
