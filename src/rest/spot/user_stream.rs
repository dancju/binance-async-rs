use crate::models::{spot::UserDataStream, Product};
use reqwest::Method;

crate::define_request! {
    Name => StartUserDataStream;
    Product => Product::Spot;
    Method => Method::POST;
    Endpoint => "/api/v3/userDataStream";
    Keyed => true;
    Signed => false;
    Request => {};
    Response => UserDataStream;
}

crate::define_request! {
    Name => KeepaliveUserDataStream;
    Product => Product::Spot;
    Method => Method::PUT;
    Endpoint => "/api/v3/userDataStream";
    Keyed => true;
    Signed => false;
    Request => { pub listen_key: String };
    Response => {};
}

crate::define_request! {
    Name => CloseUserDataStream;
    Product => Product::Spot;
    Method => Method::DELETE;
    Endpoint => "/api/v3/userDataStream";
    Keyed => true;
    Signed => false;
    Request => { pub listen_key: String };
    Response => {};
}
