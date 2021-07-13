import moon

def delete_package(*args):
    return "Deleting binary package..."


def handle_args(**kwargs):
    if kwargs["ask"]:
        confirm = input('\nEnter "yes" or "y" to proceed: ')

        if confirm in ("y", "yes"):
            pass
        else:
            quit()

    if kwargs["remove"]:
        return delete_package()
    else:
        package = moon.Package(name="st", source="asdf", file="tar.gz", format="asdf", version="4.0.5", binaries=["st"], dependencies=["x11"], steps=["hello", "hello"])
        print(package)
        return moon.find_path(kwargs["package"], kwargs["version"])




        


