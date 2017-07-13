require 'helix_runtime/cli/bootstrap'

module HelixRuntime
  module CLI
    class Base < Thor
      include Thor::Actions

      register CLI::Bootstrap, "bootstrap", "bootstrap PATH [NAME]", "Bootstrap Helix"

      desc "crate NAME", "Generate a Helix crate"
      option :skip_bundle, type: :boolean, default: false
      def crate(name)
        invoke CLI::Bootstrap, ["crates/#{name}", name], skip_bundle: true

        append_to_file "Gemfile", "gem '#{name}', path: 'crates/#{name}'\n"

        run "bundle" unless options.skip_bundle
      end
    end
  end
end
