import Data.Char (isSpace)
import Control.Arrow ((>>>))
import Data.Function ((&))

data Choice = Rock | Paper | Scissors deriving Eq

instance Read Choice where
    readsPrec _ s = do choice <- case stripped of
                                   'A':_ -> pure Rock
                                   'X':_ -> pure Rock
                                   'B':_ -> pure Paper
                                   'Y':_ -> pure Paper
                                   'C':_ -> pure Scissors
                                   'Z':_ -> pure Scissors
                                   []    -> []
                       return (choice, tail stripped)
        where stripped = s & span isSpace & snd

Rock     `beats` Scissors = True
Paper    `beats` Rock     = True
Scissors `beats` Paper    = True
_        `beats` _        = False

whatBeats Rock     = Paper
whatBeats Paper    = Scissors
whatBeats Scissors = Rock

whatIsBeatenBy Paper    = Rock
whatIsBeatenBy Scissors = Paper
whatIsBeatenBy Rock     = Scissors

choiceScore Rock     = 1
choiceScore Paper    = 2
choiceScore Scissors = 3

score opponentsChoice yourChoice = choiceScore yourChoice + matchResultScore
    where matchResultScore = if yourChoice `beats` opponentsChoice
                               then 6
                               else if yourChoice == opponentsChoice
                               then 3
                               else 0

main = interact
     ( lines
   >>> fmap ( words
          >>> \[opponent, instruction] -> let opponentsChoice = read opponent
                                              yourChoice = case instruction of
                                                             "X" -> whatIsBeatenBy opponentsChoice
                                                             "Y" -> opponentsChoice
                                                             "Z" -> whatBeats opponentsChoice
                                           in score opponentsChoice yourChoice
            )
   >>> sum
   >>> show
   >>> (++"\n")
     )
