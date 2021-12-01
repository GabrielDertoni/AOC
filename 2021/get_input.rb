#!/usr/bin/ruby

require 'optparse'

cookie = ENV["AOC_SESSION"]
OptionParser.new do |opts|
  opts.banner = "Usage get_input.rb [optsion] day"

  opts.on("-s", "--session SESSION", "The session cookie. Defaults to AOC_SESSION") do |sess|
    cookie = sess
  end
end.parse!

if cookie == nil
  puts "No session. Please use -s or set AOC_SESSION"
end

if ARGV.length < 1
  puts "No day. Plese specify the day to get input for"
end

day = ARGV[0].to_i

puts `curl -sS -o- https://adventofcode.com/2021/day/#{day}/input --cookie "session=#{cookie}"`
