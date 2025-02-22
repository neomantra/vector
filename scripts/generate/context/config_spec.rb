class Context
  class ConfigSpec
    attr_reader :options

    def initialize(options)
      @options = options
    end

    def categories
      @categories ||= options.collect(&:category).uniq
    end

    def grouped
      @grouped ||= options.group_by(&:category)
    end

    def tags(option)
      tags = []

      tags << (option.required? ? "required" : "optional")

      if !option.default.nil?
        tags << "default: #{option.default.to_toml}"
      else
        tags << "no default"
      end

      if option.unit
        tags << "unit: #{option.unit}"
      end

      if option.enum
        if option.enum.length > 1
          tags << "enum: #{option.enum.collect(&:to_toml).join(", ")}"
        else
          tags << "must be: #{option.enum.first.to_toml}"
        end
      end

      tags
    end
  end
end