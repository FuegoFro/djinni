package com.dropbox.djinni.test;

import junit.framework.TestCase;

public class TokenTest extends TestCase {

    private class JavaToken implements UserToken {
        public String whoami() { return "Java"; }
    }

    @Override
    protected void setUp() {
    }

    public void testTokens() {
        UserToken jt = new JavaToken();
        assertSame(TestHelpers.tokenId(jt), jt);
    }

    public void testNullToken() {
        assertSame(TestHelpers.tokenId(null), null);
    }

    public void testCppToken() {
        UserToken ct = TestHelpers.createCppToken();
        assertSame(TestHelpers.tokenId(ct), ct);
        TestHelpers.checkCppToken(ct);
        ct = null;
        System.gc();
        System.runFinalization();
    }

    public void testTokenType() {
        assertTrue("1", TestHelpers.checkTokenType(new JavaToken(), "Java"));
        assertEquals("2", TestHelpers.createCppToken().whoami(), "C++");
        assertTrue("3", TestHelpers.checkTokenType(TestHelpers.createCppToken(), "C++"));
        assertFalse("4", TestHelpers.checkTokenType(new JavaToken(), "foo"));
        assertFalse("5", TestHelpers.checkTokenType(TestHelpers.createCppToken(), "foo"));
    }

    public void testNotCppToken() {
        boolean threw = false;
        try {
            TestHelpers.checkCppToken(new JavaToken());
        } catch (RuntimeException e) {
            threw = true;
        }
        assertTrue(threw);
        System.gc();
    }
}
