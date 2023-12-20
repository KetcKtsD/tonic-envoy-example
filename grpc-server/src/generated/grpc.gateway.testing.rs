#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub message_count: i32,
}
/// Request type for server side streaming echo.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStreamingEchoRequest {
    /// Message string for server streaming request.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The total number of messages to be generated before the server
    /// closes the stream; default is 10.
    #[prost(int32, tag = "2")]
    pub message_count: i32,
    /// The interval (ms) between two server messages. The server implementation
    /// may enforce some minimum interval (e.g. 100ms) to avoid message overflow.
    #[prost(int32, tag = "3")]
    pub message_interval: i32,
}
/// Response type for server streaming response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStreamingEchoResponse {
    /// Response message.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Request type for client side streaming echo.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStreamingEchoRequest {
    /// A special value "" indicates that there's no further messages.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Response type for client side streaming echo.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStreamingEchoResponse {
    /// Total number of client messages that have been received.
    #[prost(int32, tag = "1")]
    pub message_count: i32,
}
/// Generated server implementations.
pub mod echo_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EchoServiceServer.
    #[async_trait]
    pub trait EchoService: Send + Sync + 'static {
        /// One request followed by one response
        /// The server returns the client message as-is.
        async fn echo(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> std::result::Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        /// Sends back abort status.
        async fn echo_abort(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> std::result::Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        /// One empty request, ZERO processing, followed by one empty response
        /// (minimum effort to do message serialization).
        async fn no_op(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> std::result::Result<tonic::Response<super::Empty>, tonic::Status>;
        /// Server streaming response type for the ServerStreamingEcho method.
        type ServerStreamingEchoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ServerStreamingEchoResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// One request followed by a sequence of responses (streamed download).
        /// The server will return the same client message repeatedly.
        async fn server_streaming_echo(
            &self,
            request: tonic::Request<super::ServerStreamingEchoRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ServerStreamingEchoStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ServerStreamingEchoAbort method.
        type ServerStreamingEchoAbortStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ServerStreamingEchoResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// One request followed by a sequence of responses (streamed download).
        /// The server abort directly.
        async fn server_streaming_echo_abort(
            &self,
            request: tonic::Request<super::ServerStreamingEchoRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ServerStreamingEchoAbortStream>,
            tonic::Status,
        >;
        /// A sequence of requests followed by one response (streamed upload).
        /// The server returns the total number of messages as the result.
        /// Notice: Client side streaming and Bidi streaming are not supported at the moment.
        async fn client_streaming_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::ClientStreamingEchoRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::ClientStreamingEchoResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the FullDuplexEcho method.
        type FullDuplexEchoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::EchoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// A sequence of requests with each message echoed by the server immediately.
        /// The server returns the same client messages in order.
        /// E.g. this is how the speech API works.
        /// Notice: Client side streaming and Bidi streaming are not supported at the moment.
        async fn full_duplex_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::FullDuplexEchoStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the HalfDuplexEcho method.
        type HalfDuplexEchoStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::EchoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// A sequence of requests followed by a sequence of responses.
        /// The server buffers all the client messages and then returns the same
        /// client messages one by one after the client half-closes the stream.
        /// This is how an image recognition API may work.
        /// Notice: Client side streaming and Bidi streaming are not supported at the moment.
        async fn half_duplex_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::HalfDuplexEchoStream>,
            tonic::Status,
        >;
    }
    /// A simple echo service.
    #[derive(Debug)]
    pub struct EchoServiceServer<T: EchoService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EchoService> EchoServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EchoServiceServer<T>
    where
        T: EchoService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.gateway.testing.EchoService/Echo" => {
                    #[allow(non_camel_case_types)]
                    struct EchoSvc<T: EchoService>(pub Arc<T>);
                    impl<T: EchoService> tonic::server::UnaryService<super::EchoRequest>
                    for EchoSvc<T> {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::echo(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/EchoAbort" => {
                    #[allow(non_camel_case_types)]
                    struct EchoAbortSvc<T: EchoService>(pub Arc<T>);
                    impl<T: EchoService> tonic::server::UnaryService<super::EchoRequest>
                    for EchoAbortSvc<T> {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::echo_abort(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EchoAbortSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/NoOp" => {
                    #[allow(non_camel_case_types)]
                    struct NoOpSvc<T: EchoService>(pub Arc<T>);
                    impl<T: EchoService> tonic::server::UnaryService<super::Empty>
                    for NoOpSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Empty>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::no_op(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NoOpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/ServerStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct ServerStreamingEchoSvc<T: EchoService>(pub Arc<T>);
                    impl<
                        T: EchoService,
                    > tonic::server::ServerStreamingService<
                        super::ServerStreamingEchoRequest,
                    > for ServerStreamingEchoSvc<T> {
                        type Response = super::ServerStreamingEchoResponse;
                        type ResponseStream = T::ServerStreamingEchoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerStreamingEchoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::server_streaming_echo(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ServerStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/ServerStreamingEchoAbort" => {
                    #[allow(non_camel_case_types)]
                    struct ServerStreamingEchoAbortSvc<T: EchoService>(pub Arc<T>);
                    impl<
                        T: EchoService,
                    > tonic::server::ServerStreamingService<
                        super::ServerStreamingEchoRequest,
                    > for ServerStreamingEchoAbortSvc<T> {
                        type Response = super::ServerStreamingEchoResponse;
                        type ResponseStream = T::ServerStreamingEchoAbortStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServerStreamingEchoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::server_streaming_echo_abort(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ServerStreamingEchoAbortSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/ClientStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct ClientStreamingEchoSvc<T: EchoService>(pub Arc<T>);
                    impl<
                        T: EchoService,
                    > tonic::server::ClientStreamingService<
                        super::ClientStreamingEchoRequest,
                    > for ClientStreamingEchoSvc<T> {
                        type Response = super::ClientStreamingEchoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ClientStreamingEchoRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::client_streaming_echo(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClientStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/FullDuplexEcho" => {
                    #[allow(non_camel_case_types)]
                    struct FullDuplexEchoSvc<T: EchoService>(pub Arc<T>);
                    impl<
                        T: EchoService,
                    > tonic::server::StreamingService<super::EchoRequest>
                    for FullDuplexEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type ResponseStream = T::FullDuplexEchoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::full_duplex_echo(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FullDuplexEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.gateway.testing.EchoService/HalfDuplexEcho" => {
                    #[allow(non_camel_case_types)]
                    struct HalfDuplexEchoSvc<T: EchoService>(pub Arc<T>);
                    impl<
                        T: EchoService,
                    > tonic::server::StreamingService<super::EchoRequest>
                    for HalfDuplexEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type ResponseStream = T::HalfDuplexEchoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EchoService>::half_duplex_echo(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HalfDuplexEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: EchoService> Clone for EchoServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: EchoService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EchoService> tonic::server::NamedService for EchoServiceServer<T> {
        const NAME: &'static str = "grpc.gateway.testing.EchoService";
    }
}
