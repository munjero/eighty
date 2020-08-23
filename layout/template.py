import os
from pprint import pprint
from jinja2 import Environment, FileSystemLoader, select_autoescape

env = Environment(
    loader=FileSystemLoader(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'template')),
    autoescape=select_autoescape(['html', 'xml'])
)

def debug(text):
    print("Template debug: ", end='')
    pprint(text)
    return ''

env.filters['debug'] = debug

def render_document(page, site, sitemap):
    template = env.get_template('document.html')
    return template.render(page=page, site=site, sitemap=sitemap)

def render_spec_index(specs):
    template = env.get_template('spec-index.html')
    return template.render(specs=specs)

def render_spec_redirect(spec):
    template = env.get_template('spec-redirect.html')
    return template.render(spec=spec)
