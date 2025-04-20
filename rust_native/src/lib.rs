use jni::{
    JNIEnv,
    objects::{JObject, JValueGen},
    sys::jlong,
};

#[derive(Debug)]
#[repr(C)]
pub struct Student {
    name: String,
    age: u8,
    grade: u8,
    life: bool,
}

impl Student {
    pub fn new(name: &str, age: u8, grade: u8, life: bool) -> Box<Self> {
        Box::new(Self {
            name: name.to_string(),
            age,
            grade,
            life,
        })
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_gix_change_1object_App_doubleGrade<'l>(
    mut _env: JNIEnv<'l>,
    _obj: JObject<'l>,
    student_ptr: jlong,
) {
    println!("Get address to double: {}", student_ptr as usize);

    let student = student_ptr as *mut Student;

    unsafe {
        // let mut student = Box::from_raw(student);

        println!("before: {:?}", (*student));

        (*student).grade *= 2;
        println!("after: {:?}", (*student));
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_gix_change_1object_App_turnOff<'l>(
    mut _env: JNIEnv<'l>,
    _obj: JObject<'l>,
    student_ptr: jlong,
) {
    println!("Get address to off: {}", student_ptr);
    let student = student_ptr as *mut Student;

    unsafe {
        println!("before: {:?}", (*student));
        (*student).life = false;
        println!("after: {:?}", (*student));
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_gix_change_1object_App_mainNative<'l>(
    mut env: JNIEnv<'l>,
    obj: JObject<'l>,
) {
    let student = Student::new("Gix", 21, 6, true);

    let address = Box::into_raw(student);

    // let clazz = env.get_object_class(&obj).expect("Cannot get class");

    // let method = env
    //     .get_method_id(clazz, "setAddress", "(J)V")
    //     .expect("Cannot get method setAddress");

    println!("Address created: {}", address as jlong);

    let params = [JValueGen::Long(address as jlong)];

    env.call_method(&obj, "setAddress", "(J)V", &params)
        .expect("Cannot call setAddress");

    println!("main native loop");

    loop {
        unsafe {
            println!("life: {}", (*address).life);
            if !(*address).life {
                break;
            }
        }
    }

    println!("end main loop")
}
