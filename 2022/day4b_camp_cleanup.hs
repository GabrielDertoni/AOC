import Control.Lens.Combinators (both)
import Control.Lens.Setter ((%~))
import Control.Arrow ((>>>))
import Control.Applicative (liftA2)

data Range = Range { start :: Int, end :: Int }

instance Read Range where
    readsPrec prec s = do (start, '-':rest) <- readsPrec prec s
                          (end, rest')      <- readsPrec prec rest
                          return (Range { start = start, end = end }, rest')

split sep list = let (lhs, rhs) = span ((==sep) >>> not) list
                  in (lhs, drop 1 rhs)

Range s1 e1 `overlaps` Range s2 e2 = max s1 s2 <= min e1 e2

main = interact
     ( lines
   >>> fmap ( split ','
          >>> both %~ read
          >>> uncurry overlaps
            )
   >>> filter id
   >>> length
   >>> show
   >>> (++"\n")
     )
