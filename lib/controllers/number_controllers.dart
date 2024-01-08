import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final positionProvider = StateProvider<List<List<Offset>>>((ref) => []);

final predictionProvider = StateProvider<int?>((ref) => null);

final imageProvider = StateProvider<Uint8List?>((ref) => null);
