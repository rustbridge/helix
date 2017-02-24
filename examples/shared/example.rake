require 'rake/clean'
require_relative 'platform'

directory "lib/#{$example_name}"

namespace :cargo do
  task :build do
    sh "cargo rustc --release -- -C link-args='-Wl,-undefined,dynamic_lookup'"
  end

  task :clean do
    sh "cargo clean"
  end
end

task :clobber => "cargo:clean"

native_path = "lib/#{$example_name}/native.#{Platform.dlext}"

file native_path => ["lib/#{$example_name}", "cargo:build"] do
  cp "../../target/release/lib#{$example_name}.#{Platform.libext}", native_path
end
CLOBBER.include(native_path)

task :irb => native_path do
  exec "irb -Ilib -r#{$example_name}"
end

# alias
task :build_native => native_path
