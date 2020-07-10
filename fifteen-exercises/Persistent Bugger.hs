module Codewars.G.Persistence where

import Data.Char(digitToInt)
persistence :: Int -> Int
persistence n = pcnt n 0
    -- 这个用来记录次数的函数多余了……直接用返回值记录就行……
    where pcnt n cnt = if n < 10 then cnt else pcnt (foldl1 (*) $ map digitToInt $ show n) cnt+1