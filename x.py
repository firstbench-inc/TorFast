def split_file(input_file, output_prefix, lines_per_file):
    with open(input_file, 'r') as file:
        lines = file.readlines()
    
    total_lines = len(lines)
    num_files = total_lines // lines_per_file

    for i in range(num_files):
        start_line = i * lines_per_file
        end_line = start_line + lines_per_file
        output_file = f"{output_prefix}_{i + 1}.txt"
        
        with open(output_file, 'w') as output:
            output.writelines(lines[start_line:end_line])
    
    print(f"File split into {num_files} files of {lines_per_file} lines each.")

# Define input parameters
input_file = 'urls.txt'
output_prefix = 'a'
lines_per_file = 750

# Run the function
split_file(input_file, output_prefix, lines_per_file)

