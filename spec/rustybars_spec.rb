# frozen_string_literal: true

RSpec.describe Rustybars do
  describe ".render" do
    subject { described_class.render(template, variables) }

    let(:template) { "Hello {{name}}!" }
    let(:variables) { '{"name":"John Doe"}' }

    context "valid" do
      specify do
        expect(subject).to eq("Hello John Doe!")
      end
    end

    context "template error" do
      let(:template) { "{{" }

      specify do
        expect { subject }.to raise_error(Rustybars::TemplateError, <<~MESSAGE)
          Failed to parse template Template error: invalid handlebars syntax.
              --> Template error in "Unnamed":1:3
               |
             0 | {{
               |--
               |
               = reason: invalid handlebars syntax.
        MESSAGE
      end
    end

    context "data error" do
      let(:variables) { "{" }

      specify do
        expect { subject }.to raise_error(Rustybars::DataError, <<~MESSAGE.strip)
          EOF while parsing an object at line 1 column 1
        MESSAGE
      end
    end
  end
end
