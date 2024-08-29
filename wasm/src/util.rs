pub mod distribution;
pub mod modulus;
pub mod rng;

macro_rules! impl_try_from_js_value {
    ($ty:ident) => {
        impl TryFrom<wasm_bindgen::JsValue> for $ty {
            type Error = String;

            fn try_from(value: wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
                serde_wasm_bindgen::from_value(value).map_err(|err| format!("{}", err))
            }
        }
    };
}

macro_rules! impl_from_into_wasm_abi {
    ($ty:ident) => {
        $crate::util::impl_constructor_from_object!($ty);

        impl wasm_bindgen::describe::WasmDescribe for $ty {
            fn describe() {
                wasm_bindgen::JsValue::describe()
            }
        }

        impl wasm_bindgen::convert::IntoWasmAbi for $ty {
            type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;

            fn into_abi(self) -> Self::Abi {
                serde_wasm_bindgen::to_value(&self).unwrap().into_abi()
            }
        }

        impl wasm_bindgen::convert::FromWasmAbi for $ty {
            type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

            unsafe fn from_abi(js: Self::Abi) -> Self {
                wasm_bindgen::JsValue::from_abi(js).try_into().unwrap()
            }
        }
    };
}

macro_rules! impl_constructor_from_object {
    ($ty:ident) => {
        $crate::util::impl_try_from_js_value!($ty);

        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl $ty {
            #[wasm_bindgen(constructor)]
            pub fn new(value: wasm_bindgen::JsValue) -> Result<$ty, String> {
                $ty::try_from(value).map_err(|err| err.to_string())
            }
        }
    };
}

macro_rules! impl_from_to_bytes {
    ($ty:ident) => {
        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl $ty {
            #[wasm_bindgen(js_name = fromBytes)]
            pub fn from_bytes(bytes: &[u8]) -> Result<$ty, String> {
                bincode::deserialize(bytes).map_err(|err| err.to_string())
            }

            #[wasm_bindgen(js_name = toBytes)]
            pub fn to_bytes(&self) -> wasm_bindgen::JsValue {
                bincode::serialize(self).unwrap().into()
            }
        }
    };
}

pub(crate) use {
    impl_constructor_from_object, impl_from_into_wasm_abi, impl_from_to_bytes,
    impl_try_from_js_value,
};
