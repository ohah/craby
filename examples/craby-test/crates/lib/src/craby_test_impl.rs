use std::path::PathBuf;

use craby::{prelude::*, throw};

use crate::ffi::bridging::*;
use crate::generated::*;

pub struct CrabyTest {
    ctx: Context,
    state: Option<Number>,
}

impl CrabyTest {
    fn get_file_path(&self) -> PathBuf {
        PathBuf::from(self.ctx.data_path.clone()).join("data.txt")
    }
}

#[craby_module]
impl CrabyTestSpec for CrabyTest {
    fn new(ctx: Context) -> Self {
        CrabyTest { ctx, state: None }
    }

    fn numeric_method(&mut self, arg: Number) -> Number {
        arg * 2.0
    }

    fn boolean_method(&mut self, arg: Boolean) -> Boolean {
        !arg
    }

    fn string_method(&mut self, arg: &str) -> String {
        format!("From Rust: {}", arg.to_string())
    }

    fn object_method(&mut self, mut arg: TestObject) -> TestObject {
        arg.foo = format!("From Rust: {}", arg.foo);
        arg.bar = arg.bar * 2.0;
        arg.baz = !arg.baz;
        arg.camel_case = arg.camel_case + 1.0;
        arg.pascal_case = arg.pascal_case + 1.0;
        arg.snake_case = arg.snake_case + 1.0;
        arg
    }

    fn array_method(&mut self, mut arg: Array<Number>) -> Array<Number> {
        arg.extend(vec![1.0, 2.0, 3.0]);
        arg.iter_mut().for_each(|x| *x *= 2.0);
        arg
    }

    fn enum_method(&mut self, arg0: MyEnum, arg1: SwitchState) -> String {
        let arg0 = match arg0 {
            MyEnum::Foo => "Enum Foo!",
            MyEnum::Bar => "Enum Bar!",
            MyEnum::Baz => "Enum Baz!",
            _ => unreachable!(),
        };

        let arg1 = match arg1 {
            SwitchState::Off => "Off",
            SwitchState::On => "On",
            _ => unreachable!(),
        };

        format!("Enum {} / {}", arg0, arg1)
    }

    fn nullable_method(&mut self, arg: Nullable<Number>) -> Nullable<Number> {
        match arg.value_of() {
            Some(val) => {
                if *val < 0.0 {
                    Nullable::<Number>::none()
                } else {
                    let new_val = val * 10.0;
                    arg.value(new_val)
                }
            }
            None => Nullable::<Number>::some(123.0),
        }
    }

    fn promise_method(&mut self, arg: Number) -> Promise<Number> {
        if arg == 0.0 {
            throw!("Zero is not allowed");
        }

        if arg >= 0.0 {
            promise::resolve(arg * 2.0)
        } else {
            promise::reject("Boom!")
        }
    }

    fn set_state(&mut self, arg: Number) -> Void {
        self.state = Some(arg);
    }

    fn get_state(&mut self) -> Number {
        self.state.unwrap_or(0.0)
    }

    fn get_data_path(&mut self) -> String {
        self.ctx.data_path.clone()
    }

    fn write_data(&mut self, value: &str) -> Boolean {
        std::fs::write(self.get_file_path(), value).is_ok()
    }

    fn read_data(&mut self) -> Nullable<String> {
        match std::fs::read_to_string(self.get_file_path()) {
            Ok(data) => Nullable::<String>::some(data),
            Err(_) => Nullable::<String>::none(),
        }
    }

    fn trigger_signal(&mut self) -> Void {
        self.emit(CrabyTestSignal::OnSignal);
        self.emit(CrabyTestSignal::OnFinished);
    }

    fn trigger_signal_array_number(&mut self) -> Void {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        self.emit_array_number(CrabyTestSignal::OnSignal, &numbers);
    }

    fn trigger_signal_array_string(&mut self) -> Void {
        let strings: Vec<String> = vec!["hello".to_string(), "world".to_string(), "from".to_string(), "rust".to_string()];
        self.emit_array_string(CrabyTestSignal::OnSignal, &strings);
    }

    fn trigger_signal_object(&mut self) -> Void {
        // JSON 문자열을 직접 생성하여 바이트로 변환
        let json_str = r#"{"foo":"test_foo","bar":42.0,"baz":true,"sub":{"a":"test_sub_a","b":100.0,"c":false},"camel_case":1.0,"pascal_case":2.0,"snake_case":3.0}"#;
        let json_bytes = json_str.as_bytes();
        self.emit_object(CrabyTestSignal::OnSignal, json_bytes);
    }

    fn trigger_signal_array_object(&mut self) -> Void {
        // 각 Object를 JSON 문자열로 직렬화하여 Vec<String>으로 전달
        let json_strs: Vec<String> = vec![
            r#"{"foo":"test_foo_1","bar":1.0,"baz":true}"#.to_string(),
            r#"{"foo":"test_foo_2","bar":2.0,"baz":false}"#.to_string(),
            r#"{"foo":"test_foo_3","bar":3.0,"baz":true}"#.to_string(),
        ];
        self.emit_array_object(CrabyTestSignal::OnSignal, &json_strs);
    }

    fn camel_method(&mut self) -> Void {
        // noop
    }

    fn pascal_method(&mut self) -> Void {
        // noop
    }

    fn snake_method(&mut self) -> Void {
        // noop
    }
}
