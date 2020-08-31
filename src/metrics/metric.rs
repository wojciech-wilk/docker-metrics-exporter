const METRIC_NAME_PREFIX: &str = "dme_";

pub struct Label {
    pub name: String,
    pub value: String,
}

pub struct Metric {
    pub name: String,
    pub value: String,
    pub labels: Option<Vec<Label>>,
}

impl Label {
    pub fn new(name: String, value: String) -> Label {
        Label {
            name,
            value,
        }
    }
}

impl Metric {
    pub fn new(name: String, value: String, labels: Option<Vec<Label>>) -> Metric {
        Metric {
            name,
            value,
            labels,
        }
    }

    pub fn into_prometheus_string(self) -> String {
        let labels = match self.labels {
            Some(l) =>
                ["{",
                    l.iter()
                        .map(|label| [label.name.as_str(), "=\"", label.value.as_str(), "\""].concat())
                        .collect::<Vec<String>>()
                        .join(",").as_str(),
                    "}"
                ].concat()
            ,
            None => "".to_string(),
        };

        [
            METRIC_NAME_PREFIX, self.name.as_str(),
            labels.as_str(),
            " ",
            self.value.to_string().as_str()
        ].concat()
    }
}

#[cfg(test)]
mod test {
    use crate::metrics::metric::*;

    #[test]
    fn serialize_without_labels() {
        // given
        let metric = Metric::new("foo".to_string(), "bar".to_string(), None);

        // when
        let serialized = metric.into_prometheus_string();

        // then
        assert_eq!(serialized, METRIC_NAME_PREFIX.to_string() + "foo bar");
    }

    #[test]
    fn serialize_with_labels() {
        // given
        let labels = vec![
            Label::new("label1".to_string(), "value1".to_string()),
            Label::new("label2".to_string(), "value2".to_string()),
        ];

        let metric = Metric::new("foo".to_string(), "bar".to_string(), Some(labels));

        // when
        let serialized = metric.into_prometheus_string();

        // then
        assert_eq!(serialized, METRIC_NAME_PREFIX.to_string() + "foo{label1=\"value1\",label2=\"value2\"} bar");
    }
}
