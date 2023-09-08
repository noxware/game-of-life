import random

def generate_random_pattern(n):
    """Generate a random n x n Game of Life pattern in plaintext format."""
    pattern = "!Name: Random {}x{} Pattern\n".format(n, n)
    
    for _ in range(n):
        for _ in range(n):
            pattern += 'O' if random.randint(0, 1) == 1 else '.'
        pattern += '\n'
    
    return pattern

if __name__ == "__main__":
    n = 200
    print(generate_random_pattern(n))
