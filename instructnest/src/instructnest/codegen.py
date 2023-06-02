from pathlib import Path
from typing import Any
from jinja2 import Template

class jinja_template:
    def __init__(self, func):
        self.func = func
        self.attrname = None
        self.__doc__ = func.__doc__

    def __set_name__(self, owner, name):
        self.attrname = name

    def __get__(self, obj, objtype=None):
        template_dir = getattr(obj, "template_dir", Path(__file__).parent.joinpath("templates"))
        template_name: str = self.func(obj) + ".jinja"
        template_path = template_dir.joinpath(*template_name.split("/"))
        template_cont = template_path.read_text()
        templete_obj = Template(template_cont, trim_blocks=True, lstrip_blocks=True)
        return templete_obj

    def __call__(self, obj):
        return self.__get__(obj, type(obj))
