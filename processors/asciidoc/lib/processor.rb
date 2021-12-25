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

  def self.document
    @@document
  end

  def self.document=(value)
    @@document = value
  end

  def self.process(source)
    Processor.read_file(source)

    puts JSON.pretty_generate({
      :document => Processor.document,
      :specs => Processor.specs,
    })
  end
end
