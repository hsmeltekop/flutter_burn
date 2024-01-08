import 'dart:ffi';
import 'dart:typed_data';

import 'gen_api.dart';
import 'helpers.dart';

void main() async {
  late apiClassImpl native;
  try {
    native = apiClassImpl(
        DynamicLibrary.open("../../native/target/release/libnative.so"));
  } catch (e) {
    native = apiClassImpl(
        DynamicLibrary.open("./native/target/release/libnative.so"));
  }

  await native.initPredictor(
      modelPath: "/home/n1/flutter_projects/mnist_burn/native/guide");

  print("=" * 78);
  Float32List bytes = Float32List.fromList(await imageToRawBytes());
  var r = await native.predict(bytes: bytes);
  print("Prediction: $r");
}
