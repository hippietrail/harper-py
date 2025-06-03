import harper_py

def test_core_version():
    print("\nTesting core_version():")
    version = harper_py.core_version()
    print(f"  Harper Core Version: {version}")
    assert isinstance(version, str)
    assert len(version) > 0
    print("  Test passed!")

def test_create_english_document():
    print("\nTesting create_english_document():")
    # Test with default text
    default_result = harper_py.create_english_document()
    print(f"  Default result: {default_result}")
    assert isinstance(default_result, str)
    assert len(default_result) > 0
    
    # Test with custom text
    custom_text = "Testing custom text"
    custom_result = harper_py.create_english_document(custom_text)
    print(f"  Custom text result: {custom_result}")
    assert custom_text in custom_result
    print("  Test passed!")

if __name__ == "__main__":
    test_core_version()
    test_create_english_document()
    print("\nAll tests passed!")