use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::Client;
use color_eyre::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Debug, Clone)]
pub struct Instance {
    pub instance_id: String,
    pub tags: Vec<Tag>,
}

impl Instance {
    pub fn display(&self) -> String {
        let tags: Vec<String> = self
            .tags
            .iter()
            .map(|t| format!("{}: {}", t.key, t.value))
            .collect();
        format!("{} {}", self.instance_id, tags.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    key: String,
    value: String,
}

impl Tag {
    pub fn new(key: String, value: String) -> Self {
        Tag { key, value }
    }
}

pub async fn get_instances() -> Result<Vec<Instance>> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let filter = Filter::builder()
        .name("instance-state-name")
        .values("running")
        .name("platform")
        .values("windows")
        .build();

    let resp = client
        .describe_instances()
        .set_filters(Some(vec![filter]))
        .send()
        .await?;

    let mut instances = vec![];

    for reservation in resp.reservations() {
        for instance in reservation.instances() {
            let instance_id = instance.instance_id();
            // Skip instances with no ID
            if instance_id.is_none() {
                continue;
            }

            let mut tags = vec![];

            for tag in instance.tags().iter() {
                if let Some(key) = tag.key() {
                    if let Some(value) = tag.value() {
                        tags.push(Tag::new(key.to_string(), value.to_string()))
                    }
                }
            }

            tags.sort_by(|a, b| {
                if &a.key == "Name" && &b.key != "Name" {
                    std::cmp::Ordering::Less
                } else if &a.key != "Name" && &b.key == "Name" {
                    std::cmp::Ordering::Greater
                } else {
                    a.key.cmp(&b.key)
                }
            });

            let instance = Instance {
                instance_id: instance_id.unwrap().to_owned(),
                tags,
            };
            instances.push(instance);
        }
    }

    Ok(instances)
}

pub fn fuzzy_search_instances<'a>(
    instances: &'a [Instance],
    search_term: &'a str,
) -> Vec<&'a Instance> {
    let matcher = SkimMatcherV2::default();
    let mut matched_instances = Vec::new();

    for instance in instances {
        // Check if the instance_id matches the search term.
        if matcher
            .fuzzy_match(&instance.instance_id, search_term)
            .is_some()
        {
            matched_instances.push(instance);
            continue; // Skip to the next instance to avoid duplicate entries.
        }

        // Check if any of the tags match the search term.
        for tag in &instance.tags {
            if matcher.fuzzy_match(&tag.key, search_term).is_some()
                || matcher.fuzzy_match(&tag.value, search_term).is_some()
            {
                matched_instances.push(instance);
                break; // Found a match, no need to check more tags.
            }
        }
    }

    matched_instances
}
