$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
$LOAD_PATH.unshift File.expand_path('../support/dummy/lib', __FILE__)
require 'helix_runtime'
require 'dummy'

module HelixRuntime
  module SpecHelpers
  end
end

RSpec.configure do |c|
  c.include HelixRuntime::SpecHelpers
end
