use trees::bstree::BSTree;
use trees::rbtree::RBTree;
use trees::avltree::AVLTree;
use trees::base::Tree;

use std::io::{stdin, stdout, Write};


fn avltree_cli() {
    println!("\n------ AVL Tree branch ------\n");
    let mut tree = AVLTree::<i32>::new();
    availabe_operations();

    loop {
        print!("Operation > ");
        let operation = get_user_input();

        match operation.as_str().to_lowercase().trim() {
            "1"  => {
                let value = get_value("insert");
                tree.insert(value);
            },
            "2" => {
                let value = get_value("delete");
                tree.delete(value);
            },
            "3" => println!("Number of leaves: {:?}", tree.count_leaves()),
            "4" => println!("Number of nodes: {:?}", tree.count_nodes()),
            "5" => println!("Height of the tree: {:?}", tree.get_height()),
            "6" => {
                let max_value = tree.get_max();
                match max_value {
                    None => println!("The tree is empty!"),
                    Some(max) => println!("Maximum Value: {:?}", max),
                }
            },
            "7" => {
                let min_value = tree.get_min();
                match min_value {
                    None => println!("The tree is empty!"),
                    Some(min) => println!("Minimum Value: {:?}", min),
                }
            },
            "8" => println!("Is the tree empty? {:?}", tree.is_empty()),
            "9" | "search" => {
                let value = get_value("search");
                println!("Value found? {:?}", tree.contain(value));
            },
            "10" => {
                loop{
                    println!("Enter the number corresponding to the tree traversal type you want or type 'back' to select a different operation!\n1- Inorder \n2- Preorder \n3- Postorder");
                    print!("input > ");
                    let trav_type = get_user_input();
                    match trav_type.as_str().to_lowercase().trim() {
                        "1" => {
                            print!("Your tree:\n");
                            tree.traverse_inorder();
                            break;
                        },
                        "2" => {
                            print!("Your tree:\n");
                            tree.traverse_preorder();
                            break;
                        },
                        "3" => {
                            print!("Your tree:\n");
                            tree.traverse_postorder();
                            break;
                        },
                        "back" => break,
                        _ => {
                            eprint!("This command is not recognized. \n");
                        }
                    }
                }
            },            
            "11" => {tree.print_tree();},
            "12" => {tree.clear();},
            "help" => availabe_operations(),
            "back" => {
                print!("\n");
                return},
            _ => println!("This command is not recognized. Type 'help' for the list of valid operations"),
        }
    }
}


fn rbtree_cli() {
    println!("\n------ Red-Black Tree branch ------\n");
    let mut tree = RBTree::<i32>::new();
    availabe_operations();

    loop {
        print!("Operation > ");
        let operation = get_user_input();

        match operation.as_str().to_lowercase().trim() {
            "1"  => {
                let value = get_value("insert");
                tree.insert(value);
            },
            "2" => {
                let value = get_value("delete");
                tree.delete(value);
            },
            "3" => println!("Number of leaves: {:?}", tree.count_leaves()),
            "4" => println!("Number of nodes: {:?}", tree.count_nodes()),
            "5" => println!("Height of the tree: {:?}", tree.get_height()),
            "6" => {
                let max_value = tree.get_max();
                match max_value {
                    None => println!("The tree is empty!"),
                    Some(max) => println!("Maximum Value: {:?}", max),
                }
            },
            "7" => {
                let min_value = tree.get_min();
                match min_value {
                    None => println!("The tree is empty!"),
                    Some(min) => println!("Minimum Value: {:?}", min),
                }
            },
            "8" => println!("Is the tree empty? {:?}", tree.is_empty()),
            "9" | "search" => {
                let value = get_value("search");
                println!("Value found? {:?}", tree.contain(value));
            },
            "10" => {
                loop{
                    println!("Enter the number corresponding to the tree traversal type you want or type 'back' to select a different operation!\n1-Inorder \n2-Preorder \n3-Postorder");
                    print!("input > ");
                    let trav_type = get_user_input();
                    match trav_type.as_str().to_lowercase().trim() {
                        "1" => {
                            print!("Your tree:\n");
                            tree.traverse_inorder();
                            break;
                        },
                        "2" => {
                            print!("Your tree:\n");
                            tree.traverse_preorder();
                            break;
                        },
                        "3" => {
                            print!("Your tree:\n");
                            tree.traverse_postorder();
                            break;
                        },
                        "back" => break,
                        _ => {
                            eprint!("This command is not recognized. \n");
                        }
                    }
                }
            },            
            "11" => {tree.print_tree();},
            "12" => {tree.clear();},
            "help" => availabe_operations(),
            "back" => {
                print!("\n");
                return},
            _ => println!("This command is not recognized. Type 'help' for the list of valid operations"),
        }
    }
}


fn bstree_cli() {
    println!("\n------ Binary Search Tree branch ------\n");
    let mut tree = BSTree::<i32>::new();
    availabe_operations();

    loop {
        print!("Operation > ");
        let operation = get_user_input();

        match operation.as_str().to_lowercase().trim() {
            "1"  => {
                let value = get_value("insert");
                tree.insert(value);
            },
            "2" => {
                let value = get_value("delete");
                tree.delete(value);
            },
            "3" => println!("Number of leaves: {:?}", tree.count_leaves()),
            "4" => println!("Number of nodes: {:?}", tree.count_nodes()),
            "5" => println!("Height of the tree: {:?}", tree.get_height()),
            "6" => {
                let max_value = tree.get_max();
                match max_value {
                    None => println!("The tree is empty!"),
                    Some(max) => println!("Maximum Value: {:?}", max),
                }
            },
            "7" => {
                let min_value = tree.get_min();
                match min_value {
                    None => println!("The tree is empty!"),
                    Some(min) => println!("Minimum Value: {:?}", min),
                }
            },
            "8" => println!("Is the tree empty? {:?}", tree.is_empty()),
            "9" | "search" => {
                let value = get_value("search");
                println!("Value found? {:?}", tree.contain(value));
            },
            "10" => {
                loop{
                    println!("Enter the number corresponding to the tree traversal type you want or type 'back' to select a different operation!\n1-Inorder \n2-Preorder \n3-Postorder");
                    print!("input > ");
                    let trav_type = get_user_input();
                    match trav_type.as_str().to_lowercase().trim() {
                        "1" => {
                            print!("Your tree:\n");
                            tree.traverse_inorder();
                            break;
                        },
                        "2" => {
                            print!("Your tree:\n");
                            tree.traverse_preorder();
                            break;
                        },
                        "3" => {
                            print!("Your tree:\n");
                            tree.traverse_postorder();
                            break;
                        },
                        "back" => break,
                        _ => {
                            eprint!("This command is not recognized. \n");
                        }
                    }
                }
            },
            "11" => {tree.print_tree();},
            "12" => {tree.clear();},
            "help" => availabe_operations(),
            "back" => {
                print!("\n");
                return},
            _ => println!("This command is not recognized. Type 'help' for the list of valid operations"),
        }
    }
}


pub fn start_cli(){
    loop {
        println!("You can select a tree number to start or type 'exit' to leave!");
        println!("Select a tree!\n1- Red-Black Tree \n2- AVL Tree \n3- Binary Search Tree");
        print!("input > ");
        let selected_tree_num = get_user_input();

        match selected_tree_num.as_str().to_lowercase().trim() {
            "1" => {
                rbtree_cli();
            },
            "2" => {
                avltree_cli();
            },
            "3" => {
                bstree_cli();
            },
            "exit" => break,
            _ => {
                eprint!("This command is not recognized.\n\n");
            }
        }
    }
}

pub fn get_user_input() -> String {
    let mut line = String::new();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(&mut line).expect("Failed to read from stdin");
    line.to_string()
}

pub fn get_value(oper: &str)-> i32 {
    loop {
        print!("{} value > ", oper);
        let value = get_user_input();
        let trimmed_value = value.trim();
        match trimmed_value.parse::<i32>(){
            Ok(val) => {
                println!("The {} operation for '{}' in the tree is complete!", oper, val);
                return val;
            },
            Err(..) => {
                println!("This is not an integer number");
            },
        };
    }
}

pub fn availabe_operations(){
    println!("\nAvailabe Operations: \n------------------");
    println!("Enter the number corresponding to the operation you want to perform! \n");
    println!("1) Insert         - insert a node into the tree.");
    println!("2) Delete         - delete a node from the tree.");
    println!("3) Count Leaves   - count the number of leaves in the tree.");
    println!("4) Count Nodes    - count the number of nodes in the tree.");
    println!("5) Height         - return the height of the tree.");
    println!("6) Maximum        - find the maximum value in the tree.");
    println!("7) Minimum        - find the minimum value in the tree.");
    println!("8) Empty          - check if the tree is empty.");
    println!("9) Search         - check if the tree contains a certain value.");
    println!("10) Traverse      - traverse the tree (Inorder, Preorder, or Postorder)");
    println!("11) Print         - print the tree.");
    println!("12) Clear         - clear the tree, removing all elements.");

    println!("Back              - Go back to previous menu and erase current tree \n");
}

pub fn welcome(){
    println!("---------------------------------------- Welcome to our Trees Command Line Interface ----------------------------------------\n");
    println!("Available trees:\n- Red-Black Tree \n- AVL tree \n- Binary Search Tree \n");
    println!("Availabe operations: \n1- Insert \n2- Delete \n3- Count Leaves \n4- Count Nodes \n5- Height \n6- Maximum \n7- Minimum \n8- Empty \n9- Search \n10- Traverse \n11- Print \n12- Clear\n");
    println!("How to use the Command Line Interface: ");
    println!("-------------------");
}
