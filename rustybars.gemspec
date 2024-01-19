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

  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,lock,rb}"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/rustybars/extconf.rb"]
end
