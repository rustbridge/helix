require 'rake/tasklib'
require 'helix_runtime'

module HelixRuntime
  class BuildTask < Rake::TaskLib

    def self.delegate_attr(getter, to:)
      setter = "#{getter}="
      define_method(getter,       -> { send(to).send(getter) })
      define_method(setter, -> (val) { send(to).send(setter, val) })
    end

    def project
      @project ||= Project.new(@name, Dir.pwd)
    end

    delegate_attr :helix_lib_dir,  to: :project
    delegate_attr :debug_rust,     to: :project
    delegate_attr :build_root,     to: :project

    attr_reader :name
    attr_accessor :pre_build

    def initialize(name = nil, gem_spec = nil)
      init(name, gem_spec)
      yield self if block_given?
      define
    end

    def init(name = nil, gem_spec = nil)
      @name = name
    end

    def define
      fail "Extension name must be provided." if @name.nil?
      @name = @name.to_s

      task "helix:pre_build" do
        pre_build.call if pre_build
      end

      task "helix:check_path" do
        begin
          HelixRuntime.ensure_dll!
        rescue HelixRuntime::MissingDllError => e
          puts e.message
          abort "Run `rake helix:copy_dll` to copy to your Ruby bin dir."
        end
      end

      task "helix:copy_dll" do
        HelixRuntime.copy_dll
      end

      task "cargo:build" => ["helix:pre_build", "helix:check_path"] do
        project.cargo_build
      end

      task "cargo:clean" do
        project.cargo_clean
      end

      desc "Build #{name}"
      task :build => ["helix:pre_build", "helix:check_path"] do
        project.build
      end

      desc "Remove build artifacts"
      task :clobber do
        project.clobber
      end

      desc "Launch an IRB console for #{name}"
      task :irb => :build do
        exec "bundle exec irb -r#{name} -Ilib"
      end
    end

  end
end
