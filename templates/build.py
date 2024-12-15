import os
from bs4 import BeautifulSoup
from jinja2 import Environment, FileSystemLoader


TEMPLATE_DIR = "emails"
OUTPUT_DIR = "build"


def minify(html: str) -> str:
    soup = BeautifulSoup(html, "html.parser")
    return " ".join(soup.prettify().split())


def build():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    env = Environment(loader=FileSystemLoader("."))

    for file in os.listdir(TEMPLATE_DIR):
        if not file.endswith(".jinja"):
            continue

        filepath = os.path.join(TEMPLATE_DIR, file)
        filename = file.split(".")[0]

        template = env.get_template(filepath)
        html = minify(template.render())

        output_file = os.path.join(OUTPUT_DIR, f"{filename}.html")
        with open(output_file, "w", encoding="utf-8") as f:
            f.write(html)

    print("Successfully rendered email templates.")


if __name__ == "__main__":
    build()
