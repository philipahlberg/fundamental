/// A fixed-size stack.
/// The `const`-parameter `C` denotes the capacity.
#[derive(Clone, Copy, Debug)]
pub struct Stack<T, const C: usize> {
    elements: [Option<T>; C],
    length: usize,
}

impl<T: Copy, const C: usize> Default for Stack<T, C> {
    fn default() -> Self {
        Stack::new()
    }
}

impl<T, const C: usize> Stack<T, C> {
    /// Constructs a new, empty `Stack<T, C>`.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let stack = Stack::<i32, 4>::new();
    /// ```
    pub fn new() -> Self
    where
        T: Copy,
    {
        let elements: [Option<T>; C] = [None; C];
        Self {
            elements,
            length: 0,
        }
    }

    /// Returns the number of elements the stack can hold.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let stack = Stack::<i32, 4>::new();
    /// assert_eq!(stack.capacity(), 4);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        C
    }

    /// Returns the number of elements on the stack.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 1>::new();
    /// assert_eq!(stack.len(), 0);
    /// stack.push(1);
    /// assert_eq!(stack.len(), 1);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the stack contains no elements.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 1>::new();
    /// assert_eq!(stack.is_empty(), true);
    /// stack.push(1);
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the stack cannot contain any more elements.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 1>::new();
    /// assert_eq!(stack.is_full(), false);
    /// stack.push(1);
    /// assert_eq!(stack.is_full(), true);
    /// ```
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Returns a reference to the element at the given index.
    /// Returns `None` if there is no element at the position.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 1>::new();
    /// assert_eq!(stack.get(0), None);
    /// stack.push(1);
    /// assert_eq!(stack.get(0), Some(&1));
    /// ```
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        self.elements[index].as_ref()
    }

    /// Returns the index of the last occupied slot in the stack.
    #[inline]
    const fn top(&self) -> usize {
        self.len() - 1
    }

    /// Returns a reference to the underlying storage of the stack.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 3>::new();
    /// assert_eq!(stack.as_slice(), &[None, None, None]);
    /// ```
    #[inline]
    pub const fn as_slice(&self) -> &[Option<T>] {
        &self.elements
    }

    /// Insert an element at the top of the stack.
    /// Returns `Err(element)` if the stack is full.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 3>::new();
    /// assert_eq!(stack.as_slice(), &[None, None, None]);
    /// let _ = stack.push(1);
    /// assert_eq!(stack.as_slice(), &[Some(1), None, None]);
    /// let _ = stack.push(2);
    /// assert_eq!(stack.as_slice(), &[Some(1), Some(2), None]);
    /// ```
    #[inline]
    pub fn push(&mut self, element: T) -> Result<(), T> {
        if self.is_full() {
            return Err(element);
        }
        self.elements[self.len()] = Some(element);
        self.length += 1;
        Ok(())
    }

    /// Take an element off of the top of the stack.
    /// Returns `None` if the stack is empty.
    ///
    /// # Example
    /// ```
    /// use fundamental::Stack;
    ///
    /// let mut stack = Stack::<i32, 3>::new();
    /// stack.push(1);
    /// stack.push(2);
    /// stack.push(3);
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), Some(2));
    /// assert_eq!(stack.pop(), Some(1));
    /// assert_eq!(stack.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let element = self.elements[self.top()].take();
        self.length -= 1;
        element
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn new() {
        let stack = Stack::<usize, 5>::new();
        assert_eq!(stack.capacity(), 5);
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.is_full(), false);
    }

    #[test]
    fn push() {
        let mut stack = Stack::<usize, 2>::new();
        assert_eq!(stack.push(1), Ok(()));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.is_full(), false);

        assert_eq!(stack.push(2), Ok(()));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.is_full(), true);

        assert_eq!(stack.push(3), Err(3));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.is_full(), true);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::<usize, 2>::new();
        assert_eq!(stack.push(1), Ok(()));
        assert_eq!(stack.push(2), Ok(()));

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.is_full(), false);

        // assert_eq!(stack.pop(), Some(1));
        // assert_eq!(stack.len(), 0);
        // assert_eq!(stack.is_empty(), true);
        // assert_eq!(stack.is_full(), false);

        // assert_eq!(stack.pop(), None);
    }
}
