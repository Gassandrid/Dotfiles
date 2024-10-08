import os

# get the username
username = os.getenv("USER")

# print the greeting
print(f"Hello, this is {username} using Vim on my Raspberry Pi")


for i in range(1, 10):
    print(i)
