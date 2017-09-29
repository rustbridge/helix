require 'tomlrb'

module HelixRuntime
  class Project

    class OutdatedBuildError < StandardError
      def initialize(name)
        super("\n\nHelix crate '#{name}' is outdated. To resolve this issue, run `rake build` and restart your server.\n\n")
      end
    end

    attr_accessor :root
    attr_accessor :helix_lib_dir
    attr_accessor :debug_rust
    attr_accessor :build_root

    def initialize(root)
      @root = find_root(root)
      @debug_rust = ENV['DEBUG_RUST']
      @build_root = @root
    end

    def debug_rust?
      !!debug_rust
    end

    def name
      @name ||= Tomlrb.load_file(cargo_toml_path)["package"]["name"]
    end

    def cargo_toml_path
      "#{root}/Cargo.toml"
    end

    def build_path
      File.expand_path(debug_rust? ? 'target/debug' : 'target/release', build_root)
    end

    def lib_path
      "#{root}/lib/#{name}"
    end

    def libfile_prefix
      IS_WINDOWS ? '' : 'lib'
    end

    def native_path
      "#{lib_path}/native.#{Platform.dlext}"
    end

    def native_lib
      "#{libfile_prefix}#{name.gsub('-', '_')}.#{Platform.libext}"
    end

    def outdated_build?
      mtime = Dir["#{root}/src/**/*.rs"].map{|file| File.mtime(file) }.max
      native = "#{root}/lib/#{name}/native.#{Platform.dlext}"
      !File.exist?(native) || File.mtime(native) < mtime
    end

    def ensure_built!
      raise OutdatedBuildError.new(name) if outdated_build?
    end

    def autobuild
      build if outdated_build?
    end

    def cargo_build
      HelixRuntime.ensure_dll!

      # We have to do this here since Cargo has no internal means of specifying `-C` flags
      link_args = if IS_WINDOWS
        # SAFESEH is added to i686 Rust hosts
        # https://github.com/rust-lang/rust/blob/1.15.1/src/librustc_back/target/i686_pc_windows_msvc.rs#L25
        if `rustc -vV` =~ /host:\s+i686/
          '/SAFESEH:NO' # Can't use SAFESEH with .libs from dlltool
        end
      else
        # Allowing all methods to be undefined is a bit risky, would be nice to have a specific list.
        '-Wl,-undefined,dynamic_lookup'
      end

      env = {}
      env['HELIX_LIB_DIR'] = helix_lib_dir if helix_lib_dir

      cargo_args = []
      rustc_args = []

      if ENV['DEBUG_RUST_MACROS']
        rustc_args << "--pretty expanded"
        rustc_args << "-Z unstable-options"
      end
      unless debug_rust?
        cargo_args << ["--release"]
      end
      if ENV['VERBOSE']
        cargo_args << " --verbose"
      end
      if link_args
        rustc_args << "-C link-args=#{link_args}"
      end

      unless rustc_args.empty?
        cargo_args << "-- #{rustc_args.join(' ')}"
      end

      run env, "cargo rustc #{cargo_args.join(' ')}"
    end

    def cargo_clean
      run("cargo clean")
    end

    def copy_native
      source = "#{build_path}/#{native_lib}"
      raise "native source doesn't exist, run `cargo_build` first; source=#{source}" unless File.exist?(source)
      FileUtils.mkdir_p(File.dirname(native_path))
      FileUtils.cp source, native_path
    end

    def build
      cargo_build
      copy_native
    end

    def clobber
      cargo_clean
      FileUtils.rm_f native_path
    end

    private

      def run(*args)
        Dir.chdir(root) do
          puts(*args)
          system(*args)
        end
      end

      def find_root(root)
        root = File.expand_path(root)
        dir = root
        loop do
          return dir if File.exist?("#{dir}/Cargo.toml")

          new_dir = File.dirname(dir)
          raise "Unable to find root for #{root}" if new_dir == dir

          dir = new_dir
        end
      end

  end
end
