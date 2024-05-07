# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/extensiontask"

task build: :compile

Rake::ExtensionTask.new("rbcdt") do |ext|
  ext.lib_dir = "lib/rbcdt"
end

task default: :compile
