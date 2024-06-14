import random
import string

def generate_random_line(counter, max_size):
    # Generates a random line of text with a maximum size of `max_size` bytes
    base_length = max_size - len(str(counter)) - 1  # Ensure space for the counter and newline
    line_length = random.randint(1, base_length)
    line = ''.join(random.choices(string.ascii_letters + string.digits + string.punctuation + ' ', k=line_length))
    return f'{line}{counter}\n'

def generate_text_with_offsets(seed, file_path, total_size_gb, max_line_size):
    random.seed(seed)
    total_size_bytes = total_size_gb * 1024**3

    # Generate two random offsets between 0 and total_size_bytes
    offset1 = random.randint(0, total_size_bytes)
    offset2 = random.randint(0, total_size_bytes)

    # Ensure offset1 is the smaller one
    offset1, offset2 = min(offset1, offset2), max(offset1, offset2)

    generated_size = 0
    counter = 0
    recorded_line = None
    inserted = False

    with open(file_path, 'w', encoding='utf-8') as f:
        while generated_size < total_size_bytes:
            line = generate_random_line(counter, max_line_size)

            if not inserted and generated_size >= offset2:
                f.write(recorded_line)
                generated_size += len(recorded_line.encode('utf-8'))
                inserted = True
                print(f'Inserted recorded line at offset {offset2}: {recorded_line.strip()}')

            f.write(line)
            generated_size += len(line.encode('utf-8'))  # Update size with the actual byte size of the line

            if generated_size >= offset1 and recorded_line is None:
                recorded_line = line
                print(f'Recorded line at offset {offset1}: {recorded_line.strip()}')

            counter += 1

    print(f'Generated text file: {file_path}, Size: {total_size_bytes / 1024**3} GB')
    return recorded_line.strip()

# User input for seed, line size, total size in GB, and file path
seed_input = int(input("Enter the seed: "))
line_size_input = int(input("Enter the maximum line size in bytes: "))
total_size_gb_input = int(input("Enter the total file size in GB: "))
file_path_input = 'random_text_file.txt'

# Generate the text file and get the recorded line data
recorded_line_data = generate_text_with_offsets(seed_input, file_path_input, total_size_gb_input, line_size_input)
print(f'Recorded line data: {recorded_line_data}')
