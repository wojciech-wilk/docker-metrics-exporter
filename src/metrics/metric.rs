const METRIC_NAME_PREFIX: &str = "dme_";

pub struct Label {
    pub name: String,
    pub value: String,
}

pub struct Metric {
    pub name: String,
    pub value: String,
    pub labels: Vec<Label>,
}

impl Metric {
    pub fn into_prometheus_string(&self) -> String {
        let labels = self.labels.iter()
            .map(|label| [label.name.as_str(), "=", label.value.as_str()].concat())
            .collect::<Vec<String>>()
            .join(",")
            ;

        [
            METRIC_NAME_PREFIX, self.name.as_str(),
            "{", labels.as_str(), "}",
            " ",
            self.value.to_string().as_str()
        ].concat()
    }
}


