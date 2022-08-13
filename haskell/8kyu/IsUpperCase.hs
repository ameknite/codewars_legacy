module Codewars.Kata.IsUpperCase where

import Data.Char (isLower)

isUpperCase :: String -> Bool
isUpperCase = not . any isLower
