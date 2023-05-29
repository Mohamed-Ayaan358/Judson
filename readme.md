## Two questions before we start off :-
   - ###  Do you have multiple web projects taking up space on your system with their bulky modules?

   - ###  Frustrated as you dont know which one you have to delete because you dont remember which one was last modified?

## Well Judson might just help you there !

----

#### A. Written in rust and uses several libraries to allow you to:- 

     A. Dictate which folders you wish to target 
     B. Removes the node_modules from the directories
     C. Replaces the folder with a zipped version of it after it has been 15 days



#### B. Commands to get it working

```   
git clone git@github.com:Mohamed-Ayaan358/Judson.git
```

```
cd Judson
```

```
sh run.sh
```
##### The below commands can be used if you wish for it to be automated at 11pm everyday
```
export EDITOR=vi
crontab-e
```

```
00 23 * * * judson > /dev/null 2>&1

```

#### C. Remaining Tasks to accomplish
- [ ] Make it so the user can set up the entire config with one command

- [ ] Allow for users to modify the number of days they wish to keep the project, via a command

- [ ] Improve the zipping procedure as well as potentially adding parallelism


----

<!-- The main issues with the project was that the UNIX file system in of itself is quite strange.

DIAG

This has been resolved with my own system of checking if a file has been modified anywhere in the main project directory.

I will also have to search for better methods of zipping the directories as I am aiming for a sub 5-10 minute process for any root directory of any size. -->


