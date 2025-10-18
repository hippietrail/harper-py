#!/usr/bin/env python3
"""
Harper CLI - Command line interface for the harper-py library.
"""

import argparse
import harper_py

def main():
    parser = argparse.ArgumentParser(description='Process text with Harper grammar checker')
    parser.add_argument('text', nargs='?', default="Hello, world!",
                      help='Text to process (default: "Hello, world!")')
    parser.add_argument('--lint', '-l', action='store_true',
                      help='Show linting issues')
    args = parser.parse_args()

    # Process the text
    doc = harper_py.create_english_document(args.text)
    
    # Display results
    if args.lint:
        text = doc.get_text()
        lint_group = harper_py.create_curated_lint_group()
        unsorted_lints = doc.get_lints(lint_group)
        
        if not unsorted_lints:
            print("No issues found!")
        else:
            print(f"Found {len(unsorted_lints)} issue(s):")
            lines = text.split('\n')

            lints = sorted(unsorted_lints, key=lambda lint: lint.start())
            
            for i, lint in enumerate(lints, 1):
                # Find which line the lint is on
                line_num = 0
                line_start = 0
                for line in lines:
                    line_end = line_start + len(line) + 1  # +1 for newline
                    if line_start <= lint.start() < line_end:
                        break
                    line_start = line_end
                    line_num += 1
                
                line_text = lines[line_num] if line_num < len(lines) else ""
                print(f"\n{i}. {lint.kind()} ({lint.priority()}) : {lint.message()}")
                print(f"   Line {line_num + 1}: {line_text}")
                print("           " + " " * (lint.start() - line_start) + "^" * max(1, lint.end() - lint.start()))
                print(f"   Suggestions:")
                for suggestion in lint.suggestions():
                    print(f"     - {suggestion.text()}")
    else:
        print(doc.get_text())

if __name__ == '__main__':
    main()