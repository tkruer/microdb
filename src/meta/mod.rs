use crate::buffer::InputBuffer;

pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub fn do_meta_command(input_buffer: &InputBuffer, table: &Table) -> MetaCommandResult {
    match input_buffer.buffer.as_str() {
        ".exit" => {
            println!("Exiting...");
            table.close();
            std::process::exit(0);
        }
        ".btree" => {
            print_tree("pager_placeholder", 0, 0);
            MetaCommandResult::Success
        }
        ".constants" => {
            print_constants();
            MetaCommandResult::Success
        }
        _ => MetaCommandResult::UnrecognizedCommand,
    }
}
