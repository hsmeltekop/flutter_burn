import 'package:flutter/material.dart';
import 'exports.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        debugShowCheckedModeBanner: false,
        home:
            SafeArea(child: OrientationBuilder(builder: (context, orientation) {
          if (orientation == Orientation.portrait) {
            return const InferencePortrait();
          } else {
            return const InferenceLandscape();
          }
        })));
  }
}
