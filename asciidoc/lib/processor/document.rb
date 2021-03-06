require "fileutils"

module Processor
  def self.search_description(blocks)
    blocks.each do |b|
      if b.blocks.length != 0
        subblock = Processor.search_description(b.blocks)
        if subblock
          return subblock
        end
      end

      if b.attributes["meta"] == "description"
        return b
      end
    end
    nil
  end

  def self.read_file(path, full_path, out_path, full_out_path)
    options = {
      :attributes => {
        "outfilesuffix" => ".adoc",
        "relfileprefix" => "@@ASCIIDOCLINK@@:",
        "source-highlighter" => "highlightjs",
        "rouge-css" => "style",
        "stem" => "latexmath",
        "stylesdir" => "/",
        "icons" => "font",
        "idprefix" => "",
        "idseparator" => "-",
        "url" => "@@XREFFULLLINK:#{path}@@@",
        "sectanchors" => true,
      }
    }

    item = {}
    xrefs = []
    doc = Asciidoctor.load_file full_path, options

    item[:title] = doc.attributes["doctitle"]
    item[:sourcePath] = path
    item[:license] = doc.attributes["license"]
    item[:licenseCode] = doc.attributes["license-code"]
    item[:author] = doc.attributes["author"]
    item[:email] = doc.attributes["email"]
    item[:order] = doc.attributes["order"]
    item[:layout] = "document"
    if item[:order]
      item[:order] = Integer(item[:order])
    end
    item[:toc] = doc.converter.convert(doc, "outline", toclevels: 3)
    item[:revision] = {}
    if doc.attributes["created"]
      item[:revision][:created] = doc.attributes["created"]
    end

    description_block = Processor.search_description(doc.blocks)
    item[:description] = description_block.content.tr("\n", " ")

    content = doc.convert
    replaces = []
    content.scan(/@@ASCIIDOCLINK@@:([^#"]*)/).each do |match|
      content_rel_path = match[0]
      content_path = Pathname.new(File.join(File.dirname(path), content_rel_path)).cleanpath

      replace = {
        :from => "@@ASCIIDOCLINK@@:#{content_rel_path}",
        :to => "@@XREFLINK:#{content_path}@@"
      }
      replaces.push(replace)
    end

    replaces.each do |replace|
      content = content.gsub replace[:from], replace[:to]
    end
    item[:content] = content

    Processor.xrefs.push(out_path)
    Processor.documents[out_path] = item
  end

  def self.read_folder(base, out)
    files = Dir.glob("#{base}/**/*").select { |f| File.file?(f) }

    files.each do |file|
      if File.extname(file) == ".adoc"
        path = Pathname.new(file).relative_path_from(base).to_s
        out_path = path.sub /\.[^.]+\z/, ".html.jsondoc"
        full_path = file
        full_out_path = File.join(out, out_path)

        Processor.read_file(path, full_path, out_path, full_out_path)
      else
        path = Pathname.new(file).relative_path_from(base).to_s
        out_path = path
        full_path = file

        Processor.contents[out_path] = File.open(full_path).read
      end
    end
  end
end
