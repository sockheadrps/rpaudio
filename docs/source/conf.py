# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'rpaudio'
copyright = '2024, beaux44, sockheadrps'
author = 'beaux44, sockheadrps'
release = '0.0.1'


napoleon_google_docstring = True
napoleon_include_private_with_doc = False
napoleon_use_admonition_for_examples = False
napoleon_preprocess_types = True

napoleon_use_ivar = True
napoleon_use_param = True
napoleon_use_rtype = True

extensions = [
    'autoapi.extension',
    'sphinx.ext.napoleon',
    "sphinx.ext.viewcode",
    'sphinx_autodoc_typehints',
]

html_theme = 'sphinx_rtd_theme'
templates_path = ['_templates']


# AutoAPI configuration
autoapi_dirs = ['../../']  
autoapi_generate_api_docs = True
autoapi_root = 'autoapi'
autoapi_type = 'python'
autodoc_typehints = 'description'
autoapi_file_patterns = ['*.pyi']
autoapi_ignore = ['__init__.py', '__init__.pyi', 'asynctest.py', 'threadtest.py', 'docs\source\conf.py']
autoapi_keep_files = True  # To keep the generated _autoapi files
autoapi_member_order = 'groupwise'
autoapi_options = [
    'members',
    'undoc-members',
    'show-inheritance',
]
