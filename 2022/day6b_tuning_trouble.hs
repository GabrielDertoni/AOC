import Control.Lens.Combinators (ifind, singular, _Just, _1)
import Control.Lens.Getter (view)
import Control.Arrow ((>>>))
import Data.Function ((&))
import Data.Bits ((.|.), popCount, bit)
import Data.List (foldl')
import Data.Char (ord)

windows n []   = []
windows n list = take n list : windows n (tail list)

toBitset = foldl' (\set c -> set .|. bit (ord c - ord 'a')) (0 :: Word)
allEqual list = toBitset list & popCount & (== length list)

main = interact
     ( windows 14
   >>> ifind (const allEqual)
   >>> view (singular $ _Just._1)
   >>> (+14)
   >>> show
   >>> (++"\n")
     )
