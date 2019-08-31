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


// interface

pub trait HealthCheck {
    fn echo(&self, o: ::grpc::RequestOptions, p: super::health_check::EchoRequest) -> ::grpc::SingleResponse<super::health_check::HelloReply>;

    fn echo_database(&self, o: ::grpc::RequestOptions, p: super::health_check::HelloRequest) -> ::grpc::SingleResponse<super::health_check::HelloReply>;
}

// client

pub struct HealthCheckClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Echo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::health_check::EchoRequest, super::health_check::HelloReply>>,
    method_EchoDatabase: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::health_check::HelloRequest, super::health_check::HelloReply>>,
}

impl ::grpc::ClientStub for HealthCheckClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        HealthCheckClient {
            grpc_client: grpc_client,
            method_Echo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/aisabi.health_check.HealthCheck/Echo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_EchoDatabase: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/aisabi.health_check.HealthCheck/EchoDatabase".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl HealthCheck for HealthCheckClient {
    fn echo(&self, o: ::grpc::RequestOptions, p: super::health_check::EchoRequest) -> ::grpc::SingleResponse<super::health_check::HelloReply> {
        self.grpc_client.call_unary(o, p, self.method_Echo.clone())
    }

    fn echo_database(&self, o: ::grpc::RequestOptions, p: super::health_check::HelloRequest) -> ::grpc::SingleResponse<super::health_check::HelloReply> {
        self.grpc_client.call_unary(o, p, self.method_EchoDatabase.clone())
    }
}

// server

pub struct HealthCheckServer;


impl HealthCheckServer {
    pub fn new_service_def<H : HealthCheck + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/aisabi.health_check.HealthCheck",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/aisabi.health_check.HealthCheck/Echo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.echo(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/aisabi.health_check.HealthCheck/EchoDatabase".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.echo_database(o, p))
                    },
                ),
            ],
        )
    }
}
