use crate::{
    define_request,
    models::{spot::UserDataStream, Product},
};
use reqwest::Method;

define_request! {
    Name => StartUserDataStream;
    Product => Product::PortfolioMargin;
    Method => Method::POST;
    Endpoint => "/papi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => UserDataStream;
}

define_request! {
    Name => KeepaliveUserDataStream;
    Product => Product::PortfolioMargin;
    Method => Method::PUT;
    Endpoint => "/papi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}

define_request! {
    Name => CloseUserDataStream;
    Product => Product::PortfolioMargin;
    Method => Method::DELETE;
    Endpoint => "/papi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}
