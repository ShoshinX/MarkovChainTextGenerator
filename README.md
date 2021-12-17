# Markov Chain Text Generator
A text generator to have fun given the source material in dataset.
The point of this exercise is to practice in simplifying code such that it is cache-friendly and simple to read.

Cach-friendliness of the program:
    - no extra allocations of new string objects
    - cache-friendly implementation of hashmap that rust ports from google's seleina

Currently the program is single threaded.

Plans are:
    - Create benchmarks
    - Measure current performance
    - Implement moving window markov
    - Implement other versions of markov chain generator as seen in the table below
    - Use bigger dataset to test the below table implementations

|Threadedness/Chonkness| Stream| Chonk|
|----------------------|-------|------|
|Single                | No    | Yes  |
|Multi                 | No    | No   |
