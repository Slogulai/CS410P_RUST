# Rust Web
CS410p Rust Web Dev Repo

Christopher Sloggett
sloggett@pdx.edu

This Repo will include all the work and assignments done by Christopher Sloggett

This Repo contains the code that will be used to access assignments from the class 
CS410 Rust Web Development. All code subject to grading will be under the main branch. 
The three other branches are for testing either Bart's code or implementations of my 
own code. All work that I intend to submit will be merged with main.

List of Branches:
1.) main - All code that is subject to grading will be here
2.) bart_branch - All code that is for testing Bart's code
3.) testing_branch - Branch for testing new implementations
4.) testing_branch2 - Another branch for testing other implementations
5.) in_mem_branch - Branch for saving the work of the in memory database

THIS PROGRAM WILL NOT COMPILE WITHOUT A SQL DOCKER IMAGE

This repo consists of two folders, knock-knock-yew and rust-web-axum. rust-web-axum 
is the docker server I have worked on over this term, spring 2024. To get a database 
started in this folder, first cd into it, then run the following two commands:
    docker compose up --build -d
    sqlx migrate run
Docker must be installed as well as the sqlx-cli which can be added with cargo: 
cargo add sqlx-cli. After these have been ran, go ahead and cargo run within this 
directory and then move into the knock-knock-yew directory. 

Once in the knock-knock-yew directory, ensure that trunk is installed to allow the 
yew formatting to run. After doing so, type trunk serve into the command line to let 
the program run. You can then navigate to 127.0.0.1:8080 where you can then test the 
functionality of yew and the database. 

Hope that this readme was helpful and that you enjoy the layout of the repo!

~~_~_~_~_~_~ HANDY CARGO COMMANDS _~_~_~_~_~_~~
cargo watch -q -c -w src/ -x run (Will watch main.rs for changes and run again when 
changes are made)

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

^^Don't forget to use git status often!

Deleting Branches after merging - optional
1.) git branch --merged (shows which branches have been merged, safety check)
2.) git branch -d <name of branch> (delete the branch)

1.) git log --graph --all --decorate --oneline 
2.) git checkout 
3.) git log