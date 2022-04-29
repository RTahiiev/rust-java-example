extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use primitive_types::U512;

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_MainClass_bigAdd(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                             _class: JClass,
                                             right: JString,
                                             left: JString)
                                             -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let rvalue: String =
        env.get_string(right).expect("Couldn't get java string!").into();
    let lvalue: String =
        env.get_string(left).expect("Couldn't get java string!").into();

    // Convert to big num
    let rvalue_int = U512::from_dec_str(&rvalue).expect("Couldn't convert to big int!");
    let lvalue_int = U512::from_dec_str(&lvalue).expect("Couldn't convert to big int!");

    // Try to add
    let result = match rvalue_int.checked_add(lvalue_int) {
        Some(res) => res,
        None => panic!("Coudn't add!"),
    };

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("{}", result.to_string()))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[no_mangle]
#[warn(unused_variables)]
pub extern "system" fn Java_MainClass_bigSub(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                             _class: JClass,
                                             right: JString,
                                             left: JString)
                                             -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let rvalue: String =
        env.get_string(right).expect("Couldn't get java string!").into();
    let lvalue: String =
        env.get_string(left).expect("Couldn't get java string!").into();

    // Convert to big num
    let rvalue_int = U512::from_dec_str(&rvalue).expect("Couldn't convert to big int!");
    let lvalue_int = U512::from_dec_str(&lvalue).expect("Couldn't convert to big int!");

    // Try to substract
    let result = match rvalue_int.checked_sub(lvalue_int) {
        Some(res) => res,
        None => panic!("Coudn't substract!"),
    };

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("{}", result.to_string()))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}