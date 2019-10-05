// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_HEALTH_CHECK_ECHO: ::grpcio::Method<super::health_check::EchoRequest, super::health_check::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/aisabi.health_check.HealthCheck/Echo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_HEALTH_CHECK_ECHO_DATABASE: ::grpcio::Method<super::health_check::HelloRequest, super::health_check::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/aisabi.health_check.HealthCheck/EchoDatabase",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct HealthCheckClient {
    client: ::grpcio::Client,
}

impl HealthCheckClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        HealthCheckClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn echo_opt(&self, req: &super::health_check::EchoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::health_check::HelloReply> {
        self.client.unary_call(&METHOD_HEALTH_CHECK_ECHO, req, opt)
    }

    pub fn echo(&self, req: &super::health_check::EchoRequest) -> ::grpcio::Result<super::health_check::HelloReply> {
        self.echo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn echo_async_opt(&self, req: &super::health_check::EchoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::health_check::HelloReply>> {
        self.client.unary_call_async(&METHOD_HEALTH_CHECK_ECHO, req, opt)
    }

    pub fn echo_async(&self, req: &super::health_check::EchoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::health_check::HelloReply>> {
        self.echo_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn echo_database_opt(&self, req: &super::health_check::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::health_check::HelloReply> {
        self.client.unary_call(&METHOD_HEALTH_CHECK_ECHO_DATABASE, req, opt)
    }

    pub fn echo_database(&self, req: &super::health_check::HelloRequest) -> ::grpcio::Result<super::health_check::HelloReply> {
        self.echo_database_opt(req, ::grpcio::CallOption::default())
    }

    pub fn echo_database_async_opt(&self, req: &super::health_check::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::health_check::HelloReply>> {
        self.client.unary_call_async(&METHOD_HEALTH_CHECK_ECHO_DATABASE, req, opt)
    }

    pub fn echo_database_async(&self, req: &super::health_check::HelloRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::health_check::HelloReply>> {
        self.echo_database_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait HealthCheck {
    fn echo(&mut self, ctx: ::grpcio::RpcContext, req: super::health_check::EchoRequest, sink: ::grpcio::UnarySink<super::health_check::HelloReply>);
    fn echo_database(&mut self, ctx: ::grpcio::RpcContext, req: super::health_check::HelloRequest, sink: ::grpcio::UnarySink<super::health_check::HelloReply>);
}

pub fn create_health_check<S: HealthCheck + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_HEALTH_CHECK_ECHO, move |ctx, req, resp| {
        instance.echo(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_HEALTH_CHECK_ECHO_DATABASE, move |ctx, req, resp| {
        instance.echo_database(ctx, req, resp)
    });
    builder.build()
}
