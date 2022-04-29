class MainClass {
    private static native String bigAdd(String right, String left);
    private static native String bigSub(String right, String left);

    static {
        // name of library in `cargo.toml`
        System.loadLibrary("rust_java_example");
    }

    public static void main(String[] args) {
        String output1 = MainClass.bigAdd("12345678900000", "124");
        System.out.println(output1);
        String output2 = MainClass.bigSub("12345678900000", "124");
        System.out.println(output2);
    }
}