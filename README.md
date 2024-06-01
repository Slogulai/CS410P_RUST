# Rust Web
CS410p Rust Web Dev Repo

Christopher Sloggett
sloggett@pdx.edu

This Repo will include all the work and assignments done by Christopher Sloggett

This Repo contains the code that will be used to access assignments
from the class CS410 Rust Web Development. All code subject to grading will
be under the main branch. The three other branchs are for testing either barts code
or implementations of my own code. All work that I intend to submit will be merged with main.

List of Branches:
1.) main - All code that is subject to grading will be here
2.) bart_branch - All code that is for testing Bart's code
3.) testing_branch - Branch for testing new implementations
4>) testing_branch2 - Another branch for testing other implementations
5.) in_mem_branch - Branch for saving the work of the in memory database

rust-web-axum is folder where the term project is located, make sure to 
cd into that directory before testing in these direcions. To run the program, 
Docker is required to spin up a new container as well as the sqlx cli. Install 
docker desktop and to install the sqlx cli run 'cargo install sqlx-cli' on the
command line. Once you have done so run 'docker compose up --build -d' after the 
container has spun up then also run 'sqlx migrate run' to start the db. Cargo check and
cargo run WILL NOT WORK unless a docker container is running and you have done
the sqlx migrate run command, otherwise the program will fail cargo check since there
is no database runnning to connect to. 

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
