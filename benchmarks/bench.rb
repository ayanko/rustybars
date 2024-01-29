# frozen_string_literal: true

require "bundler/setup"

require "rustybars"
require "faker"
require "benchmark"

keys = 100.times.map { |i| "key#{i}" }

template = keys.map { |key| "#{key}={{#{key}}}" }.join("\n")

variables_array = 1_000.times.map do
  keys
    .each_with_object({}) { |key, hash| hash[key] = "value_#{Faker::Lorem.word}" }
    .to_json
end

Benchmark.bm(30) do |x|
  x.report("inline") do
    variables_array.each do |variables|
      Rustybars::Engine.new.render(template, variables)
    end
  end

  x.report("reuse engine") do
    hbs = Rustybars::Engine.new
    variables_array.each do |variables|
      hbs.render(template, variables)
    end
  end

  x.report("reuse template") do
    hbs = Rustybars::Engine.new
    compiled_template = hbs.compile(template)
    variables_array.each do |variables|
      compiled_template.render(variables)
    end
  end

  x.report("using 10 ruby threads") do
    hbs = Rustybars::Engine.new
    compiled_template = hbs.compile(template)
    variables_array.each_slice(variables_array.size / 10).map do |slice|
      Thread.new do
        slice.each { |variables| compiled_template.render(variables) }
      end
    end.join
  end
end
