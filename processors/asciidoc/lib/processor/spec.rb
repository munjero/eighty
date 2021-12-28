require "yaml"

module Processor
  class SpecBlock < Asciidoctor::Extensions::BlockProcessor
    use_dsl

    named :spec
    on_context :paragraph

    def process parent, reader, attrs
      spec = YAML.load(reader.lines.join("\n"))
      id = spec["id"]
      description = spec["description"] || parent.title
      discuss = spec["discuss"]

      spec_path = URI.join("https://specs.corepaper.org", id.downcase)

      spec_item = {}
      spec_item[:id] = id
      spec_item[:description] = description
      spec_item[:discuss] = discuss
      spec_item[:sourcePath] = parent.document.attributes['url']
      spec_item[:anchor] = parent.id
      Processor.specs.push(spec_item)

      attrs["name"] = "note"
      attrs["textlabel"] = "Specification"
      create_block parent, :admonition, "This section describes a specification, with identifier #{spec_path}[#{id}]. (#{discuss}[Discuss])", attrs
      end
  end

  Asciidoctor::Extensions.register do
    block SpecBlock
  end
end
