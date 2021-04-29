/// A fixed-size queue.
/// The `const`-parameter `C` denotes the capacity.
#[derive(Clone, Copy, Debug)]
pub struct Queue<T, const C: usize> {
    elements: [Option<T>; C],
    head: usize,
    length: usize,
}

impl<T: Copy, const C: usize> Default for Queue<T, C> {
    fn default() -> Self {
        Queue::new()
    }
}

impl<T, const C: usize> Queue<T, C> {
    /// Constructs a new, empty `Queue<T, C>`.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let queue = Queue::<i32, 4>::new();
    /// ```
    pub fn new() -> Self
    where
        T: Copy,
    {
        let elements: [Option<T>; C] = [None; C];
        Self {
            elements,
            head: 0,
            length: 0,
        }
    }

    /// Returns the number of elements the queue can hold.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let queue = Queue::<i32, 4>::new();
    /// assert_eq!(queue.capacity(), 4);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        C
    }

    /// Returns the number of elements in the queue.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 1>::new();
    /// assert_eq!(queue.len(), 0);
    /// queue.enqueue(1);
    /// assert_eq!(queue.len(), 1);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the queue contains no elements.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 1>::new();
    /// assert_eq!(queue.is_empty(), true);
    /// queue.enqueue(1);
    /// assert_eq!(queue.is_empty(), false);
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the queue cannot contain any more elements.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 1>::new();
    /// assert_eq!(queue.is_full(), false);
    /// queue.enqueue(1);
    /// assert_eq!(queue.is_full(), true);
    /// ```
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Returns a reference to the element at the given index relative
    /// to the start of the queue.
    /// Returns `None` if there is no element at the position.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 1>::new();
    /// assert_eq!(queue.get(0), None);
    /// queue.enqueue(1);
    /// assert_eq!(queue.get(0), Some(&1));
    /// ```
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&T> {
        // If the index is greater than or equal to `len`,
        // then the computed index would wrap around more
        // than once, making it incorrect.
        if index >= self.len() {
            return None;
        }
        self.elements[(self.head + index) % self.capacity()].as_ref()
    }

    /// Returns the index of the first occupied slot in the queue.
    #[inline]
    const fn head(&self) -> usize {
        self.head
    }

    /// Returns the index of the first empty slot in the queue.
    #[inline]
    const fn tail(&self) -> usize {
        (self.head + self.len()) % self.capacity()
    }

    /// Returns a reference to the underlying storage of the queue.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 3>::new();
    /// assert_eq!(queue.as_slice(), &[None, None, None]);
    /// ```
    #[inline]
    pub const fn as_slice(&self) -> &[Option<T>] {
        &self.elements
    }

    /// Insert an element at the back of the queue.
    /// Returns `Err(element)` if the queue is full.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 3>::new();
    /// assert_eq!(queue.as_slice(), &[None, None, None]);
    /// let _ = queue.enqueue(1);
    /// assert_eq!(queue.as_slice(), &[Some(1), None, None]);
    /// let _ = queue.enqueue(2);
    /// assert_eq!(queue.as_slice(), &[Some(1), Some(2), None]);
    /// ```
    #[inline]
    pub fn enqueue(&mut self, element: T) -> Result<(), T> {
        if self.is_full() {
            return Err(element);
        }
        self.elements[self.tail()] = Some(element);
        self.length += 1;
        Ok(())
    }

    /// Take an element out of the front of the queue.
    /// Returns `None` if the queue is empty.
    ///
    /// # Example
    /// ```
    /// use fundamental::Queue;
    ///
    /// let mut queue = Queue::<i32, 3>::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    /// assert_eq!(queue.dequeue(), Some(1));
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), Some(3));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        let element = self.elements[self.head()].take();
        if element.is_some() {
            self.head = (self.head + 1) % self.capacity();
            self.length -= 1;
        }
        element
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn new() {
        let queue = Queue::<usize, 5>::new();
        assert_eq!(queue.capacity(), 5);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.head(), 0);
        assert_eq!(queue.tail(), 0);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_full(), false);
    }

    #[test]
    fn enqueue() {
        let mut queue = Queue::<usize, 2>::new();
        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.head(), 0);
        assert_eq!(queue.tail(), 1);
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_full(), false);

        assert_eq!(queue.enqueue(2), Ok(()));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.head(), 0);
        assert_eq!(queue.tail(), 0);
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_full(), true);

        assert_eq!(queue.enqueue(3), Err(3));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.head(), 0);
        assert_eq!(queue.tail(), 0);
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_full(), true);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::<usize, 2>::new();
        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.head(), 1);
        assert_eq!(queue.tail(), 0);
        assert_eq!(queue.is_empty(), false);
        assert_eq!(queue.is_full(), false);

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.head(), 0);
        assert_eq!(queue.tail(), 0);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.is_full(), false);
    }
}
