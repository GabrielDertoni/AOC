{-# LANGUAGE TypeApplications, DeriveFunctor, RecordWildCards, UndecidableInstances #-}

import Data.Map.Strict (Map)
import qualified Data.Map.Strict as Map
import Data.Set (Set)
import qualified Data.Set as Set
import qualified Text.ParserCombinators.ReadP as P
import Data.Char (isLetter, isDigit)
import Data.Function ((&), on)
import Control.Arrow ((>>>), (<<<))
import Data.List (findIndex, find)
import Data.Maybe (maybeToList)

import Debug.Trace

data Valve = Valve { _name :: String
                   , _rate :: Int
                   , _connections :: [String]
                   } deriving Show

instance Eq Valve where
    (==) = (==) `on` _name

instance Read Valve where
    readsPrec prec = P.readP_to_S $ do token $ P.string "Valve"
                                       name <- parseWord
                                       tokens $ P.string <$> words "has flow rate="
                                       rate <- P.readS_to_P (readsPrec prec)
                                       P.char ';'
                                       tokens [ maybePlural "tunnel"
                                              , maybePlural "lead"
                                              , () <$ P.string "to"
                                              , maybePlural "valve"
                                              ]
                                       connections <- P.sepBy1 parseWord $ token $ P.char ','
                                       return $ Valve { _name = name
                                                      , _rate = rate
                                                      , _connections = connections
                                                      }
        where maybePlural s = P.string s >> P.optional (P.char 's')
              token p = P.skipSpaces *> p <* P.skipSpaces
              tokens = sequence . fmap token
              parseWord = P.munch1 isLetter
              number = P.munch1 isDigit

newtype Term f = In { out :: f (Term f) }
data RoseR a rec = Rose a [rec] deriving (Show, Functor)
type Rose a = RoseR a (Term (RoseR a))

instance Show (f (Term f)) => Show (Term f) where
    showsPrec prec = showsPrec prec . out

cata f = out >>> fmap (cata f) >>> f
ana  f = In  <<< fmap (ana  f) <<< f

data State = State { _flowTillEnd :: Int
                   , _at :: String
                   , _minutesLeft :: Int
                   } deriving Show

type Graph = Map String Valve

gen :: (Graph, State) -> RoseR State (Graph, State)
gen (graph, st@State{..}) = if _minutesLeft == 0
                               then Rose curr []
                               else Map.elems subgraph
                                  & fmap (\nxt -> (subgraph, nextState $ _name nxt))
                                  & Rose curr
    where nextState nxt = State { _flowTillEnd = currFlow
                                , _at = nxt
                                , _minutesLeft = max 0 $ _minutesLeft - 2
                                }
          currFlow = _flowTillEnd + _rate (graph Map.! _at) * (max 0 (_minutesLeft - 1))
          curr = st { _flowTillEnd = currFlow }
          subgraph = Map.delete _at graph

removeZeroFlow :: Map String Valve -> Map String Valve
removeZeroFlow graph = foldr removeNode graph zeroFlow
    where -- The list valves with flow rate 0.
          zeroFlow = Map.elems graph
                   & filter ((==0) . _rate)
                   & map _name

          -- Assumes that the graph is undirected, that is for all edges A -> B there is also an
          -- edge B -> A.
          removeNode name graph = _connections valve
                                & foldr (Map.adjust updateConn) (Map.delete (_name valve) graph)
              where connSet = Set.fromList $ _connections valve
                    updateConn conn = let others = filter (/= _name conn) $ _connections valve
                                          -- Remove the connection to `valve` and add other
                                          -- connections from valve.
                                          newconn = filter (/= _name valve) (_connections conn) ++ others
                                       in conn { _connections = newconn }
                    valve = graph Map.! name


{-
shortestDistances :: Map String Valve -> Map (String, String) Int
shortestDistances graph = foldr () initialMap (Map.keys graph)
    where initialMap = Map.fromList $ zip pairs $ fmap initialWeight pairs
          pairs = [(a, b) | a <- Map.keys graph, b <- Map.keys graph]
          initialWeight (a, b)
            | a == b = 0
            | otherwise = case find (== _name a) (_connections b) of
                            Just _ -> 1
                            Nothing -> maxBound @Int
          keys = Map.keys graph
-}

distance graph from to | from == to = return 0
distance graph from to = do node <- maybeToList $ graph Map.!? from
                            dest <- _connections node
                            dist <- distance (Map.delete from graph) dest to
                            return (dist + 1)

naive' _ _ _ released 0 _ = return released
naive' _ closed _ released _ _ | Set.null closed = return released
naive' graph closed at released minutesLeft mindist =
    do to <- Set.elems closed
       let roundsWalking = mindist at to
       let minutesLeft' = max 0 $ minutesLeft - roundsWalking - 1
       let releasedTillEnd = minutesLeft' * _rate (graph Map.! to)
       let closed' = Set.delete to closed
       naive' graph closed' to (released + releasedTillEnd) minutesLeft' mindist

naive at minutesLeft graph = naive' graph closed at 0 minutesLeft mindist
    where closed = Set.fromList $ fmap _name $ filter ((>0) . _rate) $ Map.elems graph
          mindist a b = mindistMap Map.! (a, b)
          mindistMap = minDist graph

test = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
\Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
\Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
\Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
\Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
\Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
\Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
\Valve HH has flow rate=22; tunnel leads to valve GG\n\
\Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
\Valve JJ has flow rate=21; tunnel leads to valve II"

test' = "Valve BB has flow rate=13; tunnels lead to valves CC, DD, JJ\n\
\Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
\Valve DD has flow rate=20; tunnels lead to valves CC, EE, BB, JJ\n\
\Valve EE has flow rate=3; tunnels lead to valves DD, HH\n\
\Valve HH has flow rate=22; tunnel leads to valve EE\n\
\Valve JJ has flow rate=21; tunnel leads to valve DD, BB"

toGraph = lines >>> fmap read >>> fmap (flip (,) <*> _name) >>> Map.fromList

g = toGraph test

minDist graph = Map.fromList [((a, b), minimum $ distance graph a b) | a <- Map.keys graph, b <- Map.keys graph]

bfs graph from to = length $ takeWhile (to `Set.notMember`) $ iterate iter (Set.singleton from)
    where iter level = Set.fromList $ do node <- Set.elems level
                                         child <- _connections $ graph Map.! node
                                         return child

main = interact
     ( lines
   >>> fmap read
   >>> fmap (flip (,) <*> _name)
   >>> Map.fromList
   >>> minDist
   >>> show
   -- >>> naive "AA" 30
   -- >>> maximum
   -- >>> show
   -- >>> (++"\n")
     )
