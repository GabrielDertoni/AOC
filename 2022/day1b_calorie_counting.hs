{-# LANGUAGE TypeApplications #-}

import Data.List (sort, insert, partition)
import Data.Function ((&))
import Control.Arrow ((>>>))

splitBy when list
  | null list = []
  | otherwise = let (left, right) = span (when >>> not) list
                 in left : splitBy when (drop 1 right)

-- O(m * n), where `m = length list`.
nBiggest n list
  | n <= 0 = []
  | length init < n = init
  | otherwise = go (init & sort) (drop n list)
    where init = take n list
          go acc [] = acc
          go (small:rest) (x:xs)
            | x > small = go (insert x rest) xs
            | otherwise = go (small:rest) xs

main = interact
     ( lines
   >>> splitBy (=="")
   >>> fmap (fmap read >>> (sum @[] @Int))
   >>> nBiggest 3
   >>> sum
   >>> (show >>> (++"\n"))
     )
