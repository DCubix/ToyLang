grammar lang;

program: block;

var_def: ID ('=' test)?;
var_list: var_def (',' var_def)*;

block: stmt*;
body: stmt | '{' block '}';

stmt
	:	';'
	|	'break' ';'
	|	'continue' ';'
	|	'return' test ';'
	|	'let' var_list ';'
	|	'if' test body ('else' body)?
	|	'for' ID 'in' (list | range) body
	|	'while' test body
	|	'do' body 'while' test ';'
	|	'func' ID '(' var_list ')' body
	|	test
	;

range
	:	test '..' test
	;

list
	: '[]'
	| '[' arglist ']'
	;

test: or_test ('?' test ':' test)?
	;
or_test: and_test ('||' and_test)*
	;
and_test: not_test ('&&' not_test)*
	;
not_test: '!' not_test | comparison
	;
comparison: expr (op_comp expr)*
	;

op_augassign : '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>=' | '**=' | '=';
op_comp : '<' | '>' | '<=' | '>=' | '!=' | '==';

expr: xor_expr ('|' xor_expr)*
	;
xor_expr: and_expr ('^' and_expr)*
	;
and_expr: shift_expr ('&' shift_expr)*
	;
shift_expr: arith_expr (('<<'|'>>') arith_expr)*
	;
arith_expr: term (('+'|'-') term)*
	;
term: factor (('*'|'/'|'%') factor)*
	;
factor: ('+'|'-'|'~') factor | trail
	;
trail: atom trailer*
	;

atom : ID | NUMBER | STRING | '(' test ')';

trailer
	: '(' (arglist)? ')' | '[' test ']' | '.' ID
	;

arglist
	: test (',' test)*
	;

STRING
	:	'"' ( '\\"' | . )*? '"'
	;

ID
	:	[a-zA-Z_][a-zA-Z0-9]*
	;

HEX
	:	('0x'|'0X'|'#')? [0-9a-zA-Z]+
	;

NUMBER
	:	HEX
	|	[0-9]+
	|	[0-9]* '.' [0-9]+
	;

WS: [ \t\f\r\n] -> skip;
