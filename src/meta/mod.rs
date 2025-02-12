use crate::buffer::*;
use crate::constants::{
    COMMON_NODE_HEADER_SIZE, LEAF_NODE_CELL_SIZE, LEAF_NODE_HEADER_SIZE, LEAF_NODE_MAX_CELLS,
    LEAF_NODE_SPACE_FOR_CELLS, ROW_SIZE,
};
use crate::node::NodeType;
use crate::pager::Pager;
use crate::table::*;

pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub fn do_meta_command(input_buffer: &InputBuffer, table: &mut Table) -> MetaCommandResult {
    match input_buffer.buffer.as_str() {
        ".exit" => {
            println!("Exiting...");
            table.close();
            std::process::exit(0);
        }
        ".btree" => {
            println!("(Printing tree – not fully implemented)");
            // TODO: FINISH OUT OUR BTREE
            MetaCommandResult::Success
        }
        ".constants" => {
            print_constants();
            MetaCommandResult::Success
        }
        _ => MetaCommandResult::UnrecognizedCommand,
    }
}

/// Recursively prints the tree structure, starting at the given page number.
pub fn print_tree(pager: &mut Pager, page_num: u32, indentation_level: u32) {
    // First, extract all needed data from the node.
    let info: NodeInfo = {
        let node = pager.get_page(page_num);
        match node.get_type() {
            NodeType::Leaf => {
                let num_cells = node.leaf_num_cells();
                let mut keys = Vec::with_capacity(num_cells as usize);
                for i in 0..num_cells {
                    keys.push(node.leaf_key(i as usize));
                }
                NodeInfo::Leaf(LeafInfo { num_cells, keys })
            }
            NodeType::Internal => {
                let num_keys = node.internal_num_keys();
                let mut keys = Vec::with_capacity(num_keys as usize);
                let mut children = Vec::with_capacity(num_keys as usize);
                for i in 0..num_keys {
                    children.push(node.internal_child(i as usize));
                    keys.push(node.internal_key(i as usize));
                }
                let right_child = node.internal_right_child();
                NodeInfo::Internal(InternalInfo {
                    num_keys,
                    keys,
                    children,
                    right_child,
                })
            }
        }
    };

    // Drop the borrow on the node.
    indent(indentation_level);
    match info {
        NodeInfo::Leaf(info) => {
            println!("- leaf (size {})", info.num_cells);
            for key in info.keys {
                indent(indentation_level + 1);
                println!("- key {}", key);
            }
        }
        NodeInfo::Internal(info) => {
            println!("- internal (size {})", info.num_keys);
            if info.num_keys > 0 {
                for (i, child_page) in info.children.iter().enumerate() {
                    print_tree(pager, *child_page, indentation_level + 1);
                    indent(indentation_level + 1);
                    println!("- key {}", info.keys[i]);
                }
                print_tree(pager, info.right_child, indentation_level + 1);
            }
        }
    }
}

/// Prints indentation spaces.
fn indent(level: u32) {
    for _ in 0..level {
        print!("  ");
    }
}

/// Information for a leaf node.
struct LeafInfo {
    num_cells: u32,
    keys: Vec<u32>,
}

/// Information for an internal node.
struct InternalInfo {
    num_keys: u32,
    keys: Vec<u32>,
    children: Vec<u32>,
    right_child: u32,
}

/// An enum that can be either leaf or internal information.
enum NodeInfo {
    Leaf(LeafInfo),
    Internal(InternalInfo),
}

impl NodeInfo {
    fn leaf_info(self) -> Option<LeafInfo> {
        if let NodeInfo::Leaf(info) = self {
            Some(info)
        } else {
            None
        }
    }

    fn internal_info(self) -> Option<InternalInfo> {
        if let NodeInfo::Internal(info) = self {
            Some(info)
        } else {
            None
        }
    }
}

/// Convenience function to package info.
fn make_node_info(info: impl Into<NodeInfo>) -> NodeInfo {
    info.into()
}

fn _print_ship() {
    println!(
        "                                                  
                                       *+======+# 
                                   +---=-=+==--=*#
                               +=-==========-:-+#%
                             =+#**#%@*+=--:::.:+#%
                          +-=#+*#%@@@%*=::...-=*%%
                        +-:+##%%@@@@@*=:....:-*#% 
                      ==--=*#@@@@@@@#*-...:-=+*%  
                    %+:-=++***#%%%##+:.::--=+*##  
                  ###--==+++++**+=-:.::--=++*#%   
         ###%%#####%==++++++=--::::::--==+*#%@    
       #**#######%@*+****+++=--:::---==++*#%      
     #***#######%%%#****++=----:--===++*#%%       
    ********###%%%#***++=---==++++++**##%         
  #+=++++++***##@%*++===-=+*#****#%##%%%          
 #=--======+++*#%*=====+##*#######%%%*            
+---------= #%%#%===+*%%++####%%%%%               
-:          +###%@#%@@%*#%%%%%%%#                 
         -::..+*+*@@%#*#@%%%####                  
       =-:.....:%@@%#**#########                  
    = =-:.....=%%%+=* %#########                  
    +=-:::...+%%-:*#############                  
    =-.:....+#-.-  -+*#########                   
   +=--::..*::-+   -+**######                     
   +----===-=+     -=+*###                        
  +-:-:=* ==       -=+*#                          
  *                =*                             
    "
    )
}

fn print_constants() {
    println!("ROW_SIZE: {}", ROW_SIZE);
    println!("COMMON_NODE_HEADER_SIZE: {}", COMMON_NODE_HEADER_SIZE);
    println!("LEAF_NODE_HEADER_SIZE: {}", LEAF_NODE_HEADER_SIZE);
    println!("LEAF_NODE_CELL_SIZE: {}", LEAF_NODE_CELL_SIZE);
    println!("LEAF_NODE_SPACE_FOR_CELLS: {}", LEAF_NODE_SPACE_FOR_CELLS);
    println!("LEAF_NODE_MAX_CELLS: {}", LEAF_NODE_MAX_CELLS);
}
