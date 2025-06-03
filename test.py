import harper_py

def test_core_version():
    print("\nTesting core_version():")
    version = harper_py.core_version()
    print(f"  Harper Core Version: {version}")
    assert isinstance(version, str)
    assert len(version) > 0
    print("  Test passed!")

if __name__ == "__main__":
    test_core_version()
    print("\nAll tests passed!")