import harper_py

def test_core_version():
    print("\nTesting core_version():")
    version = harper_py.core_version()
    print(f"  Harper Core Version: {version}")
    assert version is not None and version != ""
    print("  Test passed!")

def test_document_creation():
    print("\nTesting document creation:")
    # Test creating a document
    text = "Hello, world!"
    doc = harper_py.PyDocument(text)
    assert doc.get_text() == text
    print("  Document created and text matches")
    print("  Test passed!")

if __name__ == "__main__":
    test_core_version()
    test_document_creation()
    print("\nAll tests passed!")