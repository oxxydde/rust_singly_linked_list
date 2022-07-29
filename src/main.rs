mod node;
mod sll;

fn main() {
    let mut sl_obj = sll::SLL {
        ..Default::default()
    };
    sl_obj.add_last(7);
    sl_obj.add_last(9);
    sl_obj.add_last(13);
    sl_obj.print_out();
}