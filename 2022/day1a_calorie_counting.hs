{-# LANGUAGE TypeApplications #-}

import Data.Function ((&))
import Control.Arrow ((>>>))

splitBy when list
  | null list = []
  | otherwise = let (left, right) = span (when >>> not) list
                 in left : splitBy when (drop 1 right)

main = interact
     ( lines
   >>> splitBy (=="")
   >>> fmap (fmap read >>> (sum @[] @Int))
   >>> maximum
   >>> (show >>> (++"\n"))
     )
