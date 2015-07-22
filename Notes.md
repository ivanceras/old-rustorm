How to properly construct builder pattern in rust

https://aturon.github.io/ownership/builders.html




##Postgres specific foreign key
*This is needed because information_schema is a lot slower

http://stackoverflow.com/questions/1152260/postgres-sql-to-list-table-foreign-keys

```sql
SELECT
  o.conname AS constraint_name,
  (SELECT nspname FROM pg_namespace WHERE oid=m.relnamespace) AS source_schema,
  m.relname AS source_table,
  (SELECT a.attname FROM pg_attribute a WHERE a.attrelid = m.oid AND a.attnum = o.conkey[1] AND a.attisdropped = false) AS source_column,
  (SELECT nspname FROM pg_namespace WHERE oid=f.relnamespace) AS target_schema,
  f.relname AS target_table,
  (SELECT a.attname FROM pg_attribute a WHERE a.attrelid = f.oid AND a.attnum = o.confkey[1] AND a.attisdropped = false) AS target_column
FROM
  pg_constraint o LEFT JOIN pg_class c ON c.oid = o.conrelid
  LEFT JOIN pg_class f ON f.oid = o.confrelid LEFT JOIN pg_class m ON m.oid = o.conrelid
WHERE
  o.contype = 'f' AND o.conrelid IN (SELECT oid FROM pg_class c WHERE c.relkind = 'r');
 
```  
  
##Show tables in postgresql

```sql
select 
  tablename as table 
from 
  pg_tables  
where schemaname = 'public'
```


A simple Good resource for reading about lifetimes
http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html


##Select the parent table
```sql
select relname,  
( select relname from pg_class where oid = pg_inherits.inhparent ) as parent 
					 from pg_class  
					 left join pg_inherits  
					 on pg_class.oid = pg_inherits.inhrelid 
					 where relname = 'product'
```


##Select the subclass

```sql
select relname, 
( select relname from pg_class where oid = pg_inherits.inhrelid ) as subclass 
					 from pg_inherits 
					 left join pg_class on pg_class.oid = pg_inherits.inhparent 
					 where relname = 'base' ;
```


## Useful projects:

### text editor
https://github.com/gchp/iota

## csv parsing
https://github.com/BurntSushi/rust-csv

## window tiling for the win
https://github.com/Kintaro/wtftw

## for text base interface, a wrapper for termbox

https://github.com/gchp/rustbox


##How to deal with nullable columns in the database
*Most database columns has null values, can be optional
*If nullable make the field Optional


## Check to see if extension are already installed in the database
select * from pg_extension

You may need to create the schema before installing the extensions


## releasing:
```
cargo publish
```

Make sure license, github code, documentation is properly linked in the Cargo.toml file

## Publishing the documents

```
cargo clean
cargo doc --no-deps
cd target/doc
git init
git add . -A
git commit -m "Commiting docs to github pages"
git remote add origin https://github.com/ivanceras/rustorm
git checkout -b gh-pages
git push --force origin gh-pages


```

## Using vim coding rust
* Install pathogen

```sh

mkdir -p ~/.vim/autoload ~/.vim/bundle && curl -LSso ~/.vim/autoload/pathogen.vim https://tpo.pe/pathogen.vim

```
* Add this to ~/.vimrc

```sh
execute pathogen#infect()
syntax on
filetype plugin indent on

```


* Install syntax highlighting in rust

```sh

cd ~/.vim/bundle
git clone https://github.com/rust-lang/rust.vim.git

``` 

* Install vim racer plugin

```sh
cd .vim/bundle/
git clone https://github.com/ebfe/vim-racer

```

* Add this to ~/.vimrc
```
set hidden
let g:racer_cmd = "/home/lee/Developer/racer/target/release/racer"
let $RUST_SRC_PATH="/home/lee/Developer/rust-1.0.0/src"

```

* Vim number toggle

```sh

cd ~/.vim/bundle
git clone git://github.com/jeffkreeftmeijer/vim-numbertoggle.git

```
* Install nerdtree for displaying files in a tabs

```sh
cd ~/.vim/bundle
git clone https://github.com/scrooloose/nerdtree.git

```
* Add this to ~/.vimrc
```sh
autocmd VimEnter * NERDTree
autocmd BufEnter * NERDTreeMirror

"CTRL-N to toggle tree view with CTRL-N
nmap <silent> <c-n> :NERDTreeToggle<CR>		
"Set F2 to put the cursor to the nerdtree
nmap <silent> <F2> :NERDTreeFind<CR>

```

You can use <CTRL-W><CTRL-W> to switch in between windows

vim cheatsheet at http://vim.rtorr.com/


## Screen capturing the desktop

```sh
sudo apt-get install byzanz
```

Install ScreenRuler/Kruler as well

Record using
```sh
byzanz-record --duration=30 --x=2 --y=50 --width=1095 --height=595 out.gif
byzanz-record --duration=15 --x=2 --y=50 --width=1095 --height=595 out.gif
```

put the window to the top of the OS toolbar, this is 50px including the window of the terminal
put a margin of around 2px from the side 

## git compare file changes
git config diff.tool vimdiff
git difftool


## Count the number of lines in git project
git ls-files | xargs wc -l

## Show postgresql version 
https://blog.sleeplessbeastie.eu/2014/04/04/how-to-remotely-check-postgresql-version/

SHOW server_version;
select version();


## Uuid
* uuid has 16 bytes  -> (128 bits/8) 
* 32 characters (hex digit)
* 36 when including hyphens
* 22 character when encododed to base64

gotcha: need to put & to borrow immutable from mutable
```
fn execute_with_return(&mut self, query:&Query)->DaoResult{
        let sql_frag = &self.build_query(query);
```