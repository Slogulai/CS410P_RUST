# Rust Web
CS410p Rust Web Dev Repo

Christopher Sloggett
sloggett@pdx.edu

This Repo will include all the work and assignments done by Christopher Sloggett

This Repo has the main directory that will be used to access assignments
from the class CS410 Rust Web Development. All code subject to grading will
be under the main branch. The two other branchs are for testing either barts code
or implementations of my own code. All work that I intend to submit from testing
branch will be merged with main.

rust-web-axum is where the term project is located. Sorry for making you cd one 
directory down, git can be fun like that. To run the program, cd into
the rust-web-axum directory and enter 'cargo run' into the command line.
From a web browser go to 127.0.0.1:3000 for the home page of the server. 
The different routes available can be found below: 

127.0.0.1:3000 - Home Page
127.0.0.1:3000/question - Random question
127.0.0.1:3000/questions - All questions
127.0.0.1:3000/question/{id} - Specific question. Enter a number in the id field
127.0.0.1:3000/add_question - Add a question to the database
127.0.0.1:3000/edit_question/{id} - Edit a question. Enter a number in the id field

The current version of this program uses a JSON file for a persistent database. When
the program is run, the JSON file is read and the data is stored in a HashMap. The HashMap
is then updated with any CRUD operations that are performed and the whole Hashmap is immediately
written to the JSON file. This is a taxing yet simple way of creating persistence and the program 
will be changed to use a SQLx database in the future. 

Hope that this read me was helpful and that you enjoy the layout of the repo!

~~_~_~_~_~_~ HANDY CARGO COMMANDS _~_~_~_~_~_~~
cargo watch -q -c -w src/ -x run (Will watch main.rs for changes and run again when changes are made)

~~_~_~_~_~_~ HANDY GIT COMMANDS _~_~_~_~_~_~~

-When working with branches-
1.) git branch <chris_branch1> 
2.) git checkout <chris_branch1> 
3.) ~you can begin editing and making changes~
4.) git status 
5.) git add <filenames> 
6.) git commit -a -m "Reasons for the Commit" (commit all changes and write a reason)
7.) git checkout main 
8.) git merge <name of branch> 
9.) git push 

^^Dont forget to use git status often!

Deleting Branches after merging - optional
1.) git branch --merged (shows which branches have been merged, safety check)
2.) git branch -d <name of branch> (delete the branch)

1.) git log --graph --all --decorate --oneline 
2.) git checkout 
3.) git log
