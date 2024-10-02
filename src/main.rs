use aquapack::linked_list::LinkedList;
use colored::Colorize;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // Test: Adding elements to the list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.len(), 3, "{}", "Test Case 1 (Adding) Failed!".red().bold());

    // Test: Removing elements from the list
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.len(), 1, "{}", "Test Case 2 (Removing) Failed!".red().bold());

    // Test: Checking the remaining elements
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0, "{}", "Test Case 3 (Checking) Failed!".red().bold());

    // Test: Adding elements again to the list
    list.push_back(4);
    list.push_back(5);
    assert_eq!(list.len(), 2, "{}", "Test Case 4 (Adding Again) Failed!".red().bold());

    // Test: Clearing the list
    list.clear();
    assert_eq!(list.len(), 0, "{}", "Test Case 5.1 (Clearing) Failed!".red().bold());
    assert_eq!(list.pop_front(), None, "{}", "Test Case 5.2 (Clearing) Failed!".red().bold());

    println!("{}", "All Tests Passed!".bright_purple().bold());
}