# 200GB Challenge

## Problem

Given a file of approximately 200GB, where each line is a string of characters no longer than 1KB. In this file, there are **exactly two duplicate** strings. You are provided with a computer that has 16GB of memory. Find a solution to identify the duplicate string as quickly as possible.

## Generate challenge

Run rust script to generate:

```bash
cargo run --release <seed> <max_line> <size_in_gb> <output>
# `seed` - random seed in unsigned integer for data generation
# `max_line` - maximum size of each line in bytes
# `size_in_gb` - total file size in GB
# `output` - path to the output file

# example:
#                seed line size_in_gb output
cargo run --release 1 1024 1 random_text_file.txt
```

## Submission

To submit your solution, follow these steps:

1. Fork repository to your own GitHub account
2. Create your submission folder: In your forked repository, create new directory under `submission/<your_name/github_username>`
3. Create README (optional): Include a README file in your submission folder with a brief description of your approach, or instructions on how to run your solution
4. Create Pull Request

<samp>
From WeBuild with 💖

We look forward to your innovative solutions! Happy coding!
</samp>
