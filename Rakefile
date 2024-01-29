# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"

require "rspec/core/rake_task"
require "rubocop/rake_task"
require "rake/extensiontask"

cross_rubies = %w[3.3.0 3.2.0 3.1.0]
cross_platforms = %w[
  x86_64-linux
  x86_64-linux-musl
  aarch64-linux
  x86_64-darwin
  arm64-darwin
]

GEMSPEC = Gem::Specification.load("rustybars.gemspec")

RbSys::ExtensionTask.new("rustybars", GEMSPEC) do |ext|
  ext.lib_dir = "lib/rustybars"
end

namespace "build" do
  cross_platforms.each do |platform|
    desc "Build the native gem for #{platform}"
    task platform do
      sh(
        "bundle", "exec", "rb-sys-dock",
        "-p", platform,
        "-r", cross_rubies.join(","),
        "--build"
      )
    end
  end
end

RSpec::Core::RakeTask.new(:spec)

RuboCop::RakeTask.new

desc "Run benchmark"
task :bench do
  require_relative "benchmarks/bench"
end

task build: :compile

task default: %i[compile spec rubocop]
