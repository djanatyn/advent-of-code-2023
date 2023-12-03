{-# LANGUAGE NoImplicitPrelude, LambdaCase #-}

module Day02 where

import Relude
import Test.Tasty
import Test.Tasty.HUnit
import Control.Exception (catch)
import System.Exit (ExitCode(ExitSuccess))

main :: IO ()
main = do
  catch (defaultMain tests) $ \case {
    ExitSuccess -> solve;
    e           -> exitFailure
}

solve :: IO ()
solve = do
  file <- readFile "./src/input/day02/input.txt"
  undefined

tests :: TestTree
tests = testGroup "day02"
  [ testCase "example 01" $ do
      input <- readFileText "./src/input/day02/example01.txt"
      putTextLn input
  ]
