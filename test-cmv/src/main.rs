fn main() {
  let container = compact_mixed_vectors::Container::new(3, 3, 2).unwrap();
  println!("container: {container:#?}");

  {
    let chars = container.get_chars_slice();
    println!("chars: {chars:#?}");
    for c in chars.iter_mut() {
      *c = 67;
    }
    println!("chars: {chars:#?}");
  }

  {
    let lines = container.get_lines_slice();
    println!("lines: {lines:#?}");
    for line in lines.iter_mut() {
      *line = 32;
    }
    println!("lines: {lines:#?}");
  }
}
