#[derive(Debug,PartialEq)]
pub struct BagRule<'a> {
    pub id: &'a str,
    pub contents: Vec<(usize, &'a str)>,
}

pub struct BagRuleDeserializer {
    line_regex: regex::Regex,
    single_bag_regex: regex::Regex,
}

impl BagRuleDeserializer {
    pub fn new() -> Self {
        BagRuleDeserializer {
            line_regex: regex::Regex::new(
                r"(?P<container_bag_type>.*) bags contain (?P<contained_bag_types>.*).",
            )
            .unwrap(),
            single_bag_regex: regex::Regex::new(
                r"(?P<contained_bag_count>\d+) (?P<contained_bag_type>.*) bags?",
            )
            .unwrap(),
        }
    }

    pub fn deserialize<'a>(&self, data: &'a str) -> BagRule<'a> {
        let captures = self.line_regex.captures(data).unwrap();
        let id = captures.name("container_bag_type").unwrap().as_str();
        let captures = captures.name("contained_bag_types");
        let matched_text = captures.unwrap().as_str();
        match matched_text {
            "no other bags" => BagRule {
                id,
                contents: vec![],
            },
            _ => BagRule {
                id,
                contents: matched_text
                    .split(",")
                    .map(|s| self.deserialize_one_contained_bag(s.trim()))
                    .collect::<Vec<(usize, &'a str)>>(),
            },
        }
    }

    fn deserialize_one_contained_bag<'a>(&self, string: &'a str) -> (usize, &'a str) {
        let captures = self.single_bag_regex.captures(string).unwrap();
        (
            captures
                .name("contained_bag_count")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            captures.name("contained_bag_type").unwrap().as_str(),
        )
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_bag_rules_from_str() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

        let solution = BagRule {
            id: "light red",
            contents: vec![
                (
                    1,
                    "bright white"
                ),
                (
                    2,
                    "muted yellow"
                    
                ),
            ],
        };
        let bag_rule_deserializer = BagRuleDeserializer::new();
        let result = bag_rule_deserializer.deserialize(&input);

        assert_eq!(result, solution);
    }

    #[test]
    fn deserialize_single_rule_no_bag() {
        let input = "light red bags contain no other bags.";

        let solution = BagRule {
            id: "light red",
            contents: vec![],
        };
        let bag_rule_deserializer = BagRuleDeserializer::new();
        let result = bag_rule_deserializer.deserialize(&input);

        assert_eq!(result, solution);
    }

}