use crate::models::Product::Spot;
use reqwest::Method;

crate::define_request! {
    Name => StartUserDataStream;
    Product => Spot;
    Method => Method::POST;
    Endpoint => "/api/v3/userDataStream";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => { pub listen_key: String };
}

crate::define_request! {
    Name => KeepaliveUserDataStream;
    Product => Spot;
    Method => Method::PUT;
    Endpoint => "/api/v3/userDataStream";
    Keyed => true;
    Signed => false;
    Request => { pub listen_key: String };
    Response => {};
}

crate::define_request! {
    Name => CloseUserDataStream;
    Product => Spot;
    Method => Method::DELETE;
    Endpoint => "/api/v3/userDataStream";
    Keyed => true;
    Signed => false;
    Request => { pub listen_key: String };
    Response => {};
}
