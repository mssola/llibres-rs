#!/usr/bin/env ruby

require 'json'
require 'net/http'
require 'uri'
require 'yaml'

# Kinds available.
NONE = 0
POETRY = 1
THEATER = 2
ESSAY = 3
SHORTS = 4

# Statuses
READ = 0
NOTREAD = 1
TOBUY = 2
SELECTED = 3
TOBEPUBLISHED = 4

# Known stuff
KNOWN_KINDS = %w[none poetry theater essay shorts].freeze
KNOWN_STATUSES = %w[read notread tobuy selected tobepublished].freeze
KNOWN_LANGUAGES = %w[ca es en grc la it].freeze

##
# Utilities.

def blank?(str)
  str.nil? || str == ''
end

def check_mandatory!(book)
  %w[title publisher].each do |k|
    raise StandardError, "'#{book['title']}': you have to set a '#{k}' for it!" if blank?(book[k])
  end
end

def parse_kind!(title, kind)
  raise StandardError, "'#{title}': given kind '#{kind}' is not valid!" unless KNOWN_KINDS.include?(kind)

  Object.const_get(kind.upcase)
end

def parse_status!(title, status)
  raise StandardError, "'#{title}': given status '#{status}' is not valid!" unless KNOWN_STATUSES.include?(status)

  Object.const_get(status.upcase)
end

def check_language!(title, lang)
  raise StandardError, "Book '#{title}': no language given!" if blank?(lang)

  codes = lang.split(',').map(&:strip) - KNOWN_LANGUAGES
  return if codes.empty?

  raise StandardError, "Book '#{title}': unknown languages #{codes.join(', ')}"
end

##
# Make some preliminary checks before moving on.

raise StandardError, 'ruby seed.rb <file.yml> <url>' if ARGV.size != 2

books = YAML.load_file(ARGV.first)

uri = URI.parse(ARGV.last)
uri.path = '/health'
res = Net::HTTP.get_response(uri)
raise StandardError, "Got a '#{res.code}' when talking with the server..." unless res.is_a?(Net::HTTPSuccess)

##
# Make sure that the user knows what he/she is doing.

if blank?(ENV['CI'])
  print 'This will clear whatever it was in your database before, are you sure? [Y/n] '
  resp = $stdin.gets.chomp
  unless resp == '' || resp.downcase == 'y'
    puts 'bye!'
    exit 0
  end
end

##
# Nuke whatever it was before.

uri = URI.parse(ARGV.last)
uri.path = '/books'
res = Net::HTTP.get_response(uri)
if res.code != '200'
  puts 'Something went wrong, check your server!'
  exit 1
end

ids = JSON.parse(res.body).map { |h| h['id'] }.compact
puts 'Clearing existing rows...' unless ids.empty?

ids.each do |id|
  uri = URI.parse(ARGV.last)
  uri.path = "/books/#{id}"
  http = Net::HTTP.new(uri.host, uri.port)
  req = Net::HTTP::Delete.new(uri.path)
  res = http.request(req)
  if res.code != '204'
    puts 'Something went wrong, check your server!'
    exit 1
  end
end

##
# Instead of getting errors from the server, let's do some sanity checking
# first.

books = books.map do |b|
  check_mandatory!(b)

  b['rate'] = b['rate'].to_i
  b['kind'] = 'none' if blank?(b['kind'])
  b['kind'] = parse_kind!(b['title'], b['kind'])
  b['status'] = parse_status!(b['title'], b['status'])

  check_language!(b['title'], b['language'])

  b
end

##
# We are now good to go!

header = { 'Content-Type': 'text/json' }

books.each do |b|
  uri.path = '/books'
  http = Net::HTTP.new(uri.host, uri.port)
  req = Net::HTTP::Post.new(uri.request_uri, header)
  if !blank?(ENV['LLIBRES_USER']) && !blank?(ENV['LLIBRES_PASSWORD'])
    req.basic_auth(ENV['LLIBRES_USER'], ENV['LLIBRES_PASSWORD'])
  end
  req.body = b.to_json

  res = http.request(req)
  unless res.is_a?(Net::HTTPCreated)
    puts res.body
    raise StandardError, "Book '#{b['title']}': got a '#{res.code}' when talking with the server..."
  end
end

puts 'Done!'
