import subprocess

class Repository:
    def __init__(self, url="", name="", location="~/.moon"):
        self.url = url,
        self.name = name
        self.location = location

    def get_repo(self):
        """Download repository from git url"""

        print("Downloading Repo...\n\n")
        subprocess.run(["git", "clone", self.url, self.location])        

        
