use crate::Discord;

/// A reusable Iterator for the SDK's methods to acquire collections.
pub struct GenericIter<'a, 'b, T> {
    discord: &'b Discord<'a>,
    getter: Box<dyn FnMut(&Discord<'_>, usize) -> T>,
    count: usize,
    index: usize,
    back_index: usize,
}

impl<'a: 'b, 'b, T> GenericIter<'a, 'b, T> {
    pub(crate) fn new(
        discord: &'b Discord<'a>,
        getter: Box<dyn FnMut(&Discord<'_>, usize) -> T>,
        count: usize,
    ) -> Self {
        Self {
            discord,
            getter,
            count,
            index: 0,
            back_index: 0,
        }
    }
}

impl<T> Iterator for GenericIter<'_, '_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.index += 1;
            Some((self.getter)(self.discord, self.index - 1))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl<T> DoubleEndedIterator for GenericIter<'_, '_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.back_index += 1;
            Some((self.getter)(self.discord, self.count - self.back_index))
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for GenericIter<'_, '_, T> {}

impl<T> std::iter::FusedIterator for GenericIter<'_, '_, T> {}

impl<T> std::fmt::Debug for GenericIter<'_, '_, T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("discord_game_sdk::GenericIter")
            .field("discord", &self.discord)
            .field("getter", &())
            .field("count", &self.count)
            .field("index", &self.index)
            .field("back_index", &self.back_index)
            .finish()
    }
}
