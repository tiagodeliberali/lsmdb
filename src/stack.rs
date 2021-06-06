struct StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    pub fn new<S>(value: S, next: Option<StackNode<S>>) -> StackNode<S> {
        let next = match next {
            Some(node) => Some(Box::new(node)),
            None => None,
        };

        StackNode { value, next }
    }
}

pub struct Stack<T> {
    root: Option<StackNode<T>>,
}

impl<T> Stack<T> {
    pub fn new<S>() -> Stack<S> {
        Stack { root: None }
    }

    pub fn push(&mut self, value: T) {
        let old_root = self.root.take();

        let new_root = StackNode::<T>::new::<T>(value, old_root);

        self.root.replace(new_root);
    }

    pub fn pop(&mut self) -> Option<T> {
        let root = self.root.take();

        if let Some(node) = root {
            if let Some(next) = node.next {
                self.root.replace(*next);
            }

            return Some(node.value);
        }
        None
    }

    pub fn has_value(&self) -> bool {
        self.root.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_stack() {
        let stack = Stack::<String>::new::<String>();

        assert_eq!(stack.has_value(), false);
    }

    #[test]
    fn push_items_on_stack_hs_value() {
        let stack = &mut Stack::<String>::new::<String>();

        stack.push(String::from("first"));

        assert_eq!(stack.has_value(), true);
    }

    #[test]
    fn push_items_on_stack_pop_in_order() {
        let stack = &mut Stack::<String>::new::<String>();

        stack.push(String::from("first"));
        stack.push(String::from("second"));

        assert_eq!(stack.pop(), Some(String::from("second")));
        assert_eq!(stack.pop(), Some(String::from("first")));
        assert_eq!(stack.pop(), None);
    }
}
