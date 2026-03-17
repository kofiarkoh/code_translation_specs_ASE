from pathlib import Path

folder_path = input("Enter parent folder path: ").strip()
parent_folder = Path(folder_path)

if not parent_folder.exists() or not parent_folder.is_dir():
    print("Invalid folder path.")
else:
    for subfolder in parent_folder.iterdir():
        if subfolder.is_dir():
            files = [
                item.name
                for item in subfolder.iterdir()
                if item.is_file()
            ]

            if len(files) > 2:
                print(f"{subfolder.name} -> {len(files)} files")
                print("Counted files:", files)
                print()