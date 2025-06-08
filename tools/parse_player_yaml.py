import yaml
import csv
import os
import sys
from yaml.loader import SafeLoader

# Define a constructor for custom tags
def tag_constructor(loader, node):
    # Return the tag name without the exclamation mark
    return node.tag[1:]

# Add the constructor to SafeLoader for the custom tags
yaml.add_constructor('!Basic', tag_constructor, Loader=SafeLoader)
yaml.add_constructor('!FoodProducer', tag_constructor, Loader=SafeLoader)


def parse_player_yaml(yaml_file_path, csv_file_path):
    """
    Parses player unit data from a YAML file and writes it to a CSV file.

    Args:
        yaml_file_path (str): The path to the input YAML file.
        csv_file_path (str): The path to the output CSV file.
    """
    try:
        with open(yaml_file_path, 'r') as f:
            player_data = yaml.safe_load(f)
    except FileNotFoundError:
        print(f"Error: YAML file not found at {yaml_file_path}")
        return
    except yaml.YAMLError as e:
        print(f"Error parsing YAML file: {e}")
        return

    # Ensure the directory for the CSV file exists
    output_dir = os.path.dirname(csv_file_path)
    if output_dir and not os.path.exists(output_dir):
        os.makedirs(output_dir)

    try:
        with open(csv_file_path, 'w', newline='') as csvfile:
            csv_writer = csv.writer(csvfile)

            # Write header
            csv_writer.writerow(['name', 'tier', 'health', 'cost', 'description'])

            # Write data for each player unit
            for name, data in player_data.items():
                # Skip commented out units
                if data is None:
                    continue

                tier = data.get('tier', 1) # Default to 1 if tier is missing

                components = data.get('components', {})
                health_list = components.get('health', [0, 0])
                health = f"{health_list[0]}/{health_list[1]}" if isinstance(health_list, list) and len(health_list) == 2 else "N/A"

                cost = components.get('cost', 0)

                description = data.get('description', '')

                csv_writer.writerow([name, tier, health, cost, description])

        print(f"Successfully wrote player unit data to {csv_file_path}")

    except IOError as e:
        print(f"Error writing CSV file: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python parse_player_yaml.py <path_to_player.yaml>")
        sys.exit(1)

    input_yaml = sys.argv[1]
    output_csv = sys.argv[2]
    parse_player_yaml(input_yaml, output_csv)
