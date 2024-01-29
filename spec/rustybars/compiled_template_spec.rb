# frozen_string_literal: true

RSpec.describe Rustybars::CompiledTemplate do
  let(:template) { "Hello {{name}}!" }
  let(:variables) { '{"name":"John Doe"}' }

  describe ".new" do
    subject { described_class.new }

    specify do
      expect { subject }.to raise_error(
        TypeError, "allocator undefined for Rustybars::CompiledTemplate"
      )
    end
  end

  describe "instance" do
    let(:instance) { Rustybars::Engine.new.compile(template) }

    specify do
      expect(instance).to be_a(described_class)
    end

    context "invalid template" do
      let(:template) { "{{" }

      specify do
        expect { instance }.to raise_error(Rustybars::CompileError, <<~MESSAGE)
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

    describe "#render" do
      subject { instance.render(variables) }

      specify do
        expect(subject).to eq("Hello John Doe!")
      end

      context "invalid variables" do
        let(:variables) { "{" }

        specify do
          expect { subject }.to raise_error(Rustybars::JsonError, <<~MESSAGE.strip)
            EOF while parsing an object at line 1 column 1
          MESSAGE
        end
      end
    end
  end
end
