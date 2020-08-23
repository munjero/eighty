module Processor
  class SidenoteMacro < Asciidoctor::Extensions::InlineMacroProcessor
    use_dsl

    named :sidenote

    @@sidenote_count = 0

    def process parent, target, attrs
      notename = "sidenote-#{target}" || "sidenote-unnamed-#{@@sidenote_count}"
      content = attrs.values.join(", ")

      @@sidenote_count = @@sidenote_count + 1

      %{<label class="margin-toggle sidenote-number" for="#{notename}"></label><input id="#{notename}" class="margin-toggle" type="checkbox" /><span class="sidenote">#{content}</span>}
    end

    def resolve_regexp name, format
      if format == :short
        %r(\\?#{name}:\[((?:\\\]|[^\]])*?)\])
      else
        %r(\\?#{name}:(\S+?)\[((?:\\\]|[^\]])*?)\])
      end
    end
  end

  class MarginnoteMacro < Asciidoctor::Extensions::InlineMacroProcessor
    use_dsl

    named :marginnote

    @@sidenote_count = 0

    def process parent, target, attrs
      notename = "sidenote-#{target}" || "sidenote-unnamed-#{@@sidenote_count}"
      content = attrs.values.join(", ")

      @@sidenote_count = @@sidenote_count + 1

      %{<label class="margin-toggle" for="#{notename}"></label><input id="#{notename}" class="margin-toggle" type="checkbox" /><span class="marginnote">#{content}</span>}
    end

    def resolve_regexp name, format
      if format == :short
        %r(\\?#{name}:\[((?:\\\]|[^\]])*?)\])
      else
        %r(\\?#{name}:(\S+?)\[((?:\\\]|[^\]])*?)\])
      end
    end
  end

  Asciidoctor::Extensions.register do
    inline_macro SidenoteMacro
    inline_macro MarginnoteMacro
  end
end
