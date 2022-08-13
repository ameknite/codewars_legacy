module CountSheep where

import Text.Printf

countSheep :: Int -> String
countSheep n = [1 .. n] >>= printf "%d sheep..."
