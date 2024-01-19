# frozen_string_literal: true

require_relative "rustybars/version"

# Load extension
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require_relative "rustybars/#{Regexp.last_match(1)}/rustybars"
rescue LoadError
  require_relative "rustybars/rustybars"
end

module Rustybars
end
