---
title: Hello World
timestamp: 2023-10-24T02:44:07.740916Z
language: en-GB
tags:
---

# A Relevant Table

| Hello | World |
|:-----:|:-----:|
| 12323 | 43242 |
| 57439 | 57438 |

# Some Code

## Rust Code

Here is some rust code:

```rs
/// Highlight the contents of a fenced code block of a given source language as HTML
pub fn highlight<'src>(lang: CowStr<'src>, code: CowStr<'src>) -> CowStr<'src> {
    let lang: Language = if let Ok(lang) = lang.parse() {
        lang
    } else {
        return code;
    };
    let config = lang.get_config();

    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(config, code.as_bytes(), None, |_| None)
        .unwrap();

    let mut renderer = HtmlRenderer::new();
    renderer
        .render(highlights, code.as_bytes(), &|highlight| {
            HTML_ATTRS[highlight.0].as_bytes()
        })
        .unwrap();

    CowStr::from(String::from_utf8(renderer.html).unwrap())
}
```

## OCaml Code

Here is some OCaml code:

```ml
open Batteries

let priority ch =
  if Char.is_lowercase ch then Char.code ch - Char.code 'a' + 1
  else Char.code ch - Char.code 'A' + 27

let rec part_one contents =
  get_rucksacks contents
  |> List.map (fun group ->
         let first, second = Tuple2.mapn (Set.of_seq % String.to_seq) group in
         let result = Set.intersect first second |> Set.map priority in
         Set.fold ( + ) result 0)
  |> List.sum |> Printf.printf "Sum: %d\n"

and get_rucksacks contents =
  String.split_on_char '\n' contents
  |> List.map (fun str ->
         let mid = Int.div (String.length str) 2 in
         (String.sub str 0 mid, String.sub str mid mid))

let rec part_two contents =
  String.split_on_char '\n' contents
  |> group_three
  |> List.map (fun group ->
         let first, second, third =
           Tuple3.mapn (Set.of_seq % String.to_seq) group
         in
         Set.intersect first second |> Set.intersect third |> Set.any)
  |> List.map priority |> List.sum |> Printf.printf "Sum: %d\n"

and group_three rucksacks =
  match rucksacks with
  | first :: second :: third :: rest ->
      List.cons (first, second, third) (group_three rest)
  | [] -> []
  | _ -> assert false

let arg = try Some Sys.argv.(1) with Invalid_argument _ -> None

let () =
  let contents = File.with_file_in "input.txt" IO.read_all in
  match arg with
  | Some ("one" | "1") -> part_one contents
  | Some ("two" | "2") -> part_two contents
  | _ -> print_endline "Usage: day_3_rucksack_reorganization <part>"
```

## Haskell Code

Here is some Haskell code:

```hs
module Main where

import Data.Bits (Bits (complement, shiftL, xor))
import Data.Char (ord)
import Data.Foldable (Foldable (foldl'))
import GHC.Float (int2Float)

countDigits :: [Int] -> String -> [Int]
countDigits oneCounts number =
  let zipped = zip oneCounts number
   in map (\(num, digit) -> num + ord digit - 48) zipped

getDigits :: [String] -> [Int]
getDigits (head : tail) =
  foldl' countDigits [ord c - 48 | c <- head] tail
getDigits [] = []

toBin :: Int -> [Int] -> [Int]
toBin len =
  map (\digit -> if int2Float digit > int2Float len / 2 then 1 else 0)

getGamma :: Int -> [Int] -> Int
getGamma len numbers =
  foldl' (\acc digit -> 2 * acc + digit) 0 $ toBin len numbers

getNumbers :: String -> IO [String]
getNumbers fileName = do
  content <- readFile fileName
  return $ lines content

main :: IO ()
main = do
  numbers <- getNumbers "test_input.txt"
  let len = length numbers
      digits = getDigits numbers
      gamma = getGamma len digits
      epsilon = xor gamma $ shiftL 1 (length (head numbers)) - 1
   in print $ gamma * epsilon
```

