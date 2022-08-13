module HexToDecimal where

hexChar :: Char -> Int
hexChar '1' = 1
hexChar '2' = 2
hexChar '3' = 3
hexChar '4' = 4
hexChar '5' = 5
hexChar '6' = 6
hexChar '7' = 7
hexChar '8' = 8
hexChar '9' = 9
hexChar 'a' = 10
hexChar 'b' = 11
hexChar 'c' = 12
hexChar 'd' = 13
hexChar 'e' = 14
hexChar 'f' = 15
hexChar _ = 0

hexToDec :: String -> Int
hexToDec [] = 0
hexToDec s = hexChar (last s) + (16 * hexToDec (init s))

-- module HexToDecimal where

-- import Numeric (readHex)

-- hexToDec :: String -> Int
-- hexToDec = fst . head . readHex
