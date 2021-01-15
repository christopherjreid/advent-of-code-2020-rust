use std::collections::HashMap;

use crate::days_of_advent::common::io;

mod bag_rule_map;
mod bag_rules;

pub fn run() {
    let puzzle_input = io::load_input_from_file("day07");

    let puzzle_input = puzzle_input.unwrap();

    let bag_rule_map_deserializer = bag_rule_map::BagRuleMapDeserializer::new();
    let total =
        calc_num_bags_can_contain_bag(&puzzle_input, &bag_rule_map_deserializer, "shiny gold");

    let total_bags_within =
        calc_num_bags_inside_bag(&puzzle_input, &bag_rule_map_deserializer, "shiny gold");

    let content = format!(
        "\
        Total number of options are {}\n\
        Total number of contained bags are {}",
        total, total_bags_within
    );

    let report = io::format_day_report(
        7,
        "Handy Haversacks",
        "Count bags that can contain a shiny gold bag",
        &content,
    );

    println!("{}", report);
}

pub fn calc_num_bags_can_contain_bag(
    serialized_bag_rules: &str,
    bag_rule_deserializer: &bag_rule_map::BagRuleMapDeserializer,
    bag_type: &str,
) -> usize {
    let nodes: bag_rule_map::BagRuleMap = bag_rule_deserializer.deserialize(&serialized_bag_rules);

    let mut cached_rule_distance: HashMap<String, Option<usize>> = HashMap::new();

    let mut total = 0;

    for (id, _) in &nodes {
        let res = dist_from_node(id, bag_type, &nodes, &mut cached_rule_distance);
        if res.is_some() && res.unwrap() != 0 {
            total += 1;
        }
    }

    return total;
}

pub fn calc_num_bags_inside_bag(
    serialized_bag_rules: &str,
    bag_rule_deserializer: &bag_rule_map::BagRuleMapDeserializer,
    bag_type: &str,
) -> usize {
    let nodes: bag_rule_map::BagRuleMap = bag_rule_deserializer.deserialize(&serialized_bag_rules);

    num_bags_within_bag(bag_type, 1, &nodes) - 1
}

pub fn dist_from_node(
    source_id: &str,
    target_id: &str,
    map: &bag_rule_map::BagRuleMap,
    cached_map: &mut HashMap<String, Option<usize>>,
) -> Option<usize> {
    if cached_map.contains_key(source_id) {
        return cached_map.get(source_id).unwrap().clone();
    } else if source_id == target_id {
        cached_map.insert(String::from(target_id), Some(0));
        return Some(0);
    } else if map.get(source_id).unwrap().contents.is_empty() {
        return None;
    } else {
        let mut best_result = None;
        for (_, child_node_name) in &map.get(source_id).unwrap().contents {
            let child_node = map.get(child_node_name).unwrap();
            let res = dist_from_node(child_node.id, target_id, map, cached_map);
            cached_map.insert(String::from(child_node.id), res.clone());
            if res == None {
            } else if res.is_some() {
                let num = res.unwrap();
                if best_result.is_none() || best_result.clone().unwrap() > num {
                    best_result = Some(num + 1);
                }
            }
        }
        return best_result;
    }
}

pub fn num_bags_within_bag(source_id: &str, num: usize, map: &bag_rule_map::BagRuleMap) -> usize {
    let start_rule = map.get(source_id).unwrap();
    let mut total = num;
    for (num_child, id) in &start_rule.contents {
        total += num * num_bags_within_bag(id, *num_child, &map);
    }

    return total;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn acceptance_criteria() {
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

        let bag_rule_map_deserializer = bag_rule_map::BagRuleMapDeserializer::new();

        let total = calc_num_bags_can_contain_bag(&input, &bag_rule_map_deserializer, "shiny gold");
        assert_eq!(total, 4);
    }

    #[test]
    fn acceptance_criteria_b() {
        let input = "\
        shiny gold bags contain 2 dark red bags.\n\
        dark red bags contain 2 dark orange bags.\n\
        dark orange bags contain 2 dark yellow bags.\n\
        dark yellow bags contain 2 dark green bags.\n\
        dark green bags contain 2 dark blue bags.\n\
        dark blue bags contain 2 dark violet bags.\n\
        dark violet bags contain no other bags.";

        let bag_rule_map_deserializer = bag_rule_map::BagRuleMapDeserializer::new();

        let total = calc_num_bags_inside_bag(&input, &bag_rule_map_deserializer, "shiny gold");
        assert_eq!(total, 126);
    }
}
