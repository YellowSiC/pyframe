import os
import re

# Datei, die angepasst
filename = r"c:\Users\MalekAli\Desktop\pyframe\pyframe\handler\window.py"


# Funktion zum Konvertieren von CamelCase in snake_case
def camel_to_snake(name: str) -> str:
    s1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


# Datei lesen
with open(filename, "r", encoding="utf-8") as f:
    content = f.read()

# Regex zum Finden der Methoden
pattern = re.compile(r"(async def )(\w+)(\s*\()")


# Umwandlung aller CamelCase-Methodennamen in snake_case
def replace_func(match):
    prefix = match.group(1)
    func_name = match.group(2)
    suffix = match.group(3)
    snake_name = camel_to_snake(func_name)
    return f"{prefix}{snake_name}{suffix}"


# Anpassungen vornehmen
new_content = pattern.sub(replace_func, content)

# Datei sichern (Backup)
backup_filename = filename + ".bak"
os.rename(filename, backup_filename)
print(f"Backup gespeichert unter: {backup_filename}")

# Neue Datei speichern
with open(filename, "w", encoding="utf-8") as f:
    f.write(new_content)

print("Funktionennamen erfolgreich in snake_case umgewandelt! 🚀")
