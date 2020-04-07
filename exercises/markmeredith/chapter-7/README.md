1.    Can we turn the Maybe type constructor into a functor by defining:

    fmap _ _ = Nothing

    which ignores both of its arguments? (Hint: Check the functor laws.)

    `fmap id _ = Nothing`   by definition, but `fmap id x = id x = x` by the functor laws. Since not everything is Nothing, this doesn't work.

2.    Prove functor laws for the reader functor. Hint: it’s really simple.

fmap id = id

fmap (g . f) = fmap g . fmap f

3.    Implement the reader functor in your second favorite language (the first being Haskell, of course).

4.    Prove the functor laws for the list functor. Assume that the laws are true for the tail part of the list you’re applying it to (in other words, use induction).

    `fmap . id  = id`
    `fmap (f . g) = fmap f . fmap g`
