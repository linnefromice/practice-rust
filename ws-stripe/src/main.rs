use stripe::Client;

const SECRET_KEY: &str = "";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(SECRET_KEY);
    // let products = products(&client).await?;
    // for product in products {
    //     println!("Product: {:?}", &product);
    // }

    let params = CreatePreviewInvoice {
        customer: Some("cus_QX4lljL0zmfFZq".to_string()),
        subscription: None,
        subscription_details: None,
        schedule: Some("sub_sched_1PpN8GP7LyQtEQaSNF1RgfCg".to_string()),
    };
    let invoice = invoice_preview_create(&client, params).await?;
    println!("Invoice: {:?}", &invoice);

    Ok(())
}

async fn products(client: &Client) -> anyhow::Result<Vec<stripe::Product>> {
    let res = stripe::Product::list(client, &Default::default()).await?;
    Ok(res.data)
}

#[derive(Clone, Debug, serde::Serialize, Default)]
pub struct CreatePreviewInvoiceSubscriptionDetailsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
}
#[derive(Clone, Debug, serde::Serialize, Default)]
pub struct CreatePreviewInvoiceSubscriptionDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CreatePreviewInvoiceSubscriptionDetailsItem>>,
}
#[derive(Clone, Debug, serde::Serialize, Default)]
pub struct CreatePreviewInvoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_details: Option<CreatePreviewInvoiceSubscriptionDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<String>,
}

// Response for POST /invoices/create_preview
// note: The existing Invoice object fails with the following error, so you have to create your own Invoice object.
//   Caused by id: invalid `InvoiceId`, expected id to start with "in_"
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
struct PreviewInvoiceResponse {
    amount_due: Option<i64>,
    created: Option<i64>,
    currency: Option<String>,
    period_end: Option<i64>,
    period_start: Option<i64>,
    total: Option<i64>,
    lines: Option<stripe::List<stripe::InvoiceLineItem>>,
}

async fn invoice_preview_create(
    client: &Client,
    params: CreatePreviewInvoice,
) -> anyhow::Result<PreviewInvoiceResponse> {
    let url = "/invoices/create_preview";
    let res: PreviewInvoiceResponse = client.post_form(url, &params).await?;
    Ok(res)
}
