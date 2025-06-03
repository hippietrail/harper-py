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
    result = harper_py.create_english_document()
    print(f"  Result: {result}")
    assert isinstance(result, str)
    assert len(result) > 0
    print("  Test passed!")

if __name__ == "__main__":
    test_core_version()
    test_create_english_document()
    print("\nAll tests passed!")