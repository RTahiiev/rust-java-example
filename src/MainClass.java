class MainClass {
    private static native String bigAdd(String right, String left);

    static {
        // name of library in `cargo.toml`
        System.loadLibrary("rust_java_example");
    }

    public static void main(String[] args) {
        String output = MainClass.bigAdd("123", "124");
        System.out.println(output);
    }
}