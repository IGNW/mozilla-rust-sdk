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

const METHOD_LANGUAGE_SERVICE_ANALYZE_SENTIMENT: ::grpcio::Method<super::language_service::AnalyzeSentimentRequest, super::language_service::AnalyzeSentimentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.language.v1.LanguageService/AnalyzeSentiment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITIES: ::grpcio::Method<super::language_service::AnalyzeEntitiesRequest, super::language_service::AnalyzeEntitiesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.language.v1.LanguageService/AnalyzeEntities",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITY_SENTIMENT: ::grpcio::Method<super::language_service::AnalyzeEntitySentimentRequest, super::language_service::AnalyzeEntitySentimentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.language.v1.LanguageService/AnalyzeEntitySentiment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LANGUAGE_SERVICE_ANALYZE_SYNTAX: ::grpcio::Method<super::language_service::AnalyzeSyntaxRequest, super::language_service::AnalyzeSyntaxResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.language.v1.LanguageService/AnalyzeSyntax",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LANGUAGE_SERVICE_CLASSIFY_TEXT: ::grpcio::Method<super::language_service::ClassifyTextRequest, super::language_service::ClassifyTextResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.language.v1.LanguageService/ClassifyText",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LANGUAGE_SERVICE_ANNOTATE_TEXT: ::grpcio::Method<super::language_service::AnnotateTextRequest, super::language_service::AnnotateTextResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.language.v1.LanguageService/AnnotateText",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LanguageServiceClient {
    client: ::grpcio::Client,
}

impl LanguageServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LanguageServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn analyze_sentiment_opt(&self, req: &super::language_service::AnalyzeSentimentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::language_service::AnalyzeSentimentResponse> {
        self.client.unary_call(&METHOD_LANGUAGE_SERVICE_ANALYZE_SENTIMENT, req, opt)
    }

    pub fn analyze_sentiment(&self, req: &super::language_service::AnalyzeSentimentRequest) -> ::grpcio::Result<super::language_service::AnalyzeSentimentResponse> {
        self.analyze_sentiment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_sentiment_async_opt(&self, req: &super::language_service::AnalyzeSentimentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeSentimentResponse>> {
        self.client.unary_call_async(&METHOD_LANGUAGE_SERVICE_ANALYZE_SENTIMENT, req, opt)
    }

    pub fn analyze_sentiment_async(&self, req: &super::language_service::AnalyzeSentimentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeSentimentResponse>> {
        self.analyze_sentiment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_entities_opt(&self, req: &super::language_service::AnalyzeEntitiesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::language_service::AnalyzeEntitiesResponse> {
        self.client.unary_call(&METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITIES, req, opt)
    }

    pub fn analyze_entities(&self, req: &super::language_service::AnalyzeEntitiesRequest) -> ::grpcio::Result<super::language_service::AnalyzeEntitiesResponse> {
        self.analyze_entities_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_entities_async_opt(&self, req: &super::language_service::AnalyzeEntitiesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeEntitiesResponse>> {
        self.client.unary_call_async(&METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITIES, req, opt)
    }

    pub fn analyze_entities_async(&self, req: &super::language_service::AnalyzeEntitiesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeEntitiesResponse>> {
        self.analyze_entities_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_entity_sentiment_opt(&self, req: &super::language_service::AnalyzeEntitySentimentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::language_service::AnalyzeEntitySentimentResponse> {
        self.client.unary_call(&METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITY_SENTIMENT, req, opt)
    }

    pub fn analyze_entity_sentiment(&self, req: &super::language_service::AnalyzeEntitySentimentRequest) -> ::grpcio::Result<super::language_service::AnalyzeEntitySentimentResponse> {
        self.analyze_entity_sentiment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_entity_sentiment_async_opt(&self, req: &super::language_service::AnalyzeEntitySentimentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeEntitySentimentResponse>> {
        self.client.unary_call_async(&METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITY_SENTIMENT, req, opt)
    }

    pub fn analyze_entity_sentiment_async(&self, req: &super::language_service::AnalyzeEntitySentimentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeEntitySentimentResponse>> {
        self.analyze_entity_sentiment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_syntax_opt(&self, req: &super::language_service::AnalyzeSyntaxRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::language_service::AnalyzeSyntaxResponse> {
        self.client.unary_call(&METHOD_LANGUAGE_SERVICE_ANALYZE_SYNTAX, req, opt)
    }

    pub fn analyze_syntax(&self, req: &super::language_service::AnalyzeSyntaxRequest) -> ::grpcio::Result<super::language_service::AnalyzeSyntaxResponse> {
        self.analyze_syntax_opt(req, ::grpcio::CallOption::default())
    }

    pub fn analyze_syntax_async_opt(&self, req: &super::language_service::AnalyzeSyntaxRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeSyntaxResponse>> {
        self.client.unary_call_async(&METHOD_LANGUAGE_SERVICE_ANALYZE_SYNTAX, req, opt)
    }

    pub fn analyze_syntax_async(&self, req: &super::language_service::AnalyzeSyntaxRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnalyzeSyntaxResponse>> {
        self.analyze_syntax_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn classify_text_opt(&self, req: &super::language_service::ClassifyTextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::language_service::ClassifyTextResponse> {
        self.client.unary_call(&METHOD_LANGUAGE_SERVICE_CLASSIFY_TEXT, req, opt)
    }

    pub fn classify_text(&self, req: &super::language_service::ClassifyTextRequest) -> ::grpcio::Result<super::language_service::ClassifyTextResponse> {
        self.classify_text_opt(req, ::grpcio::CallOption::default())
    }

    pub fn classify_text_async_opt(&self, req: &super::language_service::ClassifyTextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::ClassifyTextResponse>> {
        self.client.unary_call_async(&METHOD_LANGUAGE_SERVICE_CLASSIFY_TEXT, req, opt)
    }

    pub fn classify_text_async(&self, req: &super::language_service::ClassifyTextRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::ClassifyTextResponse>> {
        self.classify_text_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn annotate_text_opt(&self, req: &super::language_service::AnnotateTextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::language_service::AnnotateTextResponse> {
        self.client.unary_call(&METHOD_LANGUAGE_SERVICE_ANNOTATE_TEXT, req, opt)
    }

    pub fn annotate_text(&self, req: &super::language_service::AnnotateTextRequest) -> ::grpcio::Result<super::language_service::AnnotateTextResponse> {
        self.annotate_text_opt(req, ::grpcio::CallOption::default())
    }

    pub fn annotate_text_async_opt(&self, req: &super::language_service::AnnotateTextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnnotateTextResponse>> {
        self.client.unary_call_async(&METHOD_LANGUAGE_SERVICE_ANNOTATE_TEXT, req, opt)
    }

    pub fn annotate_text_async(&self, req: &super::language_service::AnnotateTextRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::language_service::AnnotateTextResponse>> {
        self.annotate_text_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait LanguageService {
    fn analyze_sentiment(&mut self, ctx: ::grpcio::RpcContext, req: super::language_service::AnalyzeSentimentRequest, sink: ::grpcio::UnarySink<super::language_service::AnalyzeSentimentResponse>);
    fn analyze_entities(&mut self, ctx: ::grpcio::RpcContext, req: super::language_service::AnalyzeEntitiesRequest, sink: ::grpcio::UnarySink<super::language_service::AnalyzeEntitiesResponse>);
    fn analyze_entity_sentiment(&mut self, ctx: ::grpcio::RpcContext, req: super::language_service::AnalyzeEntitySentimentRequest, sink: ::grpcio::UnarySink<super::language_service::AnalyzeEntitySentimentResponse>);
    fn analyze_syntax(&mut self, ctx: ::grpcio::RpcContext, req: super::language_service::AnalyzeSyntaxRequest, sink: ::grpcio::UnarySink<super::language_service::AnalyzeSyntaxResponse>);
    fn classify_text(&mut self, ctx: ::grpcio::RpcContext, req: super::language_service::ClassifyTextRequest, sink: ::grpcio::UnarySink<super::language_service::ClassifyTextResponse>);
    fn annotate_text(&mut self, ctx: ::grpcio::RpcContext, req: super::language_service::AnnotateTextRequest, sink: ::grpcio::UnarySink<super::language_service::AnnotateTextResponse>);
}

pub fn create_language_service<S: LanguageService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LANGUAGE_SERVICE_ANALYZE_SENTIMENT, move |ctx, req, resp| {
        instance.analyze_sentiment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITIES, move |ctx, req, resp| {
        instance.analyze_entities(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LANGUAGE_SERVICE_ANALYZE_ENTITY_SENTIMENT, move |ctx, req, resp| {
        instance.analyze_entity_sentiment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LANGUAGE_SERVICE_ANALYZE_SYNTAX, move |ctx, req, resp| {
        instance.analyze_syntax(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LANGUAGE_SERVICE_CLASSIFY_TEXT, move |ctx, req, resp| {
        instance.classify_text(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LANGUAGE_SERVICE_ANNOTATE_TEXT, move |ctx, req, resp| {
        instance.annotate_text(ctx, req, resp)
    });
    builder.build()
}
