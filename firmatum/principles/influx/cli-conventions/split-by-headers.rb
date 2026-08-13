#!/usr/bin/env ruby

# Split a markdown document by second-level headers (## headers)
# Usage: ruby split-by-headers.rb [input_file] [output_directory]

require 'fileutils'

class MarkdownSplitter
  def initialize(input_file, output_dir)
    @input_file = input_file
    @output_dir = output_dir
    @sections = []

    # Ensure output directory exists
    FileUtils.mkdir_p(@output_dir)
  end

  def split
    content = read_file
    parse_sections(content)
    write_sections
    puts "✓ Split #{@sections.size} sections into #{@output_dir}/"
    list_generated_files
  end

  private

  def read_file
    unless File.exist?(@input_file)
      puts "Error: File #{@input_file} not found"
      exit 1
    end

    File.read(@input_file, encoding: 'UTF-8')
  end

  def parse_sections(content)
    lines = content.lines
    current_section = nil
    intro_lines = []

    lines.each_with_index do |line, index|
      if line.match(/^## (.+)$/)
        # Found a second-level header
        header = $1.strip

        # Save previous section if it exists
        if current_section
          @sections << current_section
        end

        # Start new section
        current_section = {
          header: header,
          filename: header_to_filename(header),
          content: [line]
        }
      elsif current_section
        # Add line to current section
        current_section[:content] << line
      else
        # Content before first ## header goes to intro
        intro_lines << line
      end
    end

    # Add the last section
    if current_section
      @sections << current_section
    end

    # Handle introduction content (everything before first ## header)
    if intro_lines.any?
      # Remove empty lines from the end
      intro_lines.pop while intro_lines.last && intro_lines.last.strip.empty?

      if intro_lines.any?
        intro_section = {
          header: "Introduction",
          filename: "introduction.md",
          content: intro_lines
        }
        @sections.unshift(intro_section)
      end
    end
  end

  def header_to_filename(header)
    # Convert header to safe filename
    filename = header.downcase

    # Replace spaces and special characters with hyphens
    filename = filename.gsub(/[^a-z0-9]+/, '-')

    # Remove leading/trailing hyphens
    filename = filename.gsub(/^-+|-+$/, '')

    # Ensure it's not empty
    filename = 'untitled' if filename.empty?

    # Add .md extension
    "#{filename}.md"
  end

  def write_sections
    @sections.each do |section|
      filepath = File.join(@output_dir, section[:filename])

      File.open(filepath, 'w', encoding: 'UTF-8') do |file|
        # Write the content, ensuring header is included
        if section[:header] == "Introduction"
          # For introduction, write as-is
          file.write(section[:content].join)
        else
          # For other sections, ensure header is first line
          content = section[:content].join
          # If the content doesn't start with the header, add it
          unless content.start_with?("## #{section[:header]}")
            file.write("## #{section[:header]}\n")
            file.write(content)
          else
            file.write(content)
          end
        end
      end

      puts "  Created: #{section[:filename]} (#{section[:header]})"
    end
  end

  def list_generated_files
    puts "\nGenerated files:"
    Dir.glob(File.join(@output_dir, "*.md")).sort.each do |file|
      size = File.size(file)
      puts "  #{File.basename(file)} (#{format_size(size)})"
    end
  end

  def format_size(bytes)
    if bytes < 1024
      "#{bytes} B"
    elsif bytes < 1024 * 1024
      "#{(bytes / 1024.0).round(1)} KB"
    else
      "#{(bytes / (1024.0 * 1024)).round(1)} MB"
    end
  end
end

# Main execution
def main
  input_file = ARGV[0] || 'cli-conventions/full.md'
  output_dir = ARGV[1] || 'cli-conventions'

  puts "Splitting markdown document..."
  puts "Input file: #{input_file}"
  puts "Output directory: #{output_dir}"
  puts

  splitter = MarkdownSplitter.new(input_file, output_dir)
  splitter.split
end

# Run the script
if __FILE__ == $0
  main
end