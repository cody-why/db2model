import subprocess
import shutil
import os

# 功能: 运行sql_reverse生成文件，将生成的文件复制到指定目录，并修改mod.rs文件
# 使用: python reverse.py

def run_sql_reverse(template, suffix="", destination_dir="", source_dir="generated"):
    # sql_reverse mysql -f mytera/reverse.yml -p 'mytera/*' -n rbatis.tera -c mytera/mysql_rbatis.json
    command = [
        "sql_reverse",
        "mysql",
        "-f", "mytera/reverse.yml",
        "-p", "mytera/*",
        "-n", f"{template}.tera",
        "-c", "mytera/mysql_rbatis.json"
    ]
    
    try:
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        print(f"Command for {template} executed successfully.")
        print("Output:", result.stdout)
        
        # Ensure the destination directory exists
        os.makedirs(destination_dir, exist_ok=True)
        
        # Copy all files from source to destination
        for file in os.listdir(source_dir):
            if file.endswith(".rs"):
                # Check if the file is mod.rs
                if file == "mod.rs":
                    # Read the content of mod.rs
                    with open(os.path.join(source_dir, file), 'r') as f:
                        content = f.readlines()
                    
                    # Modify each line
                    modified_content = []
                    for line in content:
                        if line.strip().startswith("pub mod"):
                            # Extract the module name
                            module_name = line.strip().split()[2].rstrip(';')
                            # Modify the line
                            modified_line = f"pub mod {module_name}{suffix};\n"
                            modified_content.append(modified_line)
                        else:
                            modified_content.append(line)
                    
                    # Write the modified content to the destination file
                    with open(os.path.join(source_dir, file), 'w') as f:
                        f.writelines(modified_content)
                    
                source_file = os.path.join(source_dir, file)
                new_file_name = f"{os.path.splitext(file)[0]}{suffix}.rs"
                destination_file = os.path.join(destination_dir, new_file_name)
                if os.path.isfile(source_file):
                    shutil.copy2(source_file, destination_file)
                    print(f"Copied and renamed {file} to {destination_file}")
                    # Delete the source file after copying
                    os.remove(source_file)
        
        print(f"All generated files have been copied to {destination_dir} and deleted from {source_dir}")
       
    except subprocess.CalledProcessError as e:
        print("An error occurred while executing the command:")
        print("Error output:", e.stderr)
    except Exception as e:
        print(f"An error occurred while processing files: {str(e)}")

if __name__ == "__main__":
    run_sql_reverse("model","", destination_dir="src/model")
    run_sql_reverse("handler", "_handler", destination_dir="src/handler")
    run_sql_reverse("service", "_service", destination_dir="src/service")
    run_sql_reverse("vo", "_vo", destination_dir="src/vo")



