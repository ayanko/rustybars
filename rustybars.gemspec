# frozen_string_literal: true

require_relative "lib/rustybars/version"

Gem::Specification.new do |spec|
  spec.name = "rustybars"
  spec.version = Rustybars::VERSION
  spec.authors = ["Andriy Yanko"]
  spec.email = ["andriy.yanko@gmail.com"]

  spec.summary = "Ruby binding for handlebars-rust"
  spec.homepage = "https://github.com/ayanko/rustybars"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/rustybars/Cargo.toml"]
end
