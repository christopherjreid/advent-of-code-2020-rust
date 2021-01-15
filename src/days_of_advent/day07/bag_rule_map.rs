use super::bag_rules::*;

pub type BagRuleMap<'a> = std::collections::HashMap<&'a str, BagRule<'a>>;

pub struct BagRuleMapDeserializer {
    bag_deserializer: BagRuleDeserializer,
}

impl BagRuleMapDeserializer {
    pub fn new() -> Self {
        BagRuleMapDeserializer {
            bag_deserializer: BagRuleDeserializer::new(),
        }
    }

    pub fn deserialize<'a>(&self, data: &'a str) -> BagRuleMap<'a> {
        data.lines()
            .map(|l| {
                let bag_rule = self.bag_deserializer.deserialize(&l);
                (bag_rule.id, bag_rule)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_map() {
        let input = "\
        light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
        bright white bags contain 1 shiny gold bag.\n\
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
        faded blue bags contain no other bags.\n\
        dotted black bags contain no other bags.";

        let bag_rule_map_deserializer = BagRuleMapDeserializer::new();

        let bag_rules = bag_rule_map_deserializer.deserialize(&input);

        assert_eq!(9, bag_rules.len());
    }
}
