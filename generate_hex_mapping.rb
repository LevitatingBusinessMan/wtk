# claude generated
def generate_rust_mapping(file_path)
  mappings = []
  
  File.foreach(file_path).with_index do |line, index|
    next unless line =~ /^([0-9A-Fa-f]+):/
    unicode_hex = $1
    unicode_val = unicode_hex.to_i(16)
    mappings << [unicode_val, index]
  end

  # Grouping contiguous ranges for efficiency
  puts "|c| match c as usize {"
  
  current_range = nil
  
  mappings.each do |val, idx|
    if current_range && val == current_range[:last_val] + 1 && idx == current_range[:last_idx] + 1
      current_range[:last_val] = val
      current_range[:last_idx] = idx
    else
      print_range(current_range) if current_range
      current_range = { start_val: val, last_val: val, start_idx: idx, last_idx: idx }
    end
  end
  print_range(current_range)
  
  puts "    _ => 0, // Default to glyph 0"
  puts "}"
end

def print_range(r)
  if r[:start_val] == r[:last_val]
    puts "    #{r[:start_val]} => #{r[:start_idx]},"
  else
    # Maps char value to index via offset
    offset = r[:start_idx] - r[:start_val]
    sign = offset >= 0 ? "+" : "-"
    puts "    #{r[:start_val]}..=#{r[:last_val]} => (c as usize) #{sign} #{offset.abs},"
  end
end

generate_rust_mapping("src/assets/fonts/unscii-16-full.hex")

