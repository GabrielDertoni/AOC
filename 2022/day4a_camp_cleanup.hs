import Control.Lens.Combinators (both, _2)
import Control.Lens.Setter ((%~))
import Control.Arrow ((>>>))
import Data.Function ((&))
import Control.Applicative (liftA2)

data Range = Range { start :: Int, end :: Int }

instance Read Range where
    readsPrec prec s = do (start, '-':rest) <- readsPrec prec s
                          (end, rest')      <- readsPrec prec rest
                          return (Range { start = start, end = end }, rest')

split sep list = span ((==sep) >>> not) list
               & _2 %~ drop 1

Range s1 e1 `contains` Range s2 e2 = s1 <= s2 && e1 >= e2

main = interact
     ( lines
   >>> fmap ( split ','
          >>> both %~ read
          >>> liftA2 (||) (uncurry $ flip contains) (uncurry contains)
            )
   >>> filter id
   >>> length
   >>> show
   >>> (++"\n")
     )
