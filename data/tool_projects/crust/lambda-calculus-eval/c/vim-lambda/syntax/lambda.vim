" File: ~/.vim/syntax/lambda.vim


" Syntax file for Lambda Calculus

if exists("b:current_syntax")
  finish
endif

let b:current_syntax = "lambda"

syn keyword lambdaDef def

syn match lambdaVariable "\<[a-z]\+\>"

syn match lambdaAbstraction "@[a-z]\+\."

syn match lambdaApplication "(.\+ .\+)"

syn match lambdaGrouping "[(|)]"

hi lambdaVariable     ctermfg=Blue guifg=Blue
hi lambdaAbstraction  ctermfg=Green guifg=Green
hi lambdaApplication  ctermfg=Cyan guifg=Cyan
hi lambdaGrouping     ctermfg=DarkRed guifg=DarkRed
hi lambdaDef	      ctermfg=Yellow guifg=Yellow
hi def link lambdaLambda Keyword
hi def link lambdaVariable Identifier
hi def link lambdaAbstraction Function
hi def link lambdaApplication Statement
hi def link lambdaGrouping Delimiter
