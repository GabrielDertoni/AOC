import Control.Arrow ((>>>))
import Data.Function ((&))
import Data.Char (ord)

class Functor f => Comonad f where
    extract :: f a -> a

    duplicate :: f a -> f (f a)

    extend :: (f a -> b) -> f a -> f b
    extend f co = fmap f $ duplicate co

data ListZipper a = ListZipper { lefts :: [a]
                               , focus :: a
                               , rights :: [a]
                               }

listZipperLeft (ListZipper (left:lefts) mid rights) = ListZipper lefts left (mid:rights)
listZipperRight (ListZipper lefts mid (right:rights)) = ListZipper (mid:lefts) right rights
listZipperToList (ListZipper lefts focus rights) = foldl (flip (:)) (focus:rights) lefts
listZipperInLeftEdge (ListZipper [] _ _) = True
listZipperInLeftEdge _ = False
listZipperInRightEdge (ListZipper _ _ []) = True
listZipperInRightEdge _ = False
listToZipper (x:xs) = ListZipper [] x xs

instance Functor ListZipper where
    fmap f (ListZipper lefts focus rights) = ListZipper (fmap f lefts) (f focus) (fmap f rights)

instance Comonad ListZipper where
    extract = focus
    duplicate zipper = ListZipper leftZippers zipper rightZippers
        where leftZippers  = fmap fst $ tail $ zip (iterate listZipperLeft zipper) (lefts zipper)
              rightZippers = fmap fst $ tail $ zip (iterate listZipperRight zipper) (rights zipper)

data GridZipper a = GridZipper (ListZipper (ListZipper a))

gridZipperLeft (GridZipper inner) = GridZipper $ fmap listZipperLeft inner
gridZipperRight (GridZipper inner) = GridZipper $ fmap listZipperRight inner
gridZipperUp (GridZipper inner) = GridZipper $ listZipperLeft inner
gridZipperDown (GridZipper inner) = GridZipper $ listZipperRight inner
gridZipperToGrid (GridZipper inner) = listZipperToList $ fmap listZipperToList inner
gridZipperInLeftEdge (GridZipper (ListZipper _ mid _)) = listZipperInLeftEdge mid
gridZipperInRightEdge (GridZipper (ListZipper _ mid _)) = listZipperInRightEdge mid
gridZipperInTopEdge (GridZipper inner) = listZipperInLeftEdge inner
gridZipperInBotEdge (GridZipper inner) = listZipperInRightEdge inner
gridToZipper (x:xs) = GridZipper $ ListZipper [] (listToZipper x) (fmap listToZipper xs)

instance Functor GridZipper where
    fmap f (GridZipper inner) = GridZipper (fmap (fmap f) inner)

instance Comonad GridZipper where
    extract (GridZipper inner) = focus $ focus inner
    duplicate (GridZipper inner) = fmap GridZipper $ GridZipper (duplicate $ duplicate inner)

isVisible :: GridZipper Int -> Bool
isVisible zipper = visibleLeft || visibleRight || visibleTop || visibleBot
    where height = extract zipper
          visibleLeft = visibleInDirection gridZipperLeft gridZipperInLeftEdge
          visibleRight = visibleInDirection gridZipperRight gridZipperInRightEdge
          visibleTop = visibleInDirection gridZipperUp gridZipperInTopEdge
          visibleBot = visibleInDirection gridZipperDown gridZipperInBotEdge

          -- Iterate the zipper in that direction and drop while the zipper doesn't reach an edge
          -- or a higher tree. When we stop, see if the next one is on the edge. If so, it's
          -- visible.
          visibleInDirection dir edge = iterate dir zipper
                                      & drop 1
                                      & dropWhile (\z -> not (edge z) && extract z < height)
                                      & head
                                      & edge

main = interact
     ( lines
   >>> fmap (fmap ord)
   >>> gridToZipper
   >>> extend isVisible
   >>> gridZipperToGrid
   >>> show
     )
