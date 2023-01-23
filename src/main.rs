mod to_do;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let pending_item = to_do_factory("pending", "laundry").unwrap();
    let done_item = to_do_factory("done", "shopping").unwrap();
    print_item(pending_item);
    print_item(done_item);
}

fn print_item(item: ItemTypes) {
    match item {
        ItemTypes::Pending(item) => println!(
            "it's a pending item with the title: {}",
            item.super_struct.title
        ),

        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}
