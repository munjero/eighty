require "json"
require "asciidoctor"

require "processor/spec"
require "processor/sidenote"
require "processor/document"

module Processor
  def self.specs
    @@specs ||= []
    @@specs
  end

  def self.documents
    @@documents ||= {}
    @@documents
  end

  def self.contents
    @@contents ||= {}
    @@contents
  end

  def self.xrefs
    @@xrefs ||= []
    @@xrefs
  end

  def self.process(base, out)
    Processor.read_folder(base, out)

    Processor.contents.each do |path, content|
      full_out_path = File.join(out, path)
      full_out_dirname = File.dirname full_out_path
      unless File.directory? full_out_dirname
        FileUtils.mkdir_p full_out_dirname
      end
      File.open full_out_path, "w" do |file|
        file.write content
      end
    end

    Processor.documents.each do |path, document|
      full_out_path = File.join(out, path)
      full_out_dirname = File.dirname full_out_path
      unless File.directory? full_out_dirname
        FileUtils.mkdir_p full_out_dirname
      end
      File.open full_out_path, "w" do |file|
        file.write JSON.pretty_generate(document)
      end
    end

    specs_path = File.join(out, "_specs.json")
    specs_json = JSON.pretty_generate(Processor.specs)
    File.open specs_path, "w" do |file|
      file.write specs_json
    end
    Processor.xrefs.push("_specs.json")

    xrefs_path = File.join(out, "_xrefs.json")
    xrefs_json = JSON.pretty_generate(Processor.xrefs)
    File.open xrefs_path, "w" do |file|
      file.write xrefs_json
    end
  end
end
