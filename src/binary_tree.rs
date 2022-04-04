struct BinaryTree {
  left_child: Option<Box<BinaryTree>>,
  right_child: Option<Box<BinaryTree>>,
  meta: String,
}

impl BinaryTree {
  fn pre_order(&self, container: &mut Vec<String>) {
    // BinaryTree::visit(self);
    container.push(self.meta.clone());
    if let Some(left) = &self.left_child {
      BinaryTree::pre_order(left, container);
    }
    if let Some(right) = &self.right_child {
      BinaryTree::pre_order(right, container);
    }
  }
}


#[cfg(test)]
mod tests {
  use crate::binary_tree::BinaryTree;

  #[test]
  fn should_use_pre_order_for_binary_tree() {
    let test_binary_tree = BinaryTree {
      left_child: Some(Box::from(BinaryTree {
        left_child: Some(
          Box::from(BinaryTree {
            left_child: None,
            right_child: None,
            meta: "a".to_string(),
          })
        ),
        right_child: Some(
          Box::from(BinaryTree {
            left_child: None,
            right_child: None,
            meta: "b".to_string(),
          })
        ),
        meta: "*".to_string(),
      })),
      right_child: Some(Box::from(BinaryTree {
        left_child: Some(
          Box::from(BinaryTree {
            left_child: None,
            right_child: None,
            meta: "c".to_string(),
          })
        ),
        right_child: Some(
          Box::from(BinaryTree {
            left_child: None,
            right_child: None,
            meta: "d".to_string(),
          })
        ),
        meta: "/".to_string(),
      })),
      meta: "+".to_string(),
    };
    let mut result: Vec<String> = Vec::new();
    test_binary_tree.pre_order(&mut result);
    assert_eq!(result, vec!["+", "*", "a", "b", "/", "c", "d"]);
  }
}
