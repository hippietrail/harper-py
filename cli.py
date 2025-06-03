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
    args = parser.parse_args()

    # Process the text
    result = harper_py.create_english_document(args.text)
    print("Processed text:", result)

if __name__ == '__main__':
    main()
    