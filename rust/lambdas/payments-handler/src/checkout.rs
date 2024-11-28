use bitie_types::payments::{PaymentProcessorSecrets, QuestionDonation};
use tracing::{error, info, warn};

use stripe::{
    CheckoutSession, CheckoutSessionCustomerCreation, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, IdOrCreate, Price, Product,
};

/// Returns the URL of a Stripe checkout session for the given order details.
/// Logs errors and returns None if the URL cannot be obtained from Stripe.
pub(crate) async fn get_checkout_url(
    order_details: QuestionDonation,
    secrets: PaymentProcessorSecrets,
) -> Option<String> {
    // EXTRACT AND VALIDATE REQUIRED DETAILS

    // this builds an attribution message from whatever elements were supplied in the structure
    let attribution = match order_details.contributor {
        Some(v) => v.to_string(),
        None => "".to_string(),
    };

    // email is required
    let email = order_details.contact_email.trim().to_lowercase();
    let email = if email.is_empty() {
        warn!("Missing email in the order details");
        return None;
    } else {
        Some(email.as_str())
    };

    // values between 1 and 10
    let qty = order_details.qty;
    if !(1..11).contains(&qty) {
        warn!("Invalid quantity: {}", qty);
        return None;
    }

    // cancel URL is required
    let cancel_url = {
        let cancel_url = order_details.cancel_url.trim().to_string();
        if cancel_url.is_empty() {
            warn!("Missing cancel URL in the order details");
            return None;
        } else {
            cancel_url
        }
    };

    // success URL is required
    let success_url = {
        let success_url = order_details.success_url.trim().to_string();
        if success_url.is_empty() {
            warn!("Missing success URL in the order details");
            return None;
        } else {
            [success_url, "?session_id={CHECKOUT_SESSION_ID}".to_string()].join("")
        }
    };

    let client = Client::new(secrets.secret);

    // Build the product description specifically for this order
    let description = {
        // submitter details for attribution are optional
        let attribution = if attribution.is_empty() {
            "".to_string()
        } else {
            format!(" from {attribution}")
        };
        // topics are optional
        let topics = match order_details.topics {
            Some(v) => v.trim().to_string(),
            None => "".to_string(),
        };
        if topics.is_empty() {
            format!("Gift of bite-sized pieces of knowledge{}", attribution)
        } else {
            format!(
                "Gift of bite-sized pieces of knowledge about {}{}",
                topics.as_str(),
                attribution
            )
        }
    };

    // create a new product for this order
    let product_id = {
        //
        let params = CreateProduct::new(&description);
        match Product::create(&client, params).await {
            Ok(v) => {
                info!("Product created: {:?}", v);
                v.id.to_string()
            }
            Err(e) => {
                error!("Failed to create product: {}", e);
                return None;
            }
        }
    };

    // add a price for it in USD
    let price_id = {
        let mut params = CreatePrice::new(Currency::USD);
        params.product = Some(IdOrCreate::Id(&product_id));
        params.unit_amount = Some(5000);
        params.currency = Currency::USD;
        params.expand = &["product"];
        match Price::create(&client, params).await {
            Ok(v) => {
                info!("Price created: {:?}", v);
                v.id.to_string()
            }
            Err(e) => {
                error!("Failed to create price: {}", e);
                return None;
            }
        }
    };

    // create a checkout session for this product / price
    let checkout_session = {
        let mut params = CreateCheckoutSession::new();
        params.cancel_url = Some(&cancel_url);
        params.success_url = Some(&success_url);
        params.customer_creation = Some(CheckoutSessionCustomerCreation::IfRequired);
        params.customer_email = email;

        // params.customer = Some(customer.id);
        params.mode = Some(CheckoutSessionMode::Payment);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(qty as u64),
            price: Some(price_id),
            adjustable_quantity: Some({
                stripe::CreateCheckoutSessionLineItemsAdjustableQuantity {
                    enabled: true,
                    maximum: Some(10),
                    minimum: Some(1),
                }
            }),
            ..Default::default()
        }]);
        params.expand = &["line_items", "line_items.data.price.product"];

        match CheckoutSession::create(&client, params).await {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to create checkout session: {}", e);
                return None;
            }
        }
    };

    info!(
        "Checkout session created {:?}, url: {:?}",
        checkout_session, checkout_session.url,
    );

    info!("Checkout session URL {:?}", checkout_session.url,);

    checkout_session.url
}
