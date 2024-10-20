from flask import Flask, request, jsonify
import subprocess
from dotenv import load_dotenv
import os,re

# Load environment variables from .env file
load_dotenv()

app = Flask(__name__)

# Path to your Rust binary
BINARY_PATH = os.getenv('BINARY_PATH')  # Update this to the correct binary path

# Helper function to run the Rust binary with provided arguments
def run_command(args):
    try:
        # Start the Rust binary process
        process = subprocess.Popen(
            [BINARY_PATH, ] + args, 
            stdin=subprocess.PIPE,  # Allow input to stdin
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        # Send the password to the process stdin
        stdout, _ = process.communicate()
        

        return stdout
        
    except Exception as e:
        return {"error": str(e)}

@app.route('/get_items', methods=['GET'])
def run():
    results = []
    try:
        i = 0
        
        while True:
            result = run_command(args=["view-item-details" , str(i), "https://bigbangblock.builders", "https://bigbangblock.builders" ,"../pint/token"])
            if "None" in result: 
                break
            
            # Use regex to extract the values
            item_id_match = re.search(r'item_id:\s*(\d+)', result)
            cost_match = re.search(r'Cost:\s*(\d+)', result)
            owner_match = re.search(r'Owner:\s*"([^"]+)"', result)
            active_match = re.search(r'Active:\s*\'([^"]+)\'', result)

            # Store them in a dictionary
            result = {
                "item_id": item_id_match.group(1) if item_id_match else None,
                "cost": cost_match.group(1) if cost_match else None,
                "owner": owner_match.group(1) if owner_match else None,
                "active": active_match.group(1) if active_match else None
            }
            results.append(result)
            i+=1
        
        return jsonify(results)
    except Exception as e:
        return jsonify({"error": str(e)}), 400

if __name__ == '__main__':
    app.run(debug=True)