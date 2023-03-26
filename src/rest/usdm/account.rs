use crate::models::Product;
use reqwest::Method;

crate::define_request! {
    Name => GetCurrentPositionMode;
    Product => Product::UsdMFutures;
    Endpoint => "/fapi/v1/positionSide/dual";
    Method => Method::GET;
    Signed => true;
    Request => {};
    Response => {
        pub dual_side_position: bool,
    };
}
