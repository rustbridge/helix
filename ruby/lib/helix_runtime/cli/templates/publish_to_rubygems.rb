require 'fileutils'

puts `gem build <%= app_name %>.gemspec`

git_tag = ARGV[0]

if git_tag && git_tag.match(/^v[0-9.]+/)
  require 'yaml'
  gem_config_dir = "#{Dir.home}/.gem"
  credentials_file = "#{gem_config_dir}/credentials"

  FileUtils.mkdir_p gem_config_dir
  File.open(credentials_file, 'w') do |f|
    yaml = { rubygems_api_key: ENV['RUBYGEMS_AUTH_TOKEN'] }.to_yaml
    f.puts yaml
  end
  File.chmod 0600, credentials_file

  puts `gem push <%= app_name %>-*.gem`
end
