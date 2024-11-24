import 'package:shared_preferences/shared_preferences.dart';

class UserPreferences {
  static const _themeKey = "theme";
  static const _columnsKey = "columns";

  Future<bool> get isDarkTheme async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getBool(_themeKey) ?? false;
  }

  Future<int> get columns async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getInt(_columnsKey) ?? 4;
  }

  Future<void> setDarkTheme(bool value) async {
    final prefs = await SharedPreferences.getInstance();
    prefs.setBool(_themeKey, value);
  }

  Future<void> setColumns(int value) async {
    final prefs = await SharedPreferences.getInstance();
    prefs.setInt(_columnsKey, value);
  }
}
