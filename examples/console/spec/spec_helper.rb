$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
require 'console'

module PrintMatchers
  def print(expected = nil)
    output(expected).to_stdout_from_any_process
  end

  def println(expected)
    output(expected + "\n").to_stdout_from_any_process
  end
end

RSpec.configure do |config|
  config.include PrintMatchers
end
