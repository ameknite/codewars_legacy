module Codewars.Kata.Negative where

makeNegative :: (Num a, Ord a) => a -> a
makeNegative x
  | x < 0 = x
  | otherwise = - x
