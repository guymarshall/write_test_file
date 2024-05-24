import random
import string


def generate_random_data(size_gb):
    billion_in_bytes = 1024 ** 3
    size_bytes = size_gb * billion_in_bytes

    random_data = "".join(random.choices(string.ascii_letters + string.digits, k=1024))

    random_data_length = len(random_data)

    with open("random_data.txt", "w") as file:
        bytes_written = 0
        while bytes_written < size_bytes:
            file.write(random_data)
            bytes_written += random_data_length
            if bytes_written % billion_in_bytes == 0:
                print(f"{bytes_written / billion_in_bytes} GB written")


def main():
    user_input = int(input("Number of gigabytes to generate: "))
    generate_random_data(user_input)

if __name__ == "__main__":
    main()
