import 'package:flutter/material.dart';

class SettingsPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Settings Page')),
      body: Center(
        child: Text(
          "Settings",
          style: TextStyle(fontSize: 24),
        ),
      ),
    );
  }
}