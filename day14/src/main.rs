mod test_kitchen;

use crate::test_kitchen::TestKitchen;

fn main() {
    let mut test_kitchen = TestKitchen::new(637061);
    test_kitchen.find_scores_after_index();
    test_kitchen.find_instance_of_index();
}
