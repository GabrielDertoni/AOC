import Control.Lens.Combinators (both)
import Control.Lens.Setter ((%~))
import Control.Arrow ((>>>))
import Data.Maybe (fromMaybe)
import Data.Char (isLower)
import qualified Data.Set as Set

priority c
  | isLower c = fromEnum c - fromEnum 'a' + 1
  | otherwise = fromEnum c - fromEnum 'A' + 27

main = interact
     ( lines
   >>> fmap ( (splitAt =<< length >>> (`div` 2))
          >>> both %~ Set.fromList
          >>> uncurry Set.intersection
          >>> Set.elemAt 0
          >>> priority
            )
   >>> sum
   >>> show
   >>> (++"\n")
     )
