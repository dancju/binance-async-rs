use crate::{define_request, models::Product::PortfolioMargin};
use reqwest::Method;

define_request! {
    Name => StartUserDataStream;
    Product => PortfolioMargin;
    Method => Method::POST;
    Endpoint => "/papi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => { pub listen_key: String };
}

define_request! {
    Name => KeepaliveUserDataStream;
    Product => PortfolioMargin;
    Method => Method::PUT;
    Endpoint => "/papi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}

define_request! {
    Name => CloseUserDataStream;
    Product => PortfolioMargin;
    Method => Method::DELETE;
    Endpoint => "/papi/v1/listenKey";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => {};
}
