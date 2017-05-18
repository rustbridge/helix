module HelixRuntime
  class ParentProject

    attr_accessor :root

    def initialize(root)
      @root = find_root(root)
    end

    def projects
      @projects ||= Dir["#{root}/crates/*"]
                        .select{|f| File.exist?("#{f}/Cargo.toml") }
                        .map{|d| Project.new(File.basename(d), d) }
    end

    def ensure_built!
      projects.each(&:ensure_built!)
    end

    def outdated_build?
      projects.any?(&:outdated_build?)
    end

    private

      def find_root(root)
        root = File.expand_path(root)
        dir = root
        loop do
          return dir if !Dir["#{dir}/{Gemfile,*.gemspec}"].empty?

          new_dir = File.dirname(dir)
          raise "Unable to find root for #{root}" if new_dir == dir

          dir = new_dir
        end
      end

  end
end