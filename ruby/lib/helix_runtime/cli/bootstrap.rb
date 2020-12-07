require 'thor'

module HelixRuntime
  module CLI
    class Bootstrap < Thor::Group
      argument :path, type: :string
      argument :name, type: :string, optional: true
      class_option :skip_bundle, type: :boolean, default: false

      include Thor::Actions

      desc "Bootstrap Helix"
      source_root File.expand_path("../templates", __FILE__)

      # NOTE: Instead of using destination_root, we include the full path so
      #   that we see the path relative to the root of where we're running the command.

      def create_cargo_toml
        template "Cargo.toml", "#{base_path}/Cargo.toml"
      end

      def add_rust_lib_file
        template "lib.rs", "#{base_path}/src/lib.rs"
      end

      def create_gemspec
        template "gem.gemspec", "#{base_path}/#{app_name}.gemspec"
      end

      def create_gemfile
        template "Gemfile", "#{base_path}/Gemfile"
      end

      def add_rake_task
        template "helix_runtime.rake", "#{base_path}/lib/tasks/helix_runtime.rake"
      end

      def add_ruby_lib_file
        template "lib.rb", "#{base_path}/lib/#{app_name}.rb"
      end

      def add_gitignore
        template "gitignore", "#{base_path}/.gitignore"
      end

      def add_publish_to_rubygems_file
        template "publish_to_rubygems.rb", "#{base_path}/.scripts/publish_to_rubygems.rb"
      end

      def add_travis_yml_file
        template ".travis.yml", "#{base_path}/.travis.yml"
      end

      def add_appveyor_yml_file
        template ".appveyor.yml", "#{base_path}/.appveyor.yml"
      end

      def update_rakefile
        unless File.exists?("#{base_path}/Rakefile")
          create_file "#{base_path}/Rakefile", "require 'bundler/setup'\n"
        end

        append_to_file "#{base_path}/Rakefile", "import 'lib/tasks/helix_runtime.rake'\n"
      end

      def bundle
        unless options.skip_bundle
          inside path do
            run "bundle"
          end
        end
      end

      private

        def base_path
          File.expand_path(path)
        end

        def app_name
          name || File.basename(base_path)
        end

        def class_name
          app_name.split(/[_-]/).map(&:capitalize).join
        end
    end
  end
end
