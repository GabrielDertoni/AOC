import Control.Lens.Combinators (each)
import Control.Lens.Setter ((%~))
import Control.Arrow ((>>>))
import Data.Maybe (fromMaybe)
import Data.Char (isLower)
import qualified Data.Set as Set

chunks _ [] = []
chunks n list = let (x, rest) = splitAt n list
                 in x : chunks n rest

priority c
  | isLower c = fromEnum c - fromEnum 'a' + 1
  | otherwise = fromEnum c - fromEnum 'A' + 27

main = interact
     ( lines
   >>> chunks 3
   >>> fmap ( each %~ Set.fromList
          >>> foldr1 Set.intersection
          >>> Set.elemAt 0
          >>> priority
            )
   >>> sum
   >>> show
   >>> (++"\n")
     )

