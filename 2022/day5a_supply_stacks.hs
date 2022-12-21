{-# LANGUAGE RecordWildCards, TypeFamilies #-}

import Control.Lens.Combinators (_1, _2, _head, mapped, singular, taking, each)
import Control.Lens.Setter ((%~), (.~))
import Control.Lens.Getter ((^.))
import Control.Lens.Operators ((^?), (^..))
import Control.Lens.At (ix)
import Control.Arrow ((>>>))
import Data.Function ((&))
import Data.List (foldl')
import Text.ParserCombinators.ReadP as P

split sep list = span ((==sep) >>> not) list
               & _2 %~ drop 1

chunks _ [] = []
chunks n list = let (x, rest) = splitAt n list
                 in x : chunks n rest

parseStacks lines = flip fmap (init lines) (chunks 4 >>> fmap readBracketedLetter)
                  -- Will return a list of lists, a list of the stacks
                  & foldr1 (zipWith (++))
    where -- Reads something like "[A]" into `['A']`.
          readBracketedLetter :: String -> [Char]
          readBracketedLetter s = P.readP_to_S readBracketedLetterP s ^.. taking 1 (each._1)
          readBracketedLetterP = P.between (P.char '[') (P.char ']') P.get

data Move = Move { nToMove :: Int
                 , from    :: Int
                 , to      :: Int
                 } deriving Show

parseMove s = P.readP_to_S parser s ^. (singular _head . _1)
    where parser = do spaced "move"
                      n <- P.readS_to_P (readsPrec 0)
                      spaced "from"
                      from <- P.readS_to_P (readsPrec 0)
                      spaced "to"
                      to <- P.readS_to_P (readsPrec 0)
                      return Move { nToMove = n, from = from - 1, to = to - 1 }
          spaced s = P.between P.skipSpaces P.skipSpaces (P.string s)

simulate stacks moves = foldl' iter stacks moves
    where iter stacks Move{..} = let (moving, staying) = splitAt nToMove (stacks ^. singular (ix from))
                                  in stacks & ix to   %~ (reverse moving ++)
                                            & ix from .~ staying

main = interact
     ( lines
   >>> split "" -- split on the empty line
   >>> _1 %~ parseStacks
   >>> (_2 . mapped) %~ parseMove
   >>> uncurry simulate
   >>> fmap (take 1)
   >>> foldl1 (++)
   >>> (++"\n")
     )
