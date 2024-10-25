# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
import os
import sys
sys.path.insert(0, os.path.abspath('../../python/rpaudio')) 
project = 'rpaudio'
copyright = '2024, beaux44, sockheadrps'
author = 'beaux44, sockheadrps'
release = '0.0.16'

sys.path.insert(0, os.path.abspath('sphinxext'))




# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (amed 'sphinx.ext.*') or your custom
# ones.
extensions = [
    'autoapi.extension',
    'sphinx.ext.autodoc',
    'sphinx.ext.doctest',
    'sphinx.ext.coverage',
    'sphinx.ext.mathjax',
    'sphinx.ext.autosummary',
    'sphinx.ext.intersphinx',
    'numpydoc',
    'sphinx_copybutton',
    'sphinx_issues',
    'sphinx_design',
]

# Add any paths that contain templates here, relative to this directory.
templates_path = ['_templates']

# The root document.
root_doc = 'index'

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = ['_build', 'docstrings', 'nextgen', 'Thumbs.db', '.DS_Store']

# The reST default role (used for this markup: `text`) to use for all documents.
default_role = 'literal'

# Generate the API documentation when building
autosummary_generate = True
numpydoc_show_class_members = False

# Sphinx-issues configuration
issues_github_path = 'sockheadrps/rpaudio'

# Include the example source for plots in API docs
plot_include_source = True
plot_formats = [('png', 90)]
plot_html_show_formats = False
plot_html_show_source_link = False

# Don't add a source link in the sidebar
html_show_sourcelink = False

# Control the appearance of type hints
autodoc_typehints = "none"
autodoc_typehints_format = "short"



# # AutoAPI configuration
autoapi_dirs = ['../../python/rpaudio']
autoapi_generate_api_docs = True
autoapi_root = 'autoapi'
autoapi_type = 'python'
autodoc_typehints = 'description'
autoapi_file_patterns = ['*.pyi']
autoapi_ignore = ['__init__.py', '__init__.pyi', 'asynctest.py', 'threadtest.py', 'docs\source\conf.py']
autoapi_keep_files = True
autoapi_member_order = 'groupwise'
autoapi_options = [
    'members',
    'undoc-members',
    'private-members',
    'special-members',
    'inherited-members',
    'show-inheritance',
]

# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = 'pydata_sphinx_theme'


html_css_files = [f'../../source/cstyle.css']


html_theme_options = {
    "icon_links": [
        {
            "name": "GitHub",
            "url": "https://github.com/sockheadrps/rpaudio",
            "icon": "fab fa-github",
            "type": "fontawesome",
        },
    ],
    "show_prev_next": False,
    "navbar_start": ["navbar-logo"],
    "navbar_end": ["navbar-icon-links"],
    "header_links_before_dropdown": 8,
}

html_context = {
    "default_mode": "dark",
}

html_sidebars = {
    "index": [],
    "examples/index": [],
    "**": ["sidebar-nav-bs.html"],
}

