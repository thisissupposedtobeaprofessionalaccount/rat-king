# Rat King

## What is it ?

This is the server working with Cute Rat project, put instruction inside a instruction.rk file and the server will provide theses instructions to the next connected rat.

## How to use it ?

Once you have build the project, you can run the server.

You will need to provide a file called instructions.rk that can contains 2 types of instructions:
- cmd (command): a command for the rat to execute
- set (setting): a setting for the rat to apply

An exemple of instructions.rk file could be:
```txt
cmd ls -la / <--- This will make the rat execute the command "ls -la /"
set period 10 h <--- This will change the time between each instruction to 10 hours
set server 127.0.0.1 7777 <--- This will change the server address to localhost:7777
```
