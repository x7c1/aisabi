#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(oneof = "hello_request::OptionalNickname", tags = "2")]
    pub optional_nickname: ::std::option::Option<hello_request::OptionalNickname>,
    #[prost(oneof = "hello_request::OptionalAddress", tags = "4")]
    pub optional_address: ::std::option::Option<hello_request::OptionalAddress>,
}
pub mod hello_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalNickname {
        #[prost(string, tag = "2")]
        Nickname(std::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalAddress {
        #[prost(string, tag = "4")]
        Address(std::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
pub mod client {
    use super::{EchoRequest, HelloReply, HelloRequest};
    use ::tower_grpc::codegen::client::*;

    #[derive(Debug, Clone)]
    pub struct HealthCheck<T> {
        inner: grpc::Grpc<T>,
    }

    impl<T> HealthCheck<T> {
        pub fn new(inner: T) -> Self {
            let inner = grpc::Grpc::new(inner);
            Self { inner }
        }

        /// Poll whether this client is ready to send another request.
        pub fn poll_ready<R>(&mut self) -> futures::Poll<(), grpc::Status>
        where
            T: grpc::GrpcService<R>,
        {
            self.inner.poll_ready()
        }

        /// Get a `Future` of when this client is ready to send another request.
        pub fn ready<R>(self) -> impl futures::Future<Item = Self, Error = grpc::Status>
        where
            T: grpc::GrpcService<R>,
        {
            futures::Future::map(self.inner.ready(), |inner| Self { inner })
        }

        pub fn echo<R>(
            &mut self,
            request: grpc::Request<EchoRequest>,
        ) -> grpc::unary::ResponseFuture<HelloReply, T::Future, T::ResponseBody>
        where
            T: grpc::GrpcService<R>,
            grpc::unary::Once<EchoRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/aisabi.health_check.HealthCheck/Echo");
            self.inner.unary(request, path)
        }

        pub fn echo_database<R>(
            &mut self,
            request: grpc::Request<HelloRequest>,
        ) -> grpc::unary::ResponseFuture<HelloReply, T::Future, T::ResponseBody>
        where
            T: grpc::GrpcService<R>,
            grpc::unary::Once<HelloRequest>: grpc::Encodable<R>,
        {
            let path =
                http::PathAndQuery::from_static("/aisabi.health_check.HealthCheck/EchoDatabase");
            self.inner.unary(request, path)
        }
    }
}

pub mod server {
    use super::{EchoRequest, HelloReply, HelloRequest};
    use ::tower_grpc::codegen::server::*;

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => {
            match $e {
                Ok(futures::Async::Ready(t)) => t,
                Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
                Err(e) => return Err(From::from(e)),
            }
        };
    }

    pub trait HealthCheck: Clone {
        type EchoFuture: futures::Future<Item = grpc::Response<HelloReply>, Error = grpc::Status>;
        type EchoDatabaseFuture: futures::Future<
            Item = grpc::Response<HelloReply>,
            Error = grpc::Status,
        >;

        fn echo(&mut self, request: grpc::Request<EchoRequest>) -> Self::EchoFuture;

        fn echo_database(
            &mut self,
            request: grpc::Request<HelloRequest>,
        ) -> Self::EchoDatabaseFuture;
    }

    #[derive(Debug, Clone)]
    pub struct HealthCheckServer<T> {
        health_check: T,
    }

    impl<T> HealthCheckServer<T>
    where
        T: HealthCheck,
    {
        pub fn new(health_check: T) -> Self {
            Self { health_check }
        }
    }

    impl<T> tower::Service<http::Request<grpc::BoxBody>> for HealthCheckServer<T>
    where
        T: HealthCheck,
    {
        type Response = http::Response<health_check::ResponseBody<T>>;
        type Error = grpc::Never;
        type Future = health_check::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: http::Request<grpc::BoxBody>) -> Self::Future {
            use self::health_check::Kind::*;

            match request.uri().path() {
                "/aisabi.health_check.HealthCheck/Echo" => {
                    let service = health_check::methods::Echo(self.health_check.clone());
                    let response = grpc::unary(service, request);
                    health_check::ResponseFuture {
                        kind: Echo(response),
                    }
                }
                "/aisabi.health_check.HealthCheck/EchoDatabase" => {
                    let service = health_check::methods::EchoDatabase(self.health_check.clone());
                    let response = grpc::unary(service, request);
                    health_check::ResponseFuture {
                        kind: EchoDatabase(response),
                    }
                }
                _ => health_check::ResponseFuture {
                    kind: __Generated__Unimplemented(grpc::unimplemented(format!(
                        "unknown service: {:?}",
                        request.uri().path()
                    ))),
                },
            }
        }
    }

    impl<T> tower::Service<()> for HealthCheckServer<T>
    where
        T: HealthCheck,
    {
        type Response = Self;
        type Error = grpc::Never;
        type Future = futures::FutureResult<Self::Response, Self::Error>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(futures::Async::Ready(()))
        }

        fn call(&mut self, _target: ()) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    impl<T> tower::Service<http::Request<tower_hyper::Body>> for HealthCheckServer<T>
    where
        T: HealthCheck,
    {
        type Response = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Response;
        type Error = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Error;
        type Future = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Future;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            tower::Service::<http::Request<grpc::BoxBody>>::poll_ready(self)
        }

        fn call(&mut self, request: http::Request<tower_hyper::Body>) -> Self::Future {
            let request = request.map(|b| grpc::BoxBody::map_from(b));
            tower::Service::<http::Request<grpc::BoxBody>>::call(self, request)
        }
    }

    pub mod health_check {
        use super::super::{EchoRequest, HelloRequest};
        use super::HealthCheck;
        use ::tower_grpc::codegen::server::*;

        pub struct ResponseFuture<T>
        where
            T: HealthCheck,
        {
            pub(super) kind: Kind<
                // Echo
                grpc::unary::ResponseFuture<methods::Echo<T>, grpc::BoxBody, EchoRequest>,
                // EchoDatabase
                grpc::unary::ResponseFuture<methods::EchoDatabase<T>, grpc::BoxBody, HelloRequest>,
                // A generated catch-all for unimplemented service calls
                grpc::unimplemented::ResponseFuture,
            >,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where
            T: HealthCheck,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = grpc::Never;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Echo(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| ResponseBody { kind: Echo(body) });
                        Ok(response.into())
                    }
                    EchoDatabase(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| ResponseBody {
                            kind: EchoDatabase(body),
                        });
                        Ok(response.into())
                    }
                    __Generated__Unimplemented(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| ResponseBody {
                            kind: __Generated__Unimplemented(body),
                        });
                        Ok(response.into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where
            T: HealthCheck,
        {
            pub(super) kind: Kind<
                // Echo
                grpc::Encode<
                    grpc::unary::Once<
                        <methods::Echo<T> as grpc::UnaryService<EchoRequest>>::Response,
                    >,
                >,
                // EchoDatabase
                grpc::Encode<
                    grpc::unary::Once<
                        <methods::EchoDatabase<T> as grpc::UnaryService<HelloRequest>>::Response,
                    >,
                >,
                // A generated catch-all for unimplemented service calls
                (),
            >,
        }

        impl<T> tower::HttpBody for ResponseBody<T>
        where
            T: HealthCheck,
        {
            type Data = <grpc::BoxBody as grpc::Body>::Data;
            type Error = grpc::Status;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    Echo(ref v) => v.is_end_stream(),
                    EchoDatabase(ref v) => v.is_end_stream(),
                    __Generated__Unimplemented(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Echo(ref mut v) => v.poll_data(),
                    EchoDatabase(ref mut v) => v.poll_data(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Echo(ref mut v) => v.poll_trailers(),
                    EchoDatabase(ref mut v) => v.poll_trailers(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub(super) enum Kind<Echo, EchoDatabase, __Generated__Unimplemented> {
            Echo(Echo),
            EchoDatabase(EchoDatabase),
            __Generated__Unimplemented(__Generated__Unimplemented),
        }

        pub mod methods {
            use super::super::{EchoRequest, HealthCheck, HelloReply, HelloRequest};
            use ::tower_grpc::codegen::server::*;

            pub struct Echo<T>(pub T);

            impl<T> tower::Service<grpc::Request<EchoRequest>> for Echo<T>
            where
                T: HealthCheck,
            {
                type Response = grpc::Response<HelloReply>;
                type Error = grpc::Status;
                type Future = T::EchoFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<EchoRequest>) -> Self::Future {
                    self.0.echo(request)
                }
            }

            pub struct EchoDatabase<T>(pub T);

            impl<T> tower::Service<grpc::Request<HelloRequest>> for EchoDatabase<T>
            where
                T: HealthCheck,
            {
                type Response = grpc::Response<HelloReply>;
                type Error = grpc::Status;
                type Future = T::EchoDatabaseFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<HelloRequest>) -> Self::Future {
                    self.0.echo_database(request)
                }
            }
        }
    }
}
