import 'package:algori/user_preferences.dart';
import 'package:flutter/material.dart';

class SettingsPage extends StatefulWidget {
  @override
  _SettingsPageState createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  final UserPreferences _preferences = UserPreferences();
  bool _isDarkTheme = false;
  int _numColumns = 4;

  @override
  void initState() {
    super.initState();
    _loadPreferences();
  }

  Future<void> _loadPreferences() async {
    _isDarkTheme = await _preferences.isDarkTheme;
    _numColumns = await _preferences.columns;
    setState(() {});
  }

  Future<void> _updateThemePreference(bool value) async {
    await _preferences.setDarkTheme(value);
    setState(() {
      _isDarkTheme = value;
    });
  }

  Future<void> _updateColumnsPreference(int value) async {
    await _preferences.setColumns(value);
    setState(() {
      _numColumns = value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: Text("Settings"),
        ),
        body: ListView(padding: const EdgeInsets.all(16.0), children: [
          // Theme switch
          SwitchListTile(
            title: Text(
              'Dark Theme',
              style: TextStyle(fontSize: 20),
            ),
            value: _isDarkTheme,
            onChanged: (value) => _updateThemePreference(value),
          ),
          // Num Columns
          Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [
            Text(
              'Algorithms per row:',
              style: TextStyle(fontSize: 20),
            ),
            Row(
              children: [
                IconButton(
                  icon: Icon(Icons.remove),
                  onPressed: _numColumns > 1
                      ? () => _updateColumnsPreference(_numColumns - 1)
                      : null,
                ),
                Text(
                  '$_numColumns',
                  style: TextStyle(fontSize: 20),
                ),
                IconButton(
                  icon: Icon(Icons.add),
                  onPressed: () => _updateColumnsPreference(_numColumns + 1),
                ),
              ],
            ),
          ]),
        ]));
  }
}
