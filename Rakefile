# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("rustybars.gemspec")

RbSys::ExtensionTask.new("rustybars", GEMSPEC) do |ext|
  ext.lib_dir = "lib/rustybars"
  ext.cross_compile = true
  ext.cross_platform = %w[x86_64-linux x86_64-darwin arm64-darwin aarch64-linux]
end

task default: %i[compile spec rubocop]
