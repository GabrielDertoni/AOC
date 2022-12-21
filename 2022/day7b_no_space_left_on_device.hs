{-# LANGUAGE TypeApplications, UndecidableInstances, DeriveFunctor #-}

import Control.Arrow ((>>>))
import Control.Applicative (Alternative(..))
import Data.Function ((&), on)
import Data.Char (isDigit)
import Data.List (intersperse, unionBy)
import Data.Maybe (catMaybes, fromJust)
import Data.Either (partitionEithers, fromRight)
import Text.ParserCombinators.ReadP as P

import Debug.Trace

data Dir = Dir String deriving (Show, Eq)
data File = File Integer String deriving (Show, Eq)
data Command = Ls | Cd String deriving (Show, Eq)
data DirEntry = FileEntry File | DirEntry Dir deriving (Show, Eq)
data TermOutput = CmdEcho Command | DirOutput DirEntry deriving (Show, Eq)

dirName (Dir name) = name
fileSize (File size _) = size

parseTermOutput = parseCommand <|> parseDirEntry
    where parseCommand = do P.char '$'
                            P.skipSpaces
                            parseCd <|> parseLs
          parseCd = do P.string "cd"
                       P.skipSpaces
                       path <- P.munch1 (const True)
                       return (CmdEcho (Cd path))
          parseLs = do P.string "ls"
                       return (CmdEcho Ls)

          parseDirEntry = parseDir <|> parseFile
          parseDir = do P.string "dir"
                        P.skipSpaces
                        name <- P.munch1 (const True)
                        return (DirOutput (DirEntry (Dir name)))
          parseFile = do size <- P.munch1 isDigit
                         P.skipSpaces
                         name <- P.munch1 (const True)
                         return (DirOutput (FileEntry (File (read size) name)))

instance Read TermOutput where
    readsPrec _ = P.readP_to_S parseTermOutput

-- data DirTree = TreeFile File | TreeDir Dir [DirTree] deriving Show

newtype Term f = In { out :: f (Term f) }
instance Show (f (Term f)) => Show (Term f) where
    showsPrec prec (In inner) = showsPrec prec inner

data DirTreeF rec = TreeFile File | TreeDir Dir [rec] deriving (Show, Functor)
type DirTree = DirTreeF (Term DirTreeF)

data DirTreeCrumb = DirTreeCrumb Dir -- The name of the directory
                                 [DirTree] -- The children to the left
                                 [DirTree] -- The children to the right
data DirTreeZipper = DirTreeZipper DirTree -- Current subtree
                                   [DirTreeCrumb] -- Parents

treeEntryName (TreeDir (Dir name) _) = name
treeEntryName (TreeFile (File _ name)) = name

dirEntryToTree (FileEntry file) = TreeFile file
dirEntryToTree (DirEntry  dir)  = TreeDir dir []

dirTreeToEntry (TreeFile file) = FileEntry file
dirTreeToEntry (TreeDir dir _)   = DirEntry dir

instance Show DirTreeZipper where
    show (DirTreeZipper (TreeDir (Dir name) children) crumbs) =
        crumbPath ++ "/" ++ name ++ " : " ++ show (fmap (out >>> dirTreeToEntry) children)
        where crumbPath = [name | DirTreeCrumb (Dir name) _ _ <- reverse crumbs]
                        & intersperse "/"
                        & concat

rootDirTree = TreeDir (Dir "") []

zipperFromDirTree tree = DirTreeZipper tree []

zipperDir (DirTreeZipper focus _) = focus

zipperEnter :: String -> DirTreeZipper -> DirTreeZipper
zipperEnter name (DirTreeZipper (TreeDir dir entries) path) =
    DirTreeZipper subdir (DirTreeCrumb dir lefts rights : path)
    where (lefts, subdir, rights) = find (fmap out entries) []

          find :: [DirTree] -> [DirTree] -> ([DirTree], DirTree, [DirTree])
          find (file@(TreeFile _):rights) lefts = find rights (file : lefts)
          find (node@(TreeDir (Dir found) _):rights) lefts
            | found == name = (lefts, node, rights)
            | otherwise = find rights (node : lefts)


zipperBack :: DirTreeZipper -> DirTreeZipper
zipperBack (DirTreeZipper curr (DirTreeCrumb parent lefts rights:path)) =
    DirTreeZipper (TreeDir parent $ fmap In (reverse lefts ++ [curr] ++ rights)) path

zipperBackToTop :: DirTreeZipper -> DirTreeZipper
zipperBackToTop zipper@(DirTreeZipper _ []) = zipper
zipperBackToTop zipper = zipperBackToTop $ zipperBack zipper

zipperReplace :: DirTree -> DirTreeZipper -> DirTreeZipper
zipperReplace focus (DirTreeZipper _ path) = DirTreeZipper focus path

zipperModify :: (DirTree -> DirTree) -> DirTreeZipper -> DirTreeZipper
zipperModify f (DirTreeZipper focus path) = DirTreeZipper (f focus) path

zipperToDirTree :: DirTreeZipper -> DirTree
zipperToDirTree = zipperBackToTop >>> zipperDir

explore :: TermOutput -> DirTreeZipper -> DirTreeZipper
explore (CmdEcho Ls) zipper = zipper
explore (CmdEcho (Cd "/")) zipper = zipperBackToTop zipper
explore (CmdEcho (Cd "..")) zipper = zipperBack zipper
explore (CmdEcho (Cd dir)) zipper = zipperEnter dir zipper
explore (DirOutput entry) zipper = zipperModify joinChildren zipper
    where -- Here we use `unionBy` which will remove elements from the second list and not the first.
          -- This is necessary to handle cases where the user did something like `ls`, `cd smthg`,
          -- `cd ..` and `ls` again. If it was simple insertion it would duplicate the entries so
          -- we mantain whatever is already in `children`.
          joinChildren (TreeDir dir children) =
            TreeDir dir $ unionBy ((==) `on` (out >>> treeEntryName)) children [In $ dirEntryToTree entry]

cata :: Functor f => (f a -> a) -> Term f -> a
cata f = out >>> fmap (cata f) >>> f

totalUsedSpace (TreeFile file) = fileSize file
totalUsedSpace (TreeDir _ children) = sum children

-- At each step will return `Left Int` if it is a file and `Right (mySize, Maybe result)` where
-- `result` is the size of the samllest directory in that branch that has enough space. The `Maybe`
-- is needed since we need something to return if the node is not a viable candidate.
findDirs needToFree (TreeFile file) = Left (fileSize file)
findDirs needToFree (TreeDir _ children) = let (files, subdirs) = partitionEithers children
                                               (subdirSizes, candidates) = unzip subdirs
                                               mySize = sum files + sum subdirSizes
                                               -- Remove invalid candidates
                                               candidates' = catMaybes candidates
                                            in if mySize >= needToFree
                                                  then Right (mySize, Just $ minimum (mySize : candidates'))
                                                  else Right (mySize, if null candidates'
                                                                         then Nothing
                                                                         else Just $ minimum candidates')

main = interact
     ( lines
   >>> fmap (read @TermOutput)
   >>> foldl (flip explore) (zipperFromDirTree rootDirTree)
   >>> zipperToDirTree
   >>> In
   >>> (do used <- cata totalUsedSpace
           let needToFree = 30000000 - (70000000 - used)
           cata (findDirs needToFree)
       )
   >>> fromRight undefined
   >>> snd
   >>> fromJust
   >>> show
   >>> (++"\n")
     )
