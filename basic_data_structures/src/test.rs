use super::lib::linked_list::List;

#[test]
fn linked_list() {
    let mut list: List<u32> = List::new();

    list.push(4);
    list.push(9);
    list.push(7);
    list.push(4);

    assert_eq!(list.size, 4);
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.size, 3);

    list.push(2);
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), Some(9));
    assert_eq!(list.size, 1);

    list.pop();
    list.pop();
    assert_eq!(list.pop(), None);
    assert_eq!(list.size, 0);
}