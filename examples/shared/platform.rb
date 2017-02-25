require 'rbconfig'

module Platform
  # Normalize the platform OS
  OS = case os = RbConfig::CONFIG['host_os'].downcase
  when /linux/
    # The official ruby-alpine Docker containers pre-build Ruby. As a result,
    #   Ruby doesn't know that it's on a musl-based platform. `ldd` is the
    #   only reliable way to detect musl that we've found.
    # See https://github.com/skylightio/skylight-ruby/issues/92
    if ENV['SKYLIGHT_MUSL'] || `ldd --version 2>&1` =~ /musl/
      "linux-musl"
    else
      "linux"
    end
  when /darwin/
    "darwin"
  when /freebsd/
    "freebsd"
  when /netbsd/
    "netbsd"
  when /openbsd/
    "openbsd"
  when /sunos|solaris/
    "solaris"
  when /mingw|mswin/
    "windows"
  else
    os
  end

  # Normalize the platform CPU
  ARCH = case cpu = RbConfig::CONFIG['host_cpu'].downcase
  when /amd64|x86_64/
    "x86_64"
  when /i?86|x86|i86pc/
    "x86"
  when /ppc|powerpc/
    "powerpc"
  when /^arm/
    "arm"
  else
    cpu
  end

  LIBEXT = case OS
  when /darwin/
    'dylib'
  when /linux|bsd|solaris/
    'so'
  when /windows|cygwin/
    'dll'
  else
    'so'
  end

  TUPLE = "#{ARCH}-#{OS}"

  def self.tuple
    TUPLE
  end

  def self.libext
    LIBEXT
  end

  def self.dlext
    RbConfig::CONFIG['DLEXT']
  end
end
