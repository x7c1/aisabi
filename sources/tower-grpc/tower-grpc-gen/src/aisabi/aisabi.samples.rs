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
    use super::{HelloReply, HelloRequest};
    use ::tower_grpc::codegen::client::*;

    #[derive(Debug, Clone)]
    pub struct Greeter<T> {
        inner: grpc::Grpc<T>,
    }

    impl<T> Greeter<T> {
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

        pub fn say_hello<R>(
            &mut self,
            request: grpc::Request<HelloRequest>,
        ) -> grpc::unary::ResponseFuture<HelloReply, T::Future, T::ResponseBody>
        where
            T: grpc::GrpcService<R>,
            grpc::unary::Once<HelloRequest>: grpc::Encodable<R>,
        {
            let path = http::PathAndQuery::from_static("/aisabi.samples.Greeter/SayHello");
            self.inner.unary(request, path)
        }
    }
}

pub mod server {
    use super::{HelloReply, HelloRequest};
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

    pub trait Greeter: Clone {
        type SayHelloFuture: futures::Future<
            Item = grpc::Response<HelloReply>,
            Error = grpc::Status,
        >;

        fn say_hello(&mut self, request: grpc::Request<HelloRequest>) -> Self::SayHelloFuture;
    }

    #[derive(Debug, Clone)]
    pub struct GreeterServer<T> {
        greeter: T,
    }

    impl<T> GreeterServer<T>
    where
        T: Greeter,
    {
        pub fn new(greeter: T) -> Self {
            Self { greeter }
        }
    }

    impl<T> tower::Service<http::Request<grpc::BoxBody>> for GreeterServer<T>
    where
        T: Greeter,
    {
        type Response = http::Response<greeter::ResponseBody<T>>;
        type Error = grpc::Never;
        type Future = greeter::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: http::Request<grpc::BoxBody>) -> Self::Future {
            use self::greeter::Kind::*;

            match request.uri().path() {
                "/aisabi.samples.Greeter/SayHello" => {
                    let service = greeter::methods::SayHello(self.greeter.clone());
                    let response = grpc::unary(service, request);
                    greeter::ResponseFuture {
                        kind: SayHello(response),
                    }
                }
                _ => greeter::ResponseFuture {
                    kind: __Generated__Unimplemented(grpc::unimplemented(format!(
                        "unknown service: {:?}",
                        request.uri().path()
                    ))),
                },
            }
        }
    }

    impl<T> tower::Service<()> for GreeterServer<T>
    where
        T: Greeter,
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

    impl<T> tower::Service<http::Request<tower_hyper::Body>> for GreeterServer<T>
    where
        T: Greeter,
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

    pub mod greeter {
        use super::super::HelloRequest;
        use super::Greeter;
        use ::tower_grpc::codegen::server::*;

        pub struct ResponseFuture<T>
        where
            T: Greeter,
        {
            pub(super) kind: Kind<
                // SayHello
                grpc::unary::ResponseFuture<methods::SayHello<T>, grpc::BoxBody, HelloRequest>,
                // A generated catch-all for unimplemented service calls
                grpc::unimplemented::ResponseFuture,
            >,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where
            T: Greeter,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = grpc::Never;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| ResponseBody {
                            kind: SayHello(body),
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
            T: Greeter,
        {
            pub(super) kind: Kind<
                // SayHello
                grpc::Encode<
                    grpc::unary::Once<
                        <methods::SayHello<T> as grpc::UnaryService<HelloRequest>>::Response,
                    >,
                >,
                // A generated catch-all for unimplemented service calls
                (),
            >,
        }

        impl<T> tower::HttpBody for ResponseBody<T>
        where
            T: Greeter,
        {
            type Data = <grpc::BoxBody as grpc::Body>::Data;
            type Error = grpc::Status;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref v) => v.is_end_stream(),
                    __Generated__Unimplemented(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref mut v) => v.poll_data(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    SayHello(ref mut v) => v.poll_trailers(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub(super) enum Kind<SayHello, __Generated__Unimplemented> {
            SayHello(SayHello),
            __Generated__Unimplemented(__Generated__Unimplemented),
        }

        pub mod methods {
            use super::super::{Greeter, HelloReply, HelloRequest};
            use ::tower_grpc::codegen::server::*;

            pub struct SayHello<T>(pub T);

            impl<T> tower::Service<grpc::Request<HelloRequest>> for SayHello<T>
            where
                T: Greeter,
            {
                type Response = grpc::Response<HelloReply>;
                type Error = grpc::Status;
                type Future = T::SayHelloFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<HelloRequest>) -> Self::Future {
                    self.0.say_hello(request)
                }
            }
        }
    }
}
