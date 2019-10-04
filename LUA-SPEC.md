# Spec Progress
This is the tracking document that shows what has been implemented for the LUA 5.1 spec. This is also mirrored in the actual code comments but here for easy reference. 

- [x] chunk
- [x] block
- [ ] stat
    - [x] varlist `=´ explist | 
    - [ ] functioncall | 
    - [x] do block end | 
    - [ ] while exp do block end | 
    - [ ] repeat block until exp | 
    - [ ] if exp then block {elseif exp then block} [else block] end | 
    - [ ] for Name `=´ exp `,´ exp [`,´ exp] do block end | 
    - [ ] for namelist in explist do block end | 
    - [ ] function funcname funcbody | 
    - [ ] local function Name funcbody | 
    - [x] local namelist [`=´ explist] 
- [ ] laststat
- [ ] funcname
- [x] varlist
- [x] var
    - [x] Name
    - [x] prefixexp `[´ exp `]´
    - [x] prefixexp `.´ Name 
- [x] namelist
- [x] explist
- [ ] exp
    - [x] nil
    - [x] false
    - [x] true
    - [x] Number
    - [ ] String
    - [x] `...´
    - [ ] function
    - [x] prefixexp
    - [x] tableconstructor
    - [x] exp binop exp
    - [x] unop exp 
- [ ] prefixexp
    - [x] var
    - [ ] functioncall
    - [x] `(´ exp `)´
- [ ] functioncall 
    - [ ] prefixexp args
    - [ ] prefixexp `:´ Name args 
- [ ] args
    - [ ] `(´ [explist] `)´
    - [ ] tableconstructor
    - [ ] String 
- [ ] function
- [ ] funcbody
- [ ] parlist
    - [ ] namelist [`,´ `...´]
    - [ ] `...´
- [x] tableconstructor
- [x] fieldlist
- [x] field
    - [x] `[´ exp `]´ `=´ exp
    - [x] Name `=´ exp
    - [x] exp

# lua syntax
The complete `lua` synatax per the Lua [5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html#8)

```
chunk ::= {stat [`;´]} [laststat [`;´]]

block ::= chunk

stat ::=  varlist `=´ explist | 
        functioncall | 
        do block end | 
        while exp do block end | 
        repeat block until exp | 
        if exp then block {elseif exp then block} [else block] end | 
        for Name `=´ exp `,´ exp [`,´ exp] do block end | 
        for namelist in explist do block end | 
        function funcname funcbody | 
        local function Name funcbody | 
        local namelist [`=´ explist] 

laststat ::= return [explist] | break

funcname ::= Name {`.´ Name} [`:´ Name]

varlist ::= var {`,´ var}

var ::=  Name | prefixexp `[´ exp `]´ | prefixexp `.´ Name 

namelist ::= Name {`,´ Name}

explist ::= {exp `,´} exp

exp ::=  nil | false | true | Number | String | `...´ | function | 
        prefixexp | tableconstructor | exp binop exp | unop exp 

prefixexp ::= var | functioncall | `(´ exp `)´

functioncall ::=  prefixexp args | prefixexp `:´ Name args 

args ::=  `(´ [explist] `)´ | tableconstructor | String 

function ::= function funcbody

funcbody ::= `(´ [parlist] `)´ block end

parlist ::= namelist [`,´ `...´] | `...´

tableconstructor ::= `{´ [fieldlist] `}´

fieldlist ::= field {fieldsep field} [fieldsep]

field ::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp

fieldsep ::= `,´ | `;´

binop ::= `+´ | `-´ | `*´ | `/´ | `^´ | `%´ | `..´ | 
        `<´ | `<=´ | `>´ | `>=´ | `==´ | `~=´ | 
        and | or

unop ::= `-´ | not | `#´
```