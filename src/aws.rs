use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::model::Filter;
use aws_sdk_ec2::{Client, Error};
use color_eyre::Result;

struct Instance {
    instance_id: String,
    name: Option<String>,
}

pub async fn get_instances() -> Result<()> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let filter = Filter::builder()
        .name("instance-state-name")
        .values("running")
        .build();

    let resp = client
        .describe_instances()
        .set_filters(Some(vec![filter]))
        .send()
        .await?;

    for reservation in resp.reservations() {
        for instance in reservation.instances() {
            println!(
                "Instance ID: {}",
                instance.instance_id().unwrap_or_default()
            );
            let name_tag_value = instance
                .tags()
                .iter()
                .find(|tag| tag.key().map_or(false, |k| k == "Name"))
                .and_then(|tag| tag.value());
        }
    }

    todo!()
}
