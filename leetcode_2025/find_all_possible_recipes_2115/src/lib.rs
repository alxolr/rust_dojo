use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

type Graph<'a, 'b> = HashMap<&'a String, Vec<&'b String>>;

// I need to make a graph of dependencies between ingredients
impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let (direct_graph, reverse_graph): (Graph, Graph) = recipes
            .iter()
            .zip(ingredients.iter())
            .map(|(recipe, ingredients)| (recipe, ingredients.iter().collect()))
            .chain(supplies.iter().map(|sup| (sup, vec![])))
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut direct, mut reverse), (recipe, ingredients)| {
                    direct.entry(recipe).or_insert_with(|| ingredients.clone());

                    for ingredient in ingredients {
                        reverse.entry(ingredient).or_default().push(recipe);
                    }

                    (direct, reverse)
                },
            );

        let recipies_order = topo_sort(&direct_graph, &reverse_graph);

        recipes
            .iter()
            .filter(|recipe| recipies_order.contains(*recipe))
            .map(|recipe| recipe.to_string())
            .collect()
    }
}

fn topo_sort(direct_graph: &Graph, reverse_graph: &Graph) -> HashSet<String> {
    let mut sorted = HashSet::new();

    let mut depedency_count = direct_graph
        .iter()
        .map(|(key, val)| (*key, val.len()))
        .collect::<HashMap<_, _>>();

    let mut queue = depedency_count
        .iter()
        .filter_map(|(key, count)| if *count == 0 { Some(*key) } else { None })
        .collect::<VecDeque<&String>>();

    while let Some(key) = queue.pop_front() {
        sorted.insert(key.clone());

        if let Some(children) = reverse_graph.get(key) {
            for child in children {
                if let Some(count) = depedency_count.get_mut(child) {
                    *count -= 1;

                    if *count == 0 {
                        queue.push_back(child);
                    }
                }
            }
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    vec!["bread".to_string()],
                    vec![vec!["yeast".to_string(), "flour".to_string()]],
                    vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()],
                ),
                vec!["bread".to_string()],
            ),
            (
                (
                    vec!["bread".to_string(), "sandwich".to_string()],
                    vec![
                        vec!["yeast".to_string(), "flour".to_string()],
                        vec!["bread".to_string(), "meat".to_string()],
                    ],
                    vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
                ),
                vec!["bread".to_string(), "sandwich".to_string()],
            ),
            (
                (
                    vec![
                        "bread".to_string(),
                        "sandwich".to_string(),
                        "burger".to_string(),
                        "cake".to_string(),
                    ],
                    vec![
                        vec!["yeast".to_string(), "flour".to_string()],
                        vec!["bread".to_string(), "meat".to_string()],
                        vec![
                            "sandwich".to_string(),
                            "meat".to_string(),
                            "bread".to_string(),
                        ],
                        vec!["armadilo".to_string()],
                    ],
                    vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()],
                ),
                vec![
                    "bread".to_string(),
                    "sandwich".to_string(),
                    "burger".to_string(),
                ],
            ),
        ];

        scenarios.into_iter().enumerate().for_each(
            |(idx, ((recipes, ingredients, supplies), expected))| {
                let result = Solution::find_all_recipes(recipes, ingredients, supplies);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            },
        );
    }
}
