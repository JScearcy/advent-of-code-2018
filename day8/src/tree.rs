use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
    child_count: usize,
    metadata_count: usize,
}

pub fn create_tree(tree_items: &str) -> (usize, usize) {
    let mut tree_list: VecDeque<&str> = tree_items.split(' ').collect();
    let mut parent = Node {
        metadata: vec![],
        children: vec![],
        child_count: usize::max_value(),
        metadata_count: usize::max_value(),
    };
    let count = create_element(&mut tree_list, &mut parent);

    let root_value = find_root_value(&parent.children[0]);

    (count, root_value)
}

fn find_root_value(tree: &Node) -> usize {
    if tree.children.len() > 0 {
        let mut total = 0;
        for idx in &tree.metadata {
            let vec_idx = *idx - 1;
            if vec_idx < tree.children.len() {
                total += find_root_value(&tree.children[vec_idx]);
            }
        }
        return total;
    }
    tree.metadata
        .iter()
        .fold(0usize, |total, curr| total + curr)
}

fn create_element(tree_items: &mut VecDeque<&str>, parent: &mut Node) -> usize {
    let mut local_total = 0;
    if tree_items.len() > 0 {
        let curr_children_count = usize::from_str_radix(tree_items.pop_front().unwrap(), 10)
            .expect("children count wasn't a number?");
        let curr_metadata_count = usize::from_str_radix(tree_items.pop_front().unwrap(), 10)
            .expect("metadata count wasn't a number?");
        let mut new_child = Node {
            metadata_count: curr_metadata_count,
            child_count: curr_children_count,
            children: vec![],
            metadata: vec![],
        };
        if curr_children_count > 0 {
            for _ in 0..curr_children_count {
                let element_metadata = create_element(tree_items, &mut new_child);
                local_total += element_metadata;
            }
        }
        let mut new_metadata = vec![];
        for _ in 0..curr_metadata_count {
            let metadata_item = usize::from_str_radix(tree_items.pop_front().unwrap(), 10)
                .expect("metadata wasn't a number?");
            new_metadata.push(metadata_item);
            local_total += metadata_item;
        }
        add_metadata(&mut new_child, new_metadata);
        parent.children.push(new_child);
    } else {
        return 0;
    }

    local_total
}

fn add_metadata(tree: &mut Node, new_metadata: Vec<usize>) {
    for metadata_item in new_metadata {
        tree.metadata.push(metadata_item);
    }
}
