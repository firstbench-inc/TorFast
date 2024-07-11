import tldextract

# Function to extract the domain from a Tor URL
def extract_domain(url):
    ext = tldextract.extract(url)
    return f"{ext.domain}.{ext.suffix}"

# Read the text file containing Tor URLs
input_file_path = "test.txt"
output_file_path = "output_file.txt"

with open(input_file_path, 'r') as file:
    lines = file.readlines()

# Extract unique domains
domains = set()
for line in lines:
    url = line.strip()
    if url:  # Check if line is not empty
        domain = extract_domain(url)
        domains.add(domain)

# Write unique domains to the output file
with open(output_file_path, 'w') as file:
    for domain in sorted(domains):  # Sorting is optional
        file.write(domain + '\n')

print(f"Number of unique Tor domains: {len(domains)}")
print(f"Unique domains have been written to {output_file_path}")
