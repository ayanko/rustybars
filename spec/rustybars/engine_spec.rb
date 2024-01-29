# frozen_string_literal: true

RSpec.describe Rustybars::Engine do
  let(:engine) { described_class.new(**options) }

  let(:options) { {} }
  let(:template) { "Hello {{name}}!" }
  let(:variables) { '{"name":"John Doe"}' }

  describe "#render" do
    subject { engine.render(template, variables) }

    context "invalid template" do
      let(:template) { "{{" }

      specify do
        expect { subject }.to raise_error(Rustybars::CompileError, <<~MESSAGE)
          Template error: invalid handlebars syntax.
              --> Template error in "Unnamed":1:3
               |
             0 | {{
               |--
               |
               = reason: invalid handlebars syntax.
        MESSAGE
      end
    end

    context "invalid variables" do
      let(:variables) { "{" }

      specify do
        expect { subject }.to raise_error(Rustybars::JsonError, <<~MESSAGE.strip)
          EOF while parsing an object at line 1 column 1
        MESSAGE
      end
    end

    context "valid" do
      specify do
        expect(subject).to eq("Hello John Doe!")
      end
    end

    context "missing variable" do
      let(:variables) { '{"email":"john.doe@example.com"}' }

      specify do
        expect(subject).to eq("Hello !")
      end

      context "strict_mode" do
        let(:options) { { strict_mode: true } }

        specify do
          expect { subject }.to raise_error(Rustybars::RenderError, <<~MESSAGE.strip)
            Error rendering "Unnamed template" line 1, col 7: Failed to access variable in strict mode Some("name")
          MESSAGE
        end
      end
    end
  end

  describe "#compile" do
    subject { engine.compile(template) }

    specify do
      expect(subject).to be_a(Rustybars::CompiledTemplate)
      expect(subject.render(variables)).to eq("Hello John Doe!")
    end

    context "invalid template" do
      let(:template) { "Hello {{}}" }

      specify do
        expect { subject }.to raise_error(Rustybars::CompileError, <<~MESSAGE)
          Template error: invalid handlebars syntax.
              --> Template error in "Unnamed":1:9
               |
             0 | Hello {{}}
               |---------^
               |
               = reason: invalid handlebars syntax.
        MESSAGE
      end
    end
  end

  describe "#inspect" do
    subject { engine.inspect }

    specify do
      expect(subject).to eq("<Rustybars::Engine { strict_mode: false, dev_mode: false, prevent_indent: false }>")
    end
  end
end
