# Rustybars

Ruby binding to [handlebars-rust](https://github.com/sunng87/handlebars-rust)

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add rustybars

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install rustybars

## Usage

### Basic

```ruby
Rustybars::Engine.new.render('Hello {{name}}!', {name: 'John Doe'}.to_json)
```

### Re-use engine

```ruby
hbs = Rustybars::Engine.new
hbs.render('Hello {{name}}!', {name: 'John Doe'}.to_json)
```

### Re-use compiled template

```ruby
hbs = Rustybars::Engine.new
template = hbs.compile('Hello {{name}}!')
template.render({name: 'Alice'}.to_json)
# => "Hello Alice!"
template.render({name: 'Bob'}.to_json)
# => "Hello Bob!"
```

### Strict mode

```ruby
hbs = Rustybars::Engine.new(strict_mode: true)
template = hbs.compile('Hello {{name}}!')
template.render({name: 'Alice'}.to_json)
# => "Hello Alice!"
template.render({another_name: 'Bob'}.to_json)
# =>  Error rendering "Unnamed template" line 1, col 7: Failed to access variable in strict mode Some("name") (Rustybars::RenderError)
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Cross compilation

```sh
bundle exec rb-sys-dock -p arm64-darwin --ruby-versions 3.2 --build
bundle exec rb-sys-dock -p x86_64-linux --ruby-versions 3.2 --build
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ayanko/rustybars. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/ayanko/rustybars/blob/master/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the Rustybars project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/ayanko/rustybars/blob/master/CODE_OF_CONDUCT.md).
